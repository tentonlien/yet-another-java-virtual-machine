/// Author: Tenton Lien
/// Date: 2020/10/09

use std::process::exit;
use std::fs;
use log::{debug, error};
use crate::class_loader::constant_pool_analyzer::ConstantPool;

mod constant_pool_analyzer;
mod instruction_analyzer;


pub struct JavaClass {
    minor_version: u16,
    major_version: u16,
    constant_pool_size: u16,
    constant_pool: ConstantPool,
    byte_code_stream: Vec<u8>,
}

impl JavaClass {
    pub fn new() -> JavaClass {
        JavaClass {
            minor_version: 0,
            major_version: 0,
            constant_pool_size: 0,
            constant_pool: ConstantPool::new(),
            byte_code_stream: vec![],
        }
    }

    pub fn load(&mut self, path: &str) {

        // Read class file
        self.byte_code_stream = fs::read(path).unwrap();
        debug!("Length of byte codes: {}", self.byte_code_stream.len());

        // Check magic number
        if !(self.byte_code_stream[0] == 0xCA && self.byte_code_stream[1] == 0xFE &&
            self.byte_code_stream[2] == 0xBA && self.byte_code_stream[3] == 0xBE) {
            error!("Invalid magic number!");
            exit(-1);
        }

        // Read version info
        self.minor_version = self.byte_code_stream[4] as u16 * 256 + self.byte_code_stream[5] as u16;
        self.major_version = self.byte_code_stream[6] as u16 * 256 + self.byte_code_stream[7] as u16;

        self.constant_pool_size = (self.byte_code_stream[8] as u16) * 256 + (self.byte_code_stream[9] as u16);

        debug!("Version: Minor = {} / Major = {} (JDK {})", self.minor_version, self.major_version, match self.major_version {
            0x00..=0x33 => String::from("earlier than JDK 1.8"),
            0x34 => "1.8".to_string(),
            0x35..=0x41 => (self.major_version - 44).to_string(),
            0x42..=0xFF => String::from("later than JDK 15"),
            _ => "Unidentified".to_string()
        });

        self.constant_pool.analyze(&self.byte_code_stream, self.constant_pool_size);
    }

    pub fn print_byte_code(&self) {
        for i in 0..self.byte_code_stream.len() {
            print!("{:02X} ", self.byte_code_stream[i]);
            if (i + 1) % 16 == 0 {
                println!();
            }
        }
        println!();
    }
}