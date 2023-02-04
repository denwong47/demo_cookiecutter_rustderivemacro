extern crate proc_macro;

use demo_cookiecutter_rustderivemacro::prelude::*;

#[derive(DemoDerive)]
pub struct MyStruct {
    _populate_me: i32,
}

fn main() {
    println!("main.rs has run!");
}
