extern crate protocol;
use std::net::TcpStream;
use protocol::{Parcel, Settings};
use std::io::{Write, Read};
use protocol::hint::{Hints, FieldLength, LengthPrefixKind};

extern crate libflate;

use std::io;
use libflate::zlib::Decoder;
use libflate::zlib::Encoder;

use std::io::prelude::*;
use flate2::read::ZlibDecoder;

pub mod packets;

use packets::types::*;
use packets::clientbound;
use packets::serverbound;
use crate::connection::packets::clientbound::PacketCb::{Uncompressed, Compressed};
use crate::connection::packets::clientbound::{IgnoreOrKeepAlive, PacketInnerCb, LoginSuccess, SetCompression};
use std::collections::HashMap;
use std::fmt::Debug;


pub struct Connection {
    pub stream: TcpStream,
    pub settings: Settings,
    pub compression_threshold: i32,
}

impl Connection {
    pub fn decompress<R: Read>(mut reader: R, length: usize) -> io::Cursor<Vec<u8>> {
        let mut new = Vec::with_capacity(length as usize);
        {
            let mut reader = ZlibDecoder::new(&mut reader);
            reader.read_to_end(&mut new).unwrap();
        }

        io::Cursor::new(new)
    }

    pub fn read_packet<T: Parcel>(&mut self) -> clientbound::PacketCb<T> {
        if self.compression_threshold < 0 {
            clientbound::PacketCb::Uncompressed(self.read_uncompressed())
        } else {
            clientbound::PacketCb::Compressed(self.read_compressed())

        }

    }

    pub fn read_ignore_or_keep_alive(&mut self) -> clientbound::IgnoreOrKeepAlive {
        let VarInt { val: packet_length } = VarInt::read_field(&mut self.stream, &self.settings, &mut Hints::default()).unwrap();

        let mut reader = self.reader_from_packet(packet_length as usize);

        let data_length = packet_length;

        if self.compression_threshold >= 0 {
            let VarInt { val: data_length } = VarInt::read_field(&mut reader, &self.settings, &mut Hints::default()).unwrap();

            // This means it's compressed
            if data_length > 0 {
                reader = Connection::decompress(reader, data_length as usize);
            }
        }

        let mut hints = Hints::default();
        hints.known_field_lengths.insert(0, FieldLength { length: data_length as usize, kind: LengthPrefixKind::Bytes } );

        clientbound::IgnoreOrKeepAlive::read_field(&mut reader, &self.settings, &mut hints).unwrap()
    }

    fn read_uncompressed<T: Parcel>(&mut self) -> clientbound::ExplPacket<T> {
        // let packet_length = VarInt::read_field(&mut self.stream, &self.settings, &mut Hints::default()).unwrap();
        clientbound::ExplPacket::read_field(&mut self.stream, &self.settings, &mut Hints::default()).unwrap()
    }

    fn read_compressed<T: Parcel>(&mut self) -> clientbound::ExplCompressedPacket<T> {
        // let packet_length = VarInt::read_field(&mut self.stream, &self.settings, &mut Hints::default()).unwrap();
        let VarInt { val: packet_length } = VarInt::read_field(&mut self.stream, &self.settings, &mut Hints::default()).unwrap();

        let mut reader = self.reader_from_packet(packet_length as usize);

        let VarInt { val: data_length } = VarInt::read_field(&mut reader, &self.settings, &mut Hints::default()).unwrap();

        // This means it's compressed
        if data_length > 0 {
            reader = Connection::decompress(reader, data_length as usize);
        }


        clientbound::ExplCompressedPacket::read_field(&mut reader, &self.settings, &mut Hints::default()).unwrap()
    }

    fn reader_from_packet(&mut self, packet_length: usize) -> io::Cursor<Vec<u8>> {
        let mut ibuf = vec![0; packet_length as usize];
        self.stream.read_exact(&mut ibuf);
        io::Cursor::new(ibuf)
    }

    pub fn consume_packet(&mut self) -> io::Cursor<Vec<u8>> {
        let VarInt { val: packet_length } = VarInt::read_field(&mut self.stream, &self.settings, &mut Hints::default()).unwrap();

        self.reader_from_packet(packet_length as usize)
    }


    // TODO: Add default PacketInnerCb so that read_ignore_or_keep_alive can be replaced by this function (return type: (PacketInnerCb, Box<..>)
    pub fn read_possible_packets(&mut self, possibilities: HashMap<i32, PacketInnerCb>) -> Box<dyn CustomParcel> {
        let VarInt { val: packet_length } = VarInt::read_field(&mut self.stream, &self.settings, &mut Hints::default()).unwrap();

        let mut reader = self.reader_from_packet(packet_length as usize);


        if self.compression_threshold >= 0 {

            let VarInt { val: data_length } = VarInt::read_field(&mut reader, &self.settings, &mut Hints::default()).unwrap();

            if data_length > 0 {
                reader = Connection::decompress(reader, data_length as usize);
            }

        }

        let VarInt { val: packet_id } = VarInt::read_field(&mut reader, &self.settings, &mut Hints::default()).unwrap();

        // println!("packet_id: {}", packet_id);

        match possibilities.get(&packet_id).unwrap() {
            PacketInnerCb::LoginSuccess => {
                Box::new(clientbound::LoginSuccess::read_field(&mut reader, &self.settings, &mut Hints::default()).unwrap())
            },
            PacketInnerCb::SetCompression => {
                Box::new(clientbound::SetCompression::read_field(&mut reader, &self.settings, &mut Hints::default()).unwrap())

            },
        }

    }


    pub fn write_packet<P: Parcel>(&mut self, packet: P) {

        self.stream.write_all(&packet.raw_bytes(&self.settings).unwrap());
    }
}


pub trait CustomParcel: Debug {

}

impl CustomParcel for LoginSuccess {}
impl CustomParcel for SetCompression {}