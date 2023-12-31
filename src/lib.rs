#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown")]
    Unknown,

    #[error("conversion error: {0}")]
    ConversionError(#[from] std::num::TryFromIntError),
}

type Result<T, E = Error> = std::result::Result<T, E>;

pub fn get() -> Result<usize> {
    get_mem()
}

#[cfg(target_os = "linux")]
fn get_mem() -> Result<usize> {
    use cgroupfs::CgroupReader;
    use std::path::Path;

    let cg = CgroupReader::new(Path::new(cgroupfs::DEFAULT_CG_ROOT).to_path_buf()).unwrap();
    let mem_high = match cg.read_memory_max() {
        Ok(mem) => mem.try_into()?,
        Err(_) => get_physical()?,
    };

    Ok(mem_high)
}

#[cfg(target_os = "macos")]
fn get_mem() -> Result<usize> {
    get_physical()
}

#[cfg(not(any(target_os = "linux", target_os = "macos",)))]
fn get_mem() -> u64 {
    Err(Error::Unknown)
}

pub fn get_physical() -> Result<usize> {
    let mem_info = sys_info::mem_info().unwrap();
    // meminfo is in kilobytes
    let total: usize = mem_info.total.try_into()?;
    Ok(1024 * total)
}

#[cfg(test)]
mod tests {
    fn env_var(name: &'static str) -> Option<usize> {
        ::std::env::var(name).ok().map(|val| val.parse().unwrap())
    }

    #[test]
    fn test_get() {
        let mem = super::get().expect("mem");
        assert!(mem > 0);
        if let Some(n) = env_var("MEMAX_TEST_GET") {
            println!("{mem} ~= {n}");
            assert!(mem > n - 1024);
            assert!(mem < n + 1024);
        }
    }

    #[test]
    fn test_get_physical() {
        let mem = super::get_physical().unwrap();
        assert!(mem > 0);
        if let Some(n) = env_var("MEMAX_TEST_GET_PHY") {
            println!("{mem} ~= {n}");
            assert!(mem > n - 1024);
            assert!(mem < n + 1024);
        }
    }
}
