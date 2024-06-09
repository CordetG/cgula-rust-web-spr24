use crate::*;

pub struct AppState {
    pub store: Store,
}

pub type SharedAppState = Arc<RwLock<AppState>>;

pub type HandlerAppState = State<SharedAppState>;

impl AppState {
    pub fn new(store: Store) -> Self {
        Self { store }
    }
}
