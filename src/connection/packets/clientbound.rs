use protocol::{Parcel, Settings};
use std::io::{Write, Read};
use protocol::hint::{Hints};

mod status;
mod login;
mod play;

use super::types::*;

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct JsonResponse {
    pub json: McString,
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
pub struct KeepAliveCb {
    pub keep_alive_id: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum IgnoreOrKeepAlive {
    Ignore(Ignore),
    KeepAlive(KeepAliveCb)
}

// impl Parcel for IgnoreOrKeepAlive {
//     const TYPE_NAME: &'static str = "IgnoreOrKeepAlive";
//
//     fn read_field(read: &mut dyn Read, settings: &Settings, hints: &mut Hints) -> Result<Self, protocol::Error> {
//         unimplemented!()
//     }
//
//     fn write_field(&self, write: &mut dyn Write, settings: &Settings, hints: &mut Hints) -> Result<(), protocol::Error> {
//         Ok(())
//     }
// }

#[derive(Clone, Debug, PartialEq)]
pub struct Ignore;

// impl CustomParcel for Ignore {
//     fn give_length(&mut self, length: VarInt) {
//     }
// }

// impl Parcel for Ignore {
//     const TYPE_NAME: &'static str = "IGNORE";
//
//     fn read_field(read: &mut dyn Read, settings: &Settings, hints: &mut Hints) -> Result<Self, protocol::Error> {
//         // read hints.knownlenghts 0 bytes
//         let length = hints.known_field_lengths.get(&0).unwrap().length;
//         let mut ibuf = vec![0; length];
//         read.read_exact(&mut ibuf)?;
//         Ok(Ignore)
//     }
//
//     fn write_field(&self, write: &mut dyn Write, settings: &Settings, hints: &mut Hints) -> Result<(), protocol::Error> {
//         Ok(())
//     }
// }

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct ExplPacket<T: Parcel> {
    pub length: VarInt,
    pub packet_id: u8,
    pub data: T,
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct ExplCompressedPacket<T: Parcel> {
    pub packet_id: VarInt,
    pub data: T,
}

// #[derive(Clone, Debug, PartialEq)]
// pub struct CompressedIgnoreOrKeepAlive {
//     pub packet_id: VarInt,
//     pub ignore_or_keep_alive: IgnoreOrKeepAlive,
// }

impl Parcel for IgnoreOrKeepAlive {
    const TYPE_NAME: &'static str = "IgnoreOrKeepAlive";

    fn read_field(mut read: &mut dyn Read, settings: &Settings, mut hints: &mut Hints) -> Result<Self, protocol::Error> {
        // let packet_length = VarInt { val: hints.known_field_lengths.get(&0).unwrap() as i32 };
        let data_length = hints.known_field_lengths.get(&0).unwrap().length;
        let packet_id = VarInt::read_field(&mut read, &settings, &mut hints).unwrap();

        // remaining data_length after packet_id
        let data_length = data_length - packet_id.length() as usize;

        // hints.known_field_lengths.insert(0, data_length as FieldLength).unwrap();

        if packet_id.val == 0x21 {
            // KeepAlive
            let keep_alive = KeepAliveCb::read_field(&mut read, &settings, &mut hints)?;
            Ok(IgnoreOrKeepAlive::KeepAlive(keep_alive))
        } else {
            // Ignore
            // let mut ibuf = vec![0; data_length];
            // read.read_exact(&mut ibuf)?;
            Ok(IgnoreOrKeepAlive::Ignore(Ignore))
        }

    }

    fn write_field(&self, _: &mut dyn Write, _: &Settings, _: &mut Hints) -> Result<(), protocol::Error> {
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PacketCb<T: Parcel> {
    Compressed(ExplCompressedPacket<T>),
    Uncompressed(ExplPacket<T>),
}

pub enum PacketInnerCb {
    LoginSuccess,
    SetCompression
}

