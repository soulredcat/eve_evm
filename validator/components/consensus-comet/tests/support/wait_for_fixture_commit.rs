use std::{
    sync::{Arc, Mutex, TryLockError},
    thread,
    time::{Duration, Instant},
};

use anyhow::Result;

use super::FixtureState;

/// Require the fixture's synced application marker independently of RPC height.
pub fn wait_for_fixture_commit(
    state: &Arc<Mutex<FixtureState>>,
    height: i64,
    timeout: Duration,
) -> Result<()> {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        match state.try_lock() {
            Ok(state) if state.committed_height >= height => return Ok(()),
            Ok(_) | Err(TryLockError::WouldBlock) => {}
            Err(TryLockError::Poisoned(_)) => anyhow::bail!("fixture state lock poisoned"),
        }
        thread::sleep(Duration::from_millis(5));
    }
    anyhow::bail!("application checkpoint did not reach required height {height}")
}

#[test]
fn observed_height_does_not_replace_synced_application_checkpoint() {
    let state = Arc::new(Mutex::new(FixtureState {
        committed_height: 5,
        ..Default::default()
    }));
    assert!(wait_for_fixture_commit(&state, 6, Duration::from_millis(10)).is_err());
    assert_eq!(state.lock().unwrap().committed_height, 5);
}

#[test]
fn waits_for_delayed_application_checkpoint_publication() {
    let state = Arc::new(Mutex::new(FixtureState::default()));
    let delayed = state.clone();
    let publisher = thread::spawn(move || {
        thread::sleep(Duration::from_millis(15));
        delayed.lock().unwrap().committed_height = 6;
    });
    wait_for_fixture_commit(&state, 6, Duration::from_secs(2)).unwrap();
    publisher.join().unwrap();
    assert_eq!(state.lock().unwrap().committed_height, 6);
}
