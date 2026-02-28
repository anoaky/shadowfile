#![feature(c_variadic)]
pub(crate) mod fs;
#[cfg_attr(target_os = "macos", path = "platform/macos.rs")]
#[cfg_attr(target_os = "linux", path = "platform/linux.rs")]
pub(crate) mod platform;
pub(crate) mod utils;

pub use fs::shadow_file;

#[cfg(test)]
mod test {
    use std::{fs::File, io::Read};

    use anyhow::Result;

    use super::shadow_file;
    #[test]
    fn simple_test() -> Result<()> {
        let expected = "abc";
        let buf = expected.as_bytes();
        let _filedes = shadow_file("notarealfile", buf)?;
        let mut file = File::open("notarealfile")?;
        let mut result = String::new();
        file.read_to_string(&mut result)?;
        assert_eq!(result, expected);

        Ok(())
    }
}
