use std::sync::{Arc, RwLock};

use anyhow::{Result, bail};
use bimap::BiHashMap;
use lazy_static::lazy_static;

use crate::{platform::init_shadow_file, utils::ShadowError};

lazy_static! {
    static ref SHADOWS: Arc<RwLock<BiHashMap<String, i32>>> =
        Arc::new(RwLock::new(BiHashMap::new()));
}

pub fn shadow_file<S: Into<String>, V: Into<Vec<u8>>>(path: S, contents: V) -> Result<i32> {
    // Should only be called from one thread
    let path = path.into();
    let filedes = init_shadow_file(&path, contents.into())?;
    match SHADOWS.write() {
        Ok(mut map) => {
            map.insert(path, filedes);
        }
        Err(_) => bail!(ShadowError::RwLockPoisoned),
    };
    Ok(filedes)
}
