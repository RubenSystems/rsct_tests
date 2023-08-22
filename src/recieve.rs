use tokio::net::UdpSocket;
use std::io;

#[repr(C)]
pub struct Packet { 
    index: u8, 
    client_tied_id: u8, 
    total: u8 
}

#[tokio::main]
async fn main() -> io::Result<()> {
	let server = rsct::server::Server::new("0.0.0.0", "5254").await;
	server.start_listener().await;

	Ok(())

}