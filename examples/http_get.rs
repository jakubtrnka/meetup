use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = tokio::net::TcpStream::connect("noiseprotocol.org:80").await?;

    stream.write_all(b"GET /\n").await?; // don't forget newline
    let mut buffer = [0_u8; 1024];
    while let Ok(count) = stream.read(&mut buffer).await {
        if count == 0 {
            break;
        }
        print!("{}", String::from_utf8_lossy(&buffer[..count]).to_string());
    }
    Ok(())
}
