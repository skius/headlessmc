extern crate protocol;
use std::net::TcpStream;
use protocol::{Parcel, Settings};
use std::io::{Write, Read};
use protocol::hint::Hints;

extern crate libflate;

use std::io;
use libflate::zlib::Decoder;
use libflate::zlib::Encoder;

use std::io::prelude::*;
use flate2::read::ZlibDecoder;

use super::types::*;

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

    pub fn length(&self) -> i32 {
        self.protocol_version.length()
            + self.address.length()
            + 2 // port Unsigned Short
            + self.next_state.length()
    }
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct LoginStart {
    pub name: McString
}

impl LoginStart {
    pub fn length(&self) -> i32 {
        self.name.length()
    }
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct KeepAlive {
    pub keep_alive_id: i64,
}


#[derive(Protocol, Clone, Debug, PartialEq)]
#[protocol(discriminant = "integer")]
#[repr(u8)]
pub enum PacketKind {
    #[protocol(discriminator(0x00))]
    Handshake(Handshake),
    #[protocol(discriminator(0x00))]
    Request(Request),
    #[protocol(discriminator(0x00))]
    LoginStart(LoginStart),
    #[protocol(discriminator(0x0f))]
    KeepAlive(KeepAlive),
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Request;

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct PacketSb {
    pub length: VarInt,
    pub data: PacketKind
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct CompressedPacketSb {
    pub packet_length: VarInt,
    pub data_length: VarInt,
    pub data: PacketKind // Uncompressed because fuck this.
}

impl PacketSb {
    pub fn update_length(&mut self, length: i32) {
        self.length = VarInt { val: length + 1 }; // + 1 because of packet id
    }

    pub fn new(p: PacketKind) -> PacketSb {
        PacketSb { length: VarInt { val: 0 }, data: p }
    }
}

impl CompressedPacketSb {
    pub fn update_length(&mut self, length: i32) {
        self.packet_length = VarInt { val: length + 2 }; // + 2 because of packet id and data_length
    }

    pub fn new(p: PacketKind) -> CompressedPacketSb {
        CompressedPacketSb { data_length: VarInt { val: 0 }, packet_length: VarInt { val: 0 }, data: p }
    }
}