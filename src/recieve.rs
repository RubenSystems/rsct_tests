use tokio::net::UdpSocket;
use std::io;
use rsct::recieve::recieve_once;

#[repr(C)]
pub struct Packet { 
    index: u8, 
    client_tied_id: u8, 
    total: u8 
}

#[tokio::main]
async fn main() -> io::Result<()> {
	let socket = UdpSocket::bind("127.0.0.1:5254").await.unwrap();
    
	loop {
		let pack = recieve_once(&socket).await;
		println!("{:?}", String::from_utf8_lossy(&pack.packet.data[..pack.packet_data_size]));
	}

}