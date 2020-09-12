pub mod packets;

use packets::types::*;
use packets::clientbound;
use packets::serverbound;

use std::fmt::Debug;
use std::net::TcpStream;
use protocol::{Parcel, Settings};
use std::io::{Write, Read};
use protocol::hint::{Hints};
use std::io;
use flate2::read::ZlibDecoder;
use std::sync::{Arc, Mutex};
use std::ops::Deref;

pub struct Connection {
    stream: Arc<Mutex<TcpStream>>,
    settings: Settings,
    compression_threshold: Arc<Mutex<i32>>,
    log: bool,
}

impl Connection {
    pub fn new(address: &str, log: bool) -> (InMcStream, OutMcStream) {
        let stream = TcpStream::connect(address).unwrap();
        let settings = protocol::Settings {
            byte_order: protocol::ByteOrder::BigEndian,
            ..Default::default()
        };
        let in_conn = Connection { stream: Arc::new(Mutex::new(stream)), compression_threshold: Arc::new(Mutex::new(-1)), settings, log };
        let out_conn = in_conn.clone();
        (InMcStream { conn: in_conn }, OutMcStream { conn: out_conn})
    }

    fn decompress<R: Read>(mut reader: R, length: usize) -> io::Cursor<Vec<u8>> {
        let mut new = Vec::with_capacity(length as usize);
        {
            let mut reader = ZlibDecoder::new(&mut reader);
            reader.read_to_end(&mut new).unwrap();
        }

        io::Cursor::new(new)
    }

    fn update_compression_threshold(&mut self, new_threshold: i32) {
        *self.compression_threshold.lock().unwrap() = new_threshold;
    }

    fn reader_from_packet(&mut self, packet_length: usize) -> io::Cursor<Vec<u8>> {
        let mut ibuf = vec![0; packet_length as usize];
        self.stream.lock().unwrap().read_exact(&mut ibuf).unwrap();
        io::Cursor::new(ibuf)
    }

    fn read_data<T: StateData>(&mut self) -> T {
        let VarInt { val: packet_length } = VarInt::read_field(&mut self.stream.lock().unwrap().deref(), &self.settings, &mut Hints::default()).unwrap();

        let mut reader = self.reader_from_packet(packet_length as usize);


        if *self.compression_threshold.lock().unwrap() >= 0 {
            let VarInt { val: data_length } = VarInt::read_field(&mut reader, &self.settings, &mut Hints::default()).unwrap();

            if data_length > 0 {
                reader = Connection::decompress(reader, data_length as usize);
            }

        }
        let data = T::read_field(&mut reader, &self.settings, &mut Hints::default()).unwrap();

        if self.log {
            println!("Reading: {:?}", data);
        }

        data
    }


    fn write_packet<P: Parcel>(&mut self, packet: P) {
        self.stream.lock().unwrap().write_all(&packet.raw_bytes(&self.settings).unwrap()).unwrap();
    }

    fn write_data<T: StateData>(&mut self, data: T) {
        if self.log {
            println!("Writing: {:?}", data);
        }

        if *self.compression_threshold.lock().unwrap() < 0 {
            let length = data.length(&self);

            self.write_packet(length);
            self.write_packet(data);

        } else {
            // Handle compressed data

            // length of the VarInt data_length is 1 byte, see below
            let data_length_length = 1;

            let VarInt { val: packet_length } = data.length(&self);

            let packet_length = VarInt { val: data_length_length + packet_length };
            let data_length = VarInt { val: 0 };

            self.write_packet(packet_length);
            self.write_packet(data_length);
            self.write_packet(data);
        }

    }
}

impl Clone for Connection {
    fn clone(&self) -> Self {
        Connection { compression_threshold: self.compression_threshold.clone(), stream: self.stream.clone(), settings: self.settings.clone(), log: self.log }
    }
}

pub struct InMcStream {
    conn: Connection
}

impl InMcStream {
    pub fn read_data<T: StateData>(&mut self) -> T {
        self.conn.read_data()
    }

    pub fn update_compression_threshold(&mut self, new_threshold: i32) {
        self.conn.update_compression_threshold(new_threshold)
    }
}

pub struct OutMcStream {
    conn: Connection
}

impl OutMcStream {
    pub fn write_data<T: StateData>(&mut self, data: T) {
        self.conn.write_data(data)
    }

    pub fn update_compression_threshold(&mut self, new_threshold: i32) {
        self.conn.update_compression_threshold(new_threshold)
    }
}

impl Clone for OutMcStream {
    fn clone(&self) -> Self {
        OutMcStream { conn: self.conn.clone() }
    }
}

pub trait StateData: Parcel + Debug {
    fn length(&self, conn: &Connection) -> VarInt {
        VarInt { val: self.raw_bytes(&conn.settings).unwrap().len() as i32 }
    }
}

impl StateData for clientbound::play::Data {}
impl StateData for clientbound::login::Data {}
impl StateData for clientbound::status::Data {}
impl StateData for serverbound::login::Data {}
impl StateData for serverbound::status::Data {}
impl StateData for serverbound::play::Data {}
impl StateData for serverbound::handshaking::Data {}