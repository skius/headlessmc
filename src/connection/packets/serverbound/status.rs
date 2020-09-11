use crate::connection::packets::types::*;

#[derive(Protocol, Clone, Debug, PartialEq)]
#[protocol(discriminant = "integer")]
#[repr(u8)] // Force discriminators to be 8-bit.
pub enum Data {
    #[protocol(discriminator(0x00))]
    Request(Request),
    #[protocol(discriminator(0x01))]
    Ping(Ping),
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Request {
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Ping {
    pub payload: u64,
}