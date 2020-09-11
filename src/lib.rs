#[macro_use] extern crate protocol_derive;
use std::net::TcpStream;


pub mod connection;
use connection::packets::types::*;
use connection::packets::serverbound;
use serverbound::login::LoginStart as LoginStartSb;
use serverbound::handshaking::Handshake as HandshakeSb;
use serverbound::status::Request as RequestSb;
use serverbound::status::Ping as PingSb;
use serverbound::play::KeepAlive as KeepAliveSb;
use serverbound::play::ChatMessage as ChatMessageSb;
use connection::packets::clientbound;
use connection::Connection;

use itertools::Itertools;

pub fn better_run() {
    let mut connection = Connection::new("127.0.0.1:25565");

    let handshake = serverbound::handshaking::Data::Handshake(HandshakeSb::new("127.0.0.1".to_string(), 25565, VarInt { val: 1 }));
    connection.write_data(handshake);

    let req = serverbound::status::Data::Request(RequestSb {});
    connection.write_data(req);
    println!("{:?}", connection.read_data::<clientbound::status::Data>());

    let ping = serverbound::status::Data::Ping(PingSb {payload: 6969});
    connection.write_data(ping);
    println!("{:?}", connection.read_data::<clientbound::status::Data>());



    let mut connection = Connection::new("127.0.0.1:25565");


    let handshake = serverbound::handshaking::Data::Handshake(HandshakeSb::new("127.0.0.1".to_string(), 25565, VarInt { val: 2 }));
    connection.write_data(handshake);

    let inner = LoginStartSb { name: McString::new("testbot123") };
    let login_start = serverbound::login::Data::LoginStart(inner);
    connection.write_data(login_start);


    println!("{:?}", connection.read_data::<clientbound::login::Data>());



    loop {
        match connection.read_data() {
            clientbound::play::Data::KeepAlive(keep_alive) => {
                println!("Got keep alive: {:?}", keep_alive);

                let inner = KeepAliveSb { keep_alive_id: keep_alive.keep_alive_id };
                connection.write_data(serverbound::play::Data::KeepAlive(inner));

                let chat_response = ChatMessageSb { message: McString::new(&format!("Hi there, I got a keep alive from you: {}", keep_alive.keep_alive_id))};
                connection.write_data(serverbound::play::Data::ChatMessage(chat_response));

            },
            clientbound::play::Data::ChatMessage(chat_message) => {
                println!("Got chat: {}", chat_message.json_data.0.str);
            },
            clientbound::play::Data::Disconnect(disconnect) => {
                println!("Was disconnected: {:?}", disconnect);
                break;
            }
            _ => (), // I'm only handling Play packets that are KeepAlives
        }
    }
}

