mod class_loader;

use std::env;
use class_loader::JavaClass;

/*
 * Author: Tenton Lien
 * Date: 10/9/2020
 */

fn main() {
    println!("Java byte code parser starts!");
    for argument in env::args() {
        println!("*** args = {}", argument);
    }

    let mut java_class = JavaClass::new();
    java_class.load("resources/hi.class");
    java_class.print_info();
}
