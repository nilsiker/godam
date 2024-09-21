use anyhow::Result;

use crate::cache;

pub fn run() -> Result<()> {
    cache::clean()
}