// pub fn run() {
//     let mut stream = TcpStream::connect("localhost:25565").unwrap();
//     let settings = protocol::Settings {
//         byte_order: protocol::ByteOrder::BigEndian,
//         ..Default::default()
//     };
//
//     let login = true;
//
//     if !login {
//         let inner = Handshake::new("127.0.0.1".to_string(), 25565, VarInt { val: 1 });
//         let mut first_handshake = PacketSb::new(PacketKind::Handshake(inner.clone()));
//         first_handshake.update_length(inner.length());
//
//
//         // let mut bytes = first_handshake.raw_bytes(&settings).unwrap();
//         // println!("Packet is: {:02x}", bytes.iter().format(" "));
//
//
//         let mut handshake_request = PacketSb::new(PacketKind::Request(Request));
//         handshake_request.update_length(0);
//
//         check_bytes(&handshake_request);
//         //
//         // let mut parsed: Packet = protocol::Parcel::from_raw_bytes(&bytes, &settings).unwrap();
//         // println!("Parsed back: {:?}", parsed);
//
//
//         // let mut reader = &bytes[..];
//         // let reader = &mut reader;
//         // read_handshake_packet(reader);
//
//         stream.write_all(&first_handshake.raw_bytes(&settings).unwrap());
//         stream.write_all(&handshake_request.raw_bytes(&settings).unwrap());
//
//
//         let resp = ExplPacket::<JsonResponse>::read_field(&mut stream, &settings, &mut protocol::hint::Hints::default()).unwrap();
//         println!("Resp: {:?}", resp);
//     } else {
//         let inner = Handshake::new("127.0.0.1".to_string(), 25565, VarInt { val: 2 });
//         let mut first_handshake = PacketSb::new(PacketKind::Handshake(inner.clone()));
//         first_handshake.update_length(inner.length());
//         stream.write_all(&first_handshake.raw_bytes(&settings).unwrap());
//
//         let inner = LoginStart { name: McString::new("testbot123") };
//         let mut login_start = PacketSb::new(PacketKind::LoginStart(inner.clone()));
//         login_start.update_length(inner.length());
//         stream.write_all(&login_start.raw_bytes(&settings).unwrap());
//
//
//
//
//         let set_compression = ExplPacket::<SetCompression>::read_field(&mut stream, &settings, &mut protocol::hint::Hints::default()).unwrap();
//         println!("Resp: {:?}", set_compression);
//
//         // println!("first_varint: {:?}", VarInt::read_field(&mut stream, &settings, &mut protocol::hint::Hints::default()));
//
//
//         // read_1024(&mut stream);
//
//         let VarInt { val: packet_length } = VarInt::read_field(&mut stream, &settings, &mut protocol::hint::Hints::default()).unwrap();
//         let VarInt { val: data_length } = VarInt::read_field(&mut stream, &settings, &mut protocol::hint::Hints::default()).unwrap();
//
//         println!("Got login packet with packet_length: {}, data_length: {}", packet_length, data_length);
//
//         if data_length == 0 {
//             let id: u8 = u8::read_field(&mut stream, &settings, &mut protocol::hint::Hints::default()).unwrap();
//
//             let login_success = LoginSuccess::read_field(&mut stream, &settings, &mut protocol::hint::Hints::default()).unwrap();
//             println!("Uncompressed Resp: {:?}", login_success);
//         } else {
//             let mut ibuf = vec![0; packet_length as usize - 1];
//             stream.read_exact(&mut ibuf);
//             println!("got: {:02x}", ibuf[0..].iter().format(" "));
//             println!("str: {}", String::from_utf8_lossy(&ibuf[1..]));
//
//
//             let mut buf = io::Cursor::new(ibuf);
//
//
//
//             let mut decoder = Decoder::new(&mut buf).unwrap();
//             let id: u8 = u8::read_field(&mut decoder, &settings, &mut protocol::hint::Hints::default()).unwrap();
//
//             let login_success = LoginSuccess::read_field(&mut decoder, &settings, &mut protocol::hint::Hints::default()).unwrap();
//             println!("Compressed Resp: {:?}", login_success);
//             // println!("Resp is: {:02x}", login_success.raw_bytes(&settings).unwrap().iter().format(" "));
//
//         }
//
//
//
//         // loop keepalive
//         // now i need to notice and ignore packets i dont handle
//         // how to? every packet starts with length.
//         // 1) read length
//         // 2) read packet id
//         // 3) if it's not packet id I want, skip next length -1 bytes
//         // 4) repeat
//
//         let mut buffer = [0u8; 1024000];
//
//         loop {
//             let VarInt { val: packet_length } = VarInt::read_field(&mut stream, &settings, &mut protocol::hint::Hints::default()).unwrap();
//
//             let mut ibuf = vec![0; packet_length as usize];
//             stream.read_exact(&mut ibuf);
//             // println!("got: {:02x}", ibuf[0..].iter().format(" "));
//             // println!("str: {}", String::from_utf8_lossy(&ibuf[1..]));
//
//
//             let mut buf = io::Cursor::new(ibuf);
//             let data_length = VarInt::read_field(&mut buf, &settings, &mut protocol::hint::Hints::default()).unwrap();
//
//             // let data_length = VarInt::read_field(&mut stream, &settings, &mut protocol::hint::Hints::default()).unwrap();
//
//             println!("got packet with packet_length: {}, data_length: {}", packet_length, data_length.val);
//
//             // stream.read_exact(&mut buffer[0..(length - 1) as usize]);
//             // println!("got: {:02x}", buffer[0..(length - 1) as usize].iter().format(" "));
//
//             if data_length.val == 0 { // Uncompressed
//                 let id: u8 = u8::read_field(&mut buf, &settings, &mut protocol::hint::Hints::default()).unwrap();
//
//                 if id != 0x21 {
//                     // stream.read_exact(&mut buffer[0..(packet_length - 2) as usize]);
//                     // println!("got: {:02x}", buffer[0..(length - 1) as usize].iter().format(" "));
//                 } else {
//                     let keep_alive_id: i64 = i64::read_field(&mut buf, &settings, &mut protocol::hint::Hints::default()).unwrap();
//                     println!("Got keep alive: {}", keep_alive_id);
//
//                     let inner = KeepAlive { keep_alive_id };
//                     let mut keep_alive_response = CompressedPacketSb::new(PacketKind::KeepAlive(inner.clone()));
//                     keep_alive_response.update_length(8);
//                     stream.write_all(&keep_alive_response.raw_bytes(&settings).unwrap());
//
//                     let mut bytes = keep_alive_response.raw_bytes(&settings).unwrap();
//                     println!("KeepAliveResponse is: {:02x}", bytes.iter().format(" "));
//                 }
//             } else { // Compressed
//
//                 let mut new = Vec::with_capacity(data_length.val as usize);
//                 {
//                     let mut reader = ZlibDecoder::new(&mut buf);
//                     reader.read_to_end(&mut new).unwrap();
//                 }
//                 println!("got comppressed length: {}", new.len());
//
//                 let mut reader = io::Cursor::new(new);
//
//
//                 println!("reached once");
//
//                 // stream.read_exact(&mut buffer[0..(packet_length - data_length.length()) as usize]);
//                 // println!("got: {:02x}", buffer[0..(packet_length - data_length.length()) as usize].iter().format(" "));
//
//                 // let mut temp_buff = Vec::new();
//                 // let mut temp_buff = Vec::with_capacity(data_length.val as usize);
//
//                 // let mut decoder = Decoder::new(&mut stream).unwrap();
//                 // let mut temp_buf = &buffer[0..(packet_length - data_length.length()) as usize];
//                 // let mut decoder = ZlibDecoder::new(&mut stream);
//                 // decoder.read_to_end(&mut temp_buff);
//                 // println!("vec: {:?}", temp_buff);
//                 // println!("got: {:02x}", temp_buff[0..(data_length.val as usize)].iter().format(" "));
//
//                 let id: u8 = u8::read_field(&mut reader, &settings, &mut protocol::hint::Hints::default()).unwrap();
//
//                 if id != 0x21 {
//                     // reader.read_exact(&mut buffer[0..(data_length - 1) as usize]);
//                     // println!("got: {:02x}", buffer[0..(length - 1) as usize].iter().format(" "));
//                 } else {
//                     let keep_alive_id: i64 = i64::read_field(&mut reader, &settings, &mut protocol::hint::Hints::default()).unwrap();
//                     println!("Got keep alive: {}", keep_alive_id);
//
//                     let inner = KeepAlive { keep_alive_id };
//                     let mut keep_alive_response = CompressedPacketSb::new(PacketKind::KeepAlive(inner.clone()));
//                     keep_alive_response.update_length(8);
//                     stream.write_all(&keep_alive_response.raw_bytes(&settings).unwrap());
//
//                     let mut bytes = keep_alive_response.raw_bytes(&settings).unwrap();
//                     println!("KeepAliveResponse is: {:02x}", bytes.iter().format(" "));
//                 }
//             }
//
//
//         }
//
//     }
//
// }



