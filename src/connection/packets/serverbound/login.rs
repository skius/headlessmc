use crate::connection::packets::types::*;


#[derive(Protocol, Clone, Debug, PartialEq)]
#[protocol(discriminant = "integer")]
#[repr(u8)] // Force discriminators to be 8-bit.
pub enum Data {
    #[protocol(discriminator(0x00))]
    LoginStart(LoginStart),
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct LoginStart {
    pub name: McString,
}
