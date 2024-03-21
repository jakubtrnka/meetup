use futures::{SinkExt, StreamExt};
use std::error::Error;
use tokio_util::codec::{Framed, LinesCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stream = tokio::net::TcpStream::connect("eu.stratum.braiins.com:3333").await?;
    let mut fstream = Framed::new(stream, LinesCodec::new());

    fstream
        .send(r#"{"id":1,"method":"mining.subscribe","params":[]}"#.to_string())
        .await?;

    while let Some(decoded_result) = fstream.next().await {
        let obtained_string = decoded_result?;
        println!("{}", obtained_string);
    }
    Ok(())
}
