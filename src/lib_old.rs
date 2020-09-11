// extern crate protocol;
// #[macro_use] extern crate protocol_derive;
// use std::net::TcpStream;
// use protocol::{Parcel, Settings};
// use std::io::{Write, Read};
// use protocol::hint::Hints;
//
// #[derive(Clone, Debug, PartialEq)]
// pub struct McString {
//     length: VarInt,
//     str: String,
// }
//
// impl McString {
//     pub fn new(str: String) -> McString {
//         McString { length: VarInt { val: str.len() as i32 } , str}
//     }
//
//     pub fn length(&self) -> i32 {
//         self.length.length() + match self.length {
//             VarInt { val: len } => len,
//         }
//     }
// }
//
// impl Parcel for McString {
//     const TYPE_NAME: &'static str = "String";
//
//     fn read_field(read: &mut dyn Read, settings: &Settings, hints: &mut Hints) -> Result<Self, protocol::Error> {
//         let VarInt { val: length } = VarInt::read_field(read, settings, hints).unwrap();
//         let mut final_string = Vec::<u8>::with_capacity(length as usize);
//         let mut buf = [0u8];
//
//         for _ in 0..length {
//             read.read_exact(&mut buf).unwrap();
//             final_string.push(buf[0]);
//         }
//
//         // let final_string = &final_string[..];
//         Ok(McString::new(String::from_utf8(final_string).unwrap()))
//     }
//
//     fn write_field(&self, write: &mut dyn Write, settings: &Settings, hints: &mut Hints) -> Result<(), protocol::Error> {
//         self.length.write_field(write, settings, hints).unwrap();
//         write.write(self.str.as_bytes()).unwrap();
//
//         Ok(())
//     }
// }
//
// #[derive(Clone, Debug, PartialEq)]
// pub struct VarInt {
//     val: i32,
// }
//
// impl VarInt {
//     fn length(&self) -> i32 {
//         self.raw_bytes(&protocol::Settings::default()).unwrap().len() as i32
//     }
// }
//
// impl Parcel for VarInt {
//     const TYPE_NAME: &'static str = "VarInt";
//
//     fn read_field(read: &mut dyn Read, settings: &Settings, hints: &mut Hints) -> Result<Self, protocol::Error> {
//         let mut num_read = 0;
//         let mut result: i32 = 0;
//         let mut byte = [0u8;1];
//         loop {
//             read.read_exact(&mut byte).unwrap();
//
//             let val = byte[0] & 0b01111111;
//             result |= (val << (7 * num_read)) as i32;
//
//             num_read += 1;
//             if num_read > 5 {
//                 panic!("Incorrect VarInt");
//             }
//
//             if byte[0] & 0b10000000 == 0 {
//                 break;
//             }
//         }
//
//         Ok(VarInt { val: result })
//     }
//
//     fn write_field(&self, write: &mut dyn Write, settings: &Settings, hints: &mut Hints) -> Result<(), protocol::Error> {
//         let mut buffer = [0; 5]; // VarInts are never longer than 5 bytes
//         let mut counter = 0;
//         let mut value = self.val;
//
//         loop {
//             let mut temp = (value & 0b01111111) as u8;
//
//             value >>= 7;
//             if value != 0 {
//                 temp |= 0b10000000;
//             }
//
//             buffer[counter] = temp;
//
//             counter += 1;
//
//             if value == 0 {
//                 break;
//             }
//         }
//
//         write.write_all(&buffer[0..counter])?;
//
//         Ok(())
//     }
// }
//
// #[derive(Protocol, Clone, Debug, PartialEq)]
// pub struct Handshake {
//     packet_id: VarInt,
//     protocol_version: VarInt,
//     address: McString,
//     port: u16,
//     next_state: VarInt,
// }
//
// impl Handshake {
//     fn new(addr: String, port: u16, next_state: VarInt) -> Handshake {
//         Handshake { packet_id: VarInt { val: 0 },
//             protocol_version: VarInt { val: 578 },
//             address: McString::new(addr),
//             port,
//             next_state
//         }
//     }
// }
//
// pub trait PacketKind {
//     // fn packet_id(&self) -> VarInt;
//
//     fn length(&self) -> i32;
// }
//
// impl PacketKind for Handshake {
//     // fn packet_id(&self) -> VarInt {
//     //     VarInt { val: 0 }
//     // }
//
//     fn length(&self) -> i32 {
//         self.packet_id.length()
//         + self.protocol_version.length()
//         + self.address.length()
//         + 2 // port Unsigned Short
//         + self.next_state.length()
//     }
// }
//
// #[derive(Protocol, Clone, Debug, PartialEq)]
// pub struct Packet<T: PacketKind> {
//     length: VarInt,
//     data: T
// }
//
// impl<T: PacketKind> Packet<T> {
//     fn calc_length(&mut self) {
//         self.length = VarInt { val: self.data.length() };
//     }
// }
//
//
//
// extern crate itertools;
// use itertools::Itertools;
//
// pub fn run() {
//     let stream = TcpStream::connect("127.0.0.1:25565").unwrap();
//     let settings = protocol::Settings {
//         byte_order: protocol::ByteOrder::BigEndian,
//         ..Default::default()
//     };
//     let mut connection = protocol::wire::stream::Connection::new(stream, protocol::wire::middleware::pipeline::default(), settings.clone());
//
//     let mut first_handshake = Packet { length: VarInt{val:0}, data: Handshake::new("127.0.0.1".to_string(), 25565, VarInt { val: 2 }) };
//     first_handshake.calc_length();
//
//
//     let mut bytes = first_handshake.raw_bytes(&settings).unwrap();
//     println!("Packet is: {:02x}", bytes.iter().format(" "));
//
//
//     // check_bytes(first_handshake.data.protocol_version);
//
//
//     // let mut reader = &bytes[..];
//     // let reader = &mut reader;
//     // read_handshake_packet(reader);
//
//     connection.send_packet(&first_handshake).unwrap();
//
//     loop {
//         if let Some(response) = connection.receive_packet().unwrap() {
//             println!("{:?}", response);
//             break;
//         }
//     }
// }
//
// fn check_bytes<T: Parcel>(parc: T) {
//     let mut bytes = parc.raw_bytes(&protocol::Settings {
//         byte_order: protocol::ByteOrder::BigEndian,
//         ..Default::default()
//     }).unwrap();
//     println!("Parcel is: {:02x}", bytes.iter().format(" "));
// }
//
// use byteorder::{ByteOrder, LittleEndian, BigEndian};
//
// fn read_handshake_packet<R: Read>(reader: &mut R) {
//     let length = read_varint(reader);
//     let packet_id = read_varint(reader);
//     let protocol_version = read_varint(reader);
//     let addr_size = read_varint(reader);
//
//     let mut buf = [0u8; 1024];
//     reader.read_exact(&mut buf[0..addr_size as usize]).unwrap();
//     let addr = String::from_utf8(buf.to_vec()).unwrap();
//
//     reader.read_exact(&mut buf[0..2]).unwrap();
//     let server_port = BigEndian::read_u16(&buf[0..2]);
//
//     let next_state = read_varint(reader);
//
//
//     println!(r#"Got packet:
//     Length: {}
//     ID: {}
//     Protocol Version: {}
//     Server Address: {}
//     Server Port: {}
//     Next State: {}
//     "#, length, packet_id, protocol_version, addr, server_port, match next_state {
//         1 => "Status",
//         2 => "Login",
//         _ => "Not Found"
//     });
//
// }
//
// fn read_varint<R: Read>(reader: &mut R) -> i32 {
//     let mut num_read = 0;
//     let mut result: i32 = 0;
//     let mut byte = [0u8;1];
//     loop {
//         reader.read_exact(&mut byte).unwrap();
//
//         let val = byte[0] as i32 & 0b01111111 as i32;
//         result |= (val << (7 * num_read));
//
//         num_read += 1;
//         if num_read > 5 {
//             panic!("Incorrect VarInt");
//         }
//
//         if byte[0] & 0b10000000 == 0 {
//             break;
//         }
//     }
//
//     result
// }
