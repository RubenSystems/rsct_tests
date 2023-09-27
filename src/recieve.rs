use rsct::{allocators::basic_allocator::BasicAllocator, server::Server};

#[repr(C)]
pub struct Packet {
    index: u8,
    client_tied_id: u8,
    total: u8,
}

#[tokio::main]
async fn main() {
    let mut server = Server::<BasicAllocator>::new("0.0.0.0", "5254", BasicAllocator).await;
    loop {
        let (client, data) = server.recieve_once().await;

        let string_val = String::from_utf8_lossy(&data).to_string();

        let ip_string = if let Some(client) = &client {
            client.ip_string()
        } else {
            "unknown".to_string()
        };
        let port_string = if let Some(client) = &client {
            client.port_string()
        } else {
            "unknown".to_string()
        };
        println!("{ip_string}:{port_string} - {string_val}");
    }
}
