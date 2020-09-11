use protocol::{Parcel, Settings};
use std::io::{Write, Read};
use protocol::hint::Hints;


#[derive(Clone, Debug, PartialEq)]
pub struct McString {
    pub length: VarInt,
    pub str: String,
}

impl McString {
    pub fn new(str: &str) -> McString {
        McString { length: VarInt { val: str.len() as i32 } , str: str.to_string()}
    }

    // pub fn length(&self) -> i32 {
    //     self.length.length() + match self.length {
    //         VarInt { val: len } => len,
    //     }
    // }
}

impl Parcel for McString {
    const TYPE_NAME: &'static str = "String";

    fn read_field(read: &mut dyn Read, settings: &Settings, hints: &mut Hints) -> Result<Self, protocol::Error> {
        let VarInt { val: length } = VarInt::read_field(read, settings, hints).unwrap();
        let mut final_string = Vec::<u8>::with_capacity(length as usize);
        let mut buf = [0u8];

        for _ in 0..length {
            read.read_exact(&mut buf).unwrap();
            final_string.push(buf[0]);
        }

        // let final_string = &final_string[..];
        Ok(McString::new(&String::from_utf8(final_string).unwrap()))
    }

    fn write_field(&self, write: &mut dyn Write, settings: &Settings, hints: &mut Hints) -> Result<(), protocol::Error> {
        self.length.write_field(write, settings, hints).unwrap();
        write.write(self.str.as_bytes()).unwrap();

        Ok(())
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct VarInt {
    pub val: i32,
}
//
// impl VarInt {
//     pub fn length(&self) -> i32 {
//         self.raw_bytes(&protocol::Settings::default()).unwrap().len() as i32
//     }
// }

impl Parcel for VarInt {
    const TYPE_NAME: &'static str = "VarInt";

    fn read_field(read: &mut dyn Read, settings: &Settings, hints: &mut Hints) -> Result<Self, protocol::Error> {
        const PART: u32 = 0x7F;
        let mut size = 0;
        let mut val = 0u32;
        loop {
            let mut byte = [0u8;1];
            read.read_exact(&mut byte).unwrap();
            let b = byte[0] as u32;

            val |= (b & PART) << (size * 7);
            size += 1;
            if size > 5 {
                panic!("Incorrect VarInt");
            }
            if (b & 0x80) == 0 {
                break;
            }
        }

        Ok(VarInt { val: val as i32 })
    }

    fn write_field(&self, write: &mut dyn Write, settings: &Settings, hints: &mut Hints) -> Result<(), protocol::Error> {
        let mut buffer = [0; 5]; // VarInts are never longer than 5 bytes
        let mut counter = 0;
        let mut value = self.val;

        loop {
            let mut temp = (value & 0b01111111) as u8;

            value = ((value as u32) >> 7) as i32;
            if value != 0 {
                temp |= 0b10000000;
            }

            buffer[counter] = temp;

            counter += 1;

            if value == 0 {
                break;
            }
        }

        write.write_all(&buffer[0..counter])?;

        Ok(())
    }


}

