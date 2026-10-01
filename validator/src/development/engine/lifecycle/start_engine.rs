// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::development::{
    config::DevelopmentValidatorConfig,
    engine::{
        configuration::configure_engine_home,
        home::acquire_engine_lease,
        types::OwnedEngine,
        verification::{
            validate_engine_image_binding, validate_engine_namespace, verify_engine_binary,
        },
    },
};
use anyhow::{Result, ensure};
use std::{
    fs::OpenOptions,
    path::Path,
    process::{Command, Stdio},
};

pub(crate) fn start_engine(
    config: &DevelopmentValidatorConfig,
    home: &Path,
    application_socket: &Path,
    signer_socket: &Path,
    expected_sha256: [u8; 32],
) -> Result<OwnedEngine> {
    validate_engine_namespace(config, home)?;
    let lease = acquire_engine_lease(&config.data)?;
    let binary = verify_engine_binary(config, expected_sha256)?;
    configure_engine_home(config, home, application_socket, signer_socket, &binary)?;
    let prefix = crate::development::engine::home::engine_artifact_prefix("run")?;
    let output_log = config.data.join(format!("{prefix}.stdout.log"));
    let error_log = config.data.join(format!("{prefix}.stderr.log"));
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let output = options.open(&output_log)?;
    let error = options.open(&error_log)?;
    let child = Command::new(&binary.path)
        .args(["start", "--home"])
        .arg(home)
        .args(["--log_level", "error"])
        .stdin(Stdio::null())
        .stdout(Stdio::from(output))
        .stderr(Stdio::from(error))
        .spawn()?;
    let mut engine = OwnedEngine {
        child: Some(child),
        image: binary.image,
        process: None,
        lease: Some(lease),
        signer_socket: signer_socket.to_owned(),
        data: config.data.clone(),
        output_log,
        error_log,
    };
    let child = engine
        .child
        .as_mut()
        .ok_or_else(|| anyhow::anyhow!("spawned child missing"))?;
    ensure!(
        child.try_wait()?.is_none(),
        "owned engine exited during launch; private diagnostics preserved"
    );
    let pid = rustix::process::Pid::from_raw(i32::try_from(child.id())?)
        .ok_or_else(|| anyhow::anyhow!("invalid spawned engine PID"))?;
    engine.process = Some(rustix::process::pidfd_open(
        pid,
        rustix::process::PidfdFlags::empty(),
    )?);
    validate_engine_image_binding(&engine.image, child.id())?;
    ensure!(
        child.try_wait()?.is_none(),
        "owned engine exited during image verification"
    );
    Ok(engine)
}
