use crate::cache;

pub fn run() -> Result<(), std::io::Error> {
    cache::clean()
}
