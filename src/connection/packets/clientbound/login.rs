use crate::connection::packets::types::*;


#[derive(Protocol, Clone, Debug, PartialEq)]
#[protocol(discriminant = "integer")]
#[repr(u8)] // Force discriminators to be 8-bit.
pub enum Data {
    #[protocol(discriminator(0x00))]
    Disconnect(Disconnect),
    EncryptionRequest,
    LoginSuccess(LoginSuccess),
    SetCompression(SetCompression),
    LoginPluginRequest,
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

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Disconnect {
    pub reason: Chat,
}