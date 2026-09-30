use crate::state::{StateReader, StateRepository};
use std::sync::Arc;

pub fn state_reader(store: &StateRepository) -> StateReader {
    StateReader {
        database: Arc::clone(&store.database),
        identity: store.identity.clone(),
        genesis: Arc::clone(&store.genesis),
        budget: store.budget,
        fence: Arc::clone(&store.fence),
        snapshots: Arc::clone(&store.snapshots),
        publication: Arc::clone(&store.publication),
    }
}
