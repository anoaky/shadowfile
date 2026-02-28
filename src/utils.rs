use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ShadowError {
    #[error("RwLock has been poisoned")]
    RwLockPoisoned,
}
