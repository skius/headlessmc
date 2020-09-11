use std::io::prelude::*;
use std::net::TcpListener;
use std::{fs, io};
use std::thread;
use std::time::Duration;
use std::str::{from_utf8, from_utf8_unchecked};
use headlessmc::*;

extern crate protocol;
#[macro_use] extern crate protocol_derive;
use std::net::TcpStream;

fn main() {
    better_run();
}

// fn write_varint<W: Write>(writer: &mut W, mut value: i32) -> Result<(), io::Error> {
//     let mut buffer = [0; 5]; // VarInts are never longer than 5 bytes
//     let mut counter = 0;
//
//     loop {
//         let mut temp = (value & 0b01111111) as u8;
//
//         value >>= 7;
//         if value != 0 {
//             temp |= 0b10000000;
//         }
//
//         buffer[counter] = temp;
//
//         counter += 1;
//
//         if value == 0 {
//             break;
//         }
//     }
//
//     writer.write_all(&buffer[0..counter])?;
//
//     Ok(())
// }