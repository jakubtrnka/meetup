use std::error::Error;
use tokio_util::bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

struct Frame {
    header: u32, // trivial header - just the payload size
    payload: Vec<u8>,
}

struct AdvancedCodec {/* ... */}

impl Encoder<Frame> for AdvancedCodec {
    type Error = std::io::Error;

    fn encode(&mut self, _item: Frame, _dst: &mut BytesMut) -> Result<(), Self::Error> {
        todo!()
    }
}

impl Decoder for AdvancedCodec {
    type Item = Frame;
    type Error = std::io::Error;
    fn decode(&mut self, _src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
