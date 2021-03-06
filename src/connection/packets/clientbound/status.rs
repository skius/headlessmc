use crate::connection::packets::types::*;

#[derive(Protocol, Clone, Debug, PartialEq)]
#[protocol(discriminant = "integer")]
#[repr(u8)] // Force discriminators to be 8-bit.
pub enum Data {
    #[protocol(discriminator(0x00))]
    JsonResponse(JsonResponse),
    #[protocol(discriminator(0x01))]
    Pong(Pong),
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct JsonResponse {
    pub json: McString,
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Pong {
    pub payload: u64,
}