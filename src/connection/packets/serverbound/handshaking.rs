use crate::connection::packets::types::*;

#[derive(Protocol, Clone, Debug, PartialEq)]
#[protocol(discriminant = "integer")]
#[repr(u8)] // Force discriminators to be 8-bit.
pub enum Data {
    #[protocol(discriminator(0x00))]
    Handshake(Handshake),
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub address: McString,
    pub port: u16,
    pub next_state: VarInt,
}

impl Handshake {
    pub fn new(addr: String, port: u16, next_state: VarInt) -> Handshake {
        Handshake {
            protocol_version: VarInt { val: 578 },
            address: McString::new(&addr),
            port,
            next_state
        }
    }
    //
    // pub fn length(&self) -> i32 {
    //     self.protocol_version.length()
    //         + self.address.length()
    //         + 2 // port Unsigned Short
    //         + self.next_state.length()
    // }
}