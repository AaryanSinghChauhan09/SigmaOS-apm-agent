//! SigmaOS-apm-agent - SigmaOS Component

use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    log::info!("Starting SigmaOS-apm-agent");
    println!("SigmaOS SigmaOS-apm-agent v0.1.0");
    Ok(())
}
