/// Author: Tenton Lien
/// Date: 2020/10/09

use std::env;
use std::process::exit;
use log::{error, info, LevelFilter};

use class_loader::JavaClass;
use util::logger::Logger;

mod class_loader;
mod util;
mod vm_structure;


fn main() {
    // Config logger
    log::set_boxed_logger(Box::new(Logger {})).unwrap();
    log::set_max_level(LevelFilter::Debug);

    String::from("hello");

    // Check args from command line
    let args_vec: Vec<String> = env::args().collect();
    if args_vec.len() < 2 {
        error!("Target bytecode file required");
        exit(-1);
    }

    // Load Java class
    let mut java_class = JavaClass::new();
    java_class.load(&args_vec[1]);
    // java_class.print_info();
    // java_class.print_byte_code();
}
