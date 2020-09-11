use crate::connection::packets::types::*;
use protocol::{Parcel, Settings, hint::Hints};
use std::io::{Write, Read};



#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    KeepAlive(KeepAlive),
    Ignore(Ignore),
}

// Manually handling clientbout play Data, because I ignore all non-keepalive packets
impl Parcel for Data {
    const TYPE_NAME: &'static str = "Data";

    fn read_field(mut read: &mut dyn Read, settings: &Settings, mut hints: &mut Hints) -> Result<Self, protocol::Error> {
        let packet_id = VarInt::read_field(&mut read, &settings, &mut hints).unwrap();
        if packet_id.val == 0x21 {
            // KeepAlive
            let keep_alive = KeepAlive::read_field(&mut read, &settings, &mut hints)?;
            Ok(Data::KeepAlive(keep_alive))
        } else {
            // Ignore
            Ok(Data::Ignore(Ignore))
        }
    }

    fn write_field(&self, write: &mut dyn Write, settings: &Settings, hints: &mut Hints) -> Result<(), protocol::Error> {
        unimplemented!()
    }
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct KeepAlive {
    pub keep_alive_id: i64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ignore;