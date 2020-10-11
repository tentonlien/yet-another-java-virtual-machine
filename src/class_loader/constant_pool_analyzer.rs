/*
 * Author: Tenton Lien
 * Date: 10/11/2020
 */

pub struct ConstantPool {
    size: u16
}

pub fn analyze(byte_code: &Vec<u8>, constant_amount: u16) {
    const CONSTANT_POOL_START_POSITION: u16 = 10;
    let mut cursor = CONSTANT_POOL_START_POSITION;
    for i in 1..constant_amount {
        let tag = byte_code[cursor as usize];
        print!("#{} = {}", i, constant_tag(tag));
        cursor += match tag {
            1 => {
                let length: u16 = byte_code[(cursor + 1) as usize] as u16 * 256 + byte_code[(cursor + 2) as usize] as u16;
                length + 3
            }
            7 => 3,
            _ => 5
        };
        println!();
    }
}

fn constant_tag(value: u8) -> &'static str {
    match value {
        1 => "Utf8",
        3 => "Integer",
        4 => "Float",
        5 => "Long",
        6 => "Double",
        7 => "Class",
        8 => "String",
        9 => "Fieldref",
        10 => "Methodref",
        11 => "InterfaceMethodref",
        12 => "NameAndType",
        15 => "MethodHandle",
        16 => "MethodType",
        18 => "InvokeDynamic",
        _ => "Unknown"
    }
}
