use std::{
    ffi::{CString, c_void},
    os::unix::fs::symlink,
};

use anyhow::Result;
use libc::{ftruncate, memfd_create};
pub fn init_shadow_file(path: &String, buf: Vec<u8>) -> Result<i32> {
    // Get a file descriptor for the hashed file name
    let filedes = unsafe { memfd_create(CString::from(c"shadowfile").as_ptr(), 0) };
    // Truncate the file to `buf.len()`
    unsafe { ftruncate(filedes, buf.len() as i64) };
    // Break down the buffer, write it to memory, then clean up
    let (buf_ptr, len, cap) = buf.into_raw_parts();
    unsafe {
        libc::write(filedes, buf_ptr as *const c_void, len);
        Vec::from_raw_parts(buf_ptr, len, cap)
    };

    symlink(format!("/proc/self/fd/{}", filedes), path)?;

    Ok(filedes)
}
