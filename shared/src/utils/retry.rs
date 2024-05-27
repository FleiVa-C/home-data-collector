use std::io::Error;
use std::time::Duration;

fn retry_opt<F, T>(f: F, timeout: u32, retries: u32) -> Result<T, Error>
where
    F: Fn() -> Option<T>,
{
    let mut idx = retries;
    while idx < 3 {
        let action = f();
        match action {
            Some(value) => return Ok(value),
            None => {
                std::thread::sleep(Duration::from_secs(timeout.into()));
                idx += 1;
            }
        }
    }
    Err(Error::other("retry failed 3 times"))
}

fn retry_res<F, T>(f: F, timeout: u32, retries: u32) -> Result<T, Error>
where
    F: Fn() -> Result<T, Error>,
{
    let mut idx = retries;
    while idx < 3 {
        let action = f();
        match action {
            Ok(value) => return Ok(value),
            Err(_) => {
                std::thread::sleep(Duration::from_secs(timeout.into()));
                idx += 1;
            }
        }
    }
    Err(Error::other("retry failed 3 times"))
}