// fn check_bytes<T: Parcel>(parc: &T) {
//     let mut bytes = parc.raw_bytes(&protocol::Settings {
//         byte_order: protocol::ByteOrder::BigEndian,
//         ..Default::default()
//     }).unwrap();
//     println!("Parcel is: {:02x}", bytes.iter().format(" "));
// }
//
// use byteorder::{ByteOrder, LittleEndian, BigEndian};
// use crate::connection::Connection;
// use std::collections::HashMap;
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



// pub fn run() {
//     let stream = TcpStream::connect("127.0.0.1:25565").unwrap();
//     let settings = protocol::Settings {
//         byte_order: protocol::ByteOrder::BigEndian,
//         ..Default::default()
//     };
//     // let mut connection = protocol::wire::stream::Connection::new(stream, protocol::wire::middleware::pipeline::default(), settings.clone());
//
//     let inner = Handshake::new("127.0.0.1".to_string(), 25565, VarInt { val: 2 });
//     let mut first_handshake = Packet { length: VarInt{val:0}, data: PacketKind::Handshake(inner.clone()) };
//     first_handshake.update_length(inner.length());
//
//
//     let mut bytes = first_handshake.raw_bytes(&settings).unwrap();
//     println!("Packet is: {:02x}", bytes.iter().format(" "));
//
//
//     check_bytes(inner.protocol_version);
//
//
//     let mut reader = &bytes[..];
//     let reader = &mut reader;
//     read_handshake_packet(reader);
//
//     // connection.send_packet(&first_handshake).unwrap();
//     //
//     // loop {
//     //     if let Some(response) = connection.receive_packet().unwrap() {
//     //         println!("{:?}", response);
//     //         break;
//     //     }
//     // }
// }
