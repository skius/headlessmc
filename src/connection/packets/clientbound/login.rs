use crate::connection::packets::types::*;


#[derive(Protocol, Clone, Debug, PartialEq)]
#[protocol(discriminant = "integer")]
#[repr(u8)] // Force discriminators to be 8-bit.
pub enum Data {
    #[protocol(discriminator(0x02))]
    LoginSuccess(LoginSuccess),
    #[protocol(discriminator(0x03))]
    SetCompression(SetCompression),
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct LoginSuccess {
    pub uuid: McString,
    pub username: McString,
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct SetCompression {
    pub threshold: VarInt,
}