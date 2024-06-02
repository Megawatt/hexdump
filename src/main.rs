/*
******************************************************************************
hexdump
(c) 2024 Palatsi

Simple hexdump utility. 
Limits input length to 4 GB.

Use: Hexdump <filename>
******************************************************************************
*/

use std::{env, io};
use std::fs::File;
use std::io::Read;

const MAX_FILE_SIZE: u64 = 0xFFFF_FFFF;
const BYTES_PER_LINE: usize = 0x10;
const PRINTABLE_ASCII_MIN: u8 = 0x20;
const PRINTABLE_ASCII_MAX: u8 = 0x7E;
const ASCII_DOT: u8 = 0x2E;


fn main() -> Result<(), io::Error>{

    // Read command line args. Must have 1 arg (filename)
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: hexdump <filename>");
        return Ok(());
    }

    let filename: &String = &args[1];

    let mut f: File = File::open(filename)?;

    let mut buf: [u8; BYTES_PER_LINE];
    let mut index: u64 = 0;

    loop {
        if index > MAX_FILE_SIZE {
            break;
        }
        
        buf = [0; BYTES_PER_LINE];

        let bytes_read = f.read(&mut buf)?;

        if bytes_read == 0 {
            if index == 0 {
                println!("hexdump: zero length input file");
            }
            break;
        }

        print!("{:04X}_{:04X} ", index>>16, index & 0xFFFF);
        
        for i in 0..BYTES_PER_LINE {
            if i < bytes_read {
                print!("{:02X } ", buf[i]);
                if (buf[i] < PRINTABLE_ASCII_MIN) || 
                   (buf[i] > PRINTABLE_ASCII_MAX) {

                    buf[i] = ASCII_DOT;
                }
            } else {
                print!("-- ");
            }
        }
        println!("{}", String::from_utf8_lossy(&buf));
        index += BYTES_PER_LINE as u64;
    }

    Ok(())

}

