use rsct::transmit::transmit;
use tokio::net::UdpSocket;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Create a UDP socket bound to a specific address and port
    let socket = UdpSocket::bind("0.0.0.0:5253").await.unwrap();

    // Set a timeout for receiving data (non-blocking with timeout)

	let destination: SocketAddr = "127.0.0.1:5254".parse().unwrap();

    let string = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Cursus vitae congue mauris rhoncus aenean vel elit scelerisque. Enim diam vulputate ut pharetra sit amet aliquam id diam. Sit amet mauris commodo quis imperdiet massa tincidunt nunc. Ac auctor augue mauris augue neque. A scelerisque purus semper eget duis at tellus at. Lectus mauris ultrices eros in cursus turpis. Pretium fusce id velit ut. Adipiscing vitae proin sagittis nisl rhoncus mattis rhoncus urna neque. Sem fringilla ut morbi tincidunt augue interdum. At erat pellentesque adipiscing commodo elit. Id consectetur purus ut faucibus pulvinar.

    Scelerisque in dictum non consectetur a erat nam at. Amet venenatis urna cursus eget nunc scelerisque. Vestibulum morbi blandit cursus risus at ultrices. Turpis tincidunt id aliquet risus. Sit amet volutpat consequat mauris nunc congue nisi vitae suscipit. Ullamcorper eget nulla facilisi etiam dignissim diam quis enim lobortis. Lectus vestibulum mattis ullamcorper velit sed ullamcorper morbi tincidunt. Pulvinar elementum integer enim neque volutpat ac. Nam aliquam sem et tortor consequat id porta nibh venenatis. Arcu cursus euismod quis viverra. Non enim praesent elementum facilisis leo. In arcu cursus euismod quis viverra nibh. Turpis cursus in hac habitasse platea dictumst quisque. Mauris commodo quis imperdiet massa tincidunt. At volutpat diam ut venenatis tellus in. Augue lacus viverra vitae congue eu consequat. Aliquam sem et tortor consequat id porta nibh venenatis cras. Aliquet nibh praesent tristique magna. Ornare lectus sit amet est placerat in egestas erat. Morbi tristique senectus et netus et malesuada fames ac turpis.
    
    Fames ac turpis egestas maecenas pharetra convallis posuere morbi. Sed egestas egestas fringilla phasellus faucibus scelerisque. Dolor morbi non arcu risus. Gravida in fermentum et sollicitudin ac orci phasellus egestas tellus. Arcu felis bibendum ut tristique et egestas. Sit amet mauris commodo quis imperdiet massa tincidunt. Pretium viverra suspendisse potenti nullam ac tortor vitae purus faucibus. Pharetra magna ac placerat vestibulum lectus mauris ultrices eros in. Quam vulputate dignissim suspendisse in est ante in nibh mauris. Ut consequat semper viverra nam libero justo laoreet sit amet. Sollicitudin tempor id eu nisl nunc. Eu tincidunt tortor aliquam nulla facilisi cras fermentum.
    
    Egestas egestas fringilla phasellus faucibus scelerisque. Pretium quam vulputate dignissim suspendisse in est ante. Quam adipiscing vitae proin sagittis nisl rhoncus mattis. Turpis egestas sed tempus urna et pharetra pharetra massa. Quis hendrerit dolor magna eget est lorem ipsum. Mi proin sed libero enim sed. Ipsum a arcu cursus vitae congue mauris. Morbi quis commodo odio aenean. Sagittis vitae et leo duis ut diam quam nulla porttitor. Ut placerat orci nulla pellentesque dignissim enim sit amet. Auctor elit sed vulputate mi sit amet mauris commodo quis. Et malesuada fames ac turpis egestas sed. Ipsum a arcu cursus vitae congue mauris rhoncus.
    
    Sed lectus vestibulum mattis ullamcorper velit sed ullamcorper morbi tincidunt. Aliquet lectus proin nibh nisl condimentum id venenatis a. Eu volutpat odio facilisis mauris sit amet massa vitae. Vel turpis nunc eget lorem dolor sed viverra ipsum nunc. Dignissim convallis aenean et tortor at. At volutpat diam ut venenatis. Platea dictumst quisque sagittis purus sit amet volutpat. Enim blandit volutpat maecenas volutpat blandit. Lorem ipsum dolor sit amet consectetur adipiscing elit duis tristique. Quisque id diam vel quam elementum pulvinar etiam non quam. Sed libero enim sed faucibus turpis in eu mi bibendum. Feugiat scelerisque varius morbi enim nunc faucibus. Eget nunc scelerisque viverra mauris. In nisl nisi scelerisque eu.";

    transmit(string as *const u8, string.len(), &socket, &destination).await;


	Ok(())
}