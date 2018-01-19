use protobuf;
use protobuf::core::Message;
use protobuf::error::ProtobufError;
use protobuf::stream::CodedOutputStream;
use std::io::{BufReader, Read, Write};
use mvt::proto;

pub trait Encode {
    fn to_writer(&self, out: &mut Write) -> Result<(), ProtobufError>;
}

pub trait Decode {
    fn from_reader(input: &mut Read) -> Result<Self, ProtobufError> where Self: Sized;
}

impl Encode for proto::Tile {
    fn to_writer(&self, mut out: &mut Write) -> Result<(), ProtobufError> {
        let mut os = CodedOutputStream::new(&mut out);
        let _ = self.write_to(&mut os);
        os.flush()
    }
}

impl Decode for proto::Tile {
    fn from_reader(input: &mut Read) -> Result<Self, ProtobufError> {
        let mut reader = BufReader::new(input);
        protobuf::parse_from_reader(&mut reader)
    }
}
