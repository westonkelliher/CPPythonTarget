extern crate file_lock;
use self::file_lock::{FileLock, FileOptions};
use std::error::Error;


// one use lock
pub struct Locked {
    lock: FileLock,
}
impl Locked {
    pub fn new(s: &str) -> Result<Self, Box<dyn Error>> {
        let options = FileOptions::new().write(true).create(true).append(true);
        Ok(Locked {
            lock: FileLock::lock(s, true /* should block */, options)?,
        })
    }

    pub fn unlock(self) -> Result<(), std::io::Error>{
        self.lock.unlock()
    }

}

