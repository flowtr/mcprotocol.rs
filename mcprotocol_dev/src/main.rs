use mc_registry::registry::LockedContext;
use mc_registry::server_bound::handshaking::Handshake;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Ok(())
}

#[allow(dead_code)]
struct Context;

#[mc_registry_derive::packet_handler]
pub fn test(_packet: Handshake, _context: LockedContext<Context>) {}
