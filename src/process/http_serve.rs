use anyhow::Result;
use std::path::Path;
use tracing::info;

pub fn process_http_serve(dir: &Path, port: u16) -> Result<()> {
    info!("Serving {:?} on port {}", dir, port);
    Ok(())
}
