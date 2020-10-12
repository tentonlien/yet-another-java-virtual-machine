/*
 * Author: Tenton Lien
 * Date: 10/11/2020
 */

pub struct ConstantPool {
    constants: Vec<ConstantInfo>
}

impl ConstantPool {
    pub fn new() -> ConstantPool {
        ConstantPool {
            constants: Vec::new()
        }
    }

    pub fn analyze(&mut self, byte_code: &Vec<u8>, constant_amount: u16) {
        const CONSTANT_POOL_START_POSITION: u16 = 10;
        let mut cursor = CONSTANT_POOL_START_POSITION;
        for _i in 1..constant_amount {
            let tag =  match byte_code[cursor as usize] {
                1 => ConstantTag::Utf8,
                3 => ConstantTag::Integer,
                4 => ConstantTag::Float,
                5 => ConstantTag::Long,
                6 => ConstantTag::Double,
                7 => ConstantTag::Class,
                8 => ConstantTag::String,
                9 => ConstantTag::Fieldref,
                10 => ConstantTag::Methodref,
                11 => ConstantTag::InterfaceMethodref,
                12 => ConstantTag::NameAndType,
                15 => ConstantTag::MethodHandle,
                16 => ConstantTag::MethodType,
                18 => ConstantTag::InvokeDynamic,
                _ => ConstantTag::Unknown
            };
            let mut content: String = String::new();

            cursor += match tag {
                ConstantTag::Utf8 => {
                    let length: u16 = byte_code[(cursor + 1) as usize] as u16 * 256 + byte_code[(cursor + 2) as usize] as u16;
                    for k in (cursor + 3)..(cursor + length + 3) {
                        content.push(byte_code[k as usize] as char);
                    }
                    length + 3
                }
                ConstantTag::Class => {
                    content.push('#');
                    content += (byte_code[(cursor + 1) as usize] as u16 * 256 + byte_code[(cursor + 2) as usize] as u16).to_string().as_str();
                    3
                }
                ConstantTag::Methodref | ConstantTag::Fieldref => {
                    content.push('#');
                    content += (byte_code[(cursor + 1) as usize] as u16 * 256 + byte_code[(cursor + 2) as usize] as u16).to_string().as_str();
                    content += ".#";
                    content += (byte_code[(cursor + 3) as usize] as u16 * 256 + byte_code[(cursor + 4) as usize] as u16).to_string().as_str();
                    5
                }
                ConstantTag::NameAndType => {
                    content.push('#');
                    content += (byte_code[(cursor + 1) as usize] as u16 * 256 + byte_code[(cursor + 2) as usize] as u16).to_string().as_str();
                    content += ":#";
                    content += (byte_code[(cursor + 3) as usize] as u16 * 256 + byte_code[(cursor + 4) as usize] as u16).to_string().as_str();
                    5
                }
                _ => 5
            };
            self.constants.push(ConstantInfo::new(tag, content));
        }
    }

    pub fn print(&self) {
        println!("Constant Pool (Size = {})", self.constants.len());
        for i in 0..self.constants.len() {
            print!("#{} = ", i + 1);
            self.constants[i].print();
        }
    }
}

pub struct ConstantInfo {
    tag: ConstantTag,
    content: String
}

impl ConstantInfo {

    fn new(tag: ConstantTag, content: String) -> ConstantInfo {
        ConstantInfo {
            tag,
            content
        }
    }

    fn print(&self) {
        println!("{:?} ({})", self.tag, self.content);
    }
}

#[derive(Debug)]
enum ConstantTag {
    Utf8,
    Integer,
    Float,
    Long,
    Double,
    Class,
    String,
    Fieldref,
    Methodref,
    InterfaceMethodref,
    NameAndType,
    MethodHandle,
    MethodType,
    InvokeDynamic,
    Unknown
}
