use std::time::SystemTime;
use sntpc;
use std::error::Error;

fn retry<T>(f: impl Fn() -> Result<T, Box<dyn Error>>) -> Result<T, Box<dyn Error>> {
    let mut err = None;
    for _ in 0..50 {
        match f() {
            Ok(result) => return Ok(result),
            Err(e) => {
                err = Some(e);
            }
        }
    }

    Err(err.unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    let ntp_sec = retry(|| Ok(sntpc::request("pool.ntp.org", 123)?))?.sec as i64;
    let sys_sec = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as i64;

    let skew_min = (ntp_sec - sys_sec) / 60;
    let skew_secs = (ntp_sec - sys_sec) % 60;

    if skew_min != 0 {
        println!("Current clock skew: {} minutes {} seconds", skew_min, skew_secs)
    };

    Ok(())
}
