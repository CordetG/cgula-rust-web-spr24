use crate::auth::JwtKeys;
use crate::*;

pub struct AppState {
    pub store: Store,
    pub jwt_keys: JwtKeys,
    pub reg_key: String,
}

pub type SharedAppState = Arc<RwLock<AppState>>;

pub type HandlerAppState = State<SharedAppState>;

impl AppState {
    pub fn new(store: Store, jwt_keys: JwtKeys, reg_key: String) -> Self {
        Self {
            store,
            jwt_keys,
            reg_key,
        }
    }
}
