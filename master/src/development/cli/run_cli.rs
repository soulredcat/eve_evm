// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    print_development_status::print_development_status,
    types::{Arguments, Command},
};
use crate::development::bootstrap::{
    initialize_development_store::initialize_development_store,
    open_development_store::open_development_store,
};
use anyhow::Result;
use clap::Parser;
use eve_storage::state::{create_state_service, read_state_service, state_reader};

pub fn run_cli() -> Result<()> {
    let arguments = Arguments::parse();
    match arguments.command {
        Command::ServeDev { options, listeners } => {
            crate::development::rpc::serve_development_rpc::serve_development_rpc(
                options, listeners,
            )?;
        }
        Command::SnapshotDev { options, output } => {
            crate::development::snapshots::export_development_snapshot::export_development_snapshot(&options, &output)?;
            println!(
                "Complete local recovery snapshot exported; source finality authentication is not implied."
            );
        }
        Command::RestoreDev { options, source } => {
            let version = crate::development::snapshots::restore_development_snapshot::restore_development_snapshot(&options, &source)?;
            print_development_status(&version)?;
        }
        Command::InitDev { options } => {
            let commit = initialize_development_store(
                &options.root,
                &options.data,
                &options.genesis,
                &options.mode,
                options.acknowledge_unsafe_development,
            )?;
            print_development_status(&commit.target)?;
        }
        Command::InspectDev { options } => {
            let store = open_development_store(
                &options.root,
                &options.data,
                &options.genesis,
                &options.mode,
                options.acknowledge_unsafe_development,
            )?;
            let service = create_state_service(state_reader(&store));
            let view = read_state_service(&service)?;
            print_development_status(&view.commit().target)?;
        }
        Command::ApplyDev { options, block } => {
            let version =
                crate::development::execution::apply_development_block::apply_development_block(
                    &options, &block,
                )?;
            print_development_status(&version)?;
        }
    }
    Ok(())
}
