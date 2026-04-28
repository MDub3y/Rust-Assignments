/*
  Problem 99: Async TCP Echo Server (Simplified)

  Write an async function that starts a mock TCP echo server using
  tokio::net::TcpListener on a given port. It should accept one connection,
  read exactly 5 bytes, and write them back. Return the bytes read.

  Run the tests for this problem with:
    cargo test --test echo_server_test
*/

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn run_echo_server(port: u16) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
  let addr = format!("127.0.0.1:{}", port);
  let listener = TcpListener::bind(&addr).await?;
  
  let (mut stream, _addr) = listener.accept().await?;

  let mut temp_buffer = [0u8; 1024];

    let n = stream.read(&mut temp_buffer).await?;

    let received_data = temp_buffer[..n].to_vec();

    stream.write_all(&received_data).await?;

    Ok(received_data)
}
