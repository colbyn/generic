#![allow(unused)]
use generic_core::value::*;
use generic_core::into::*;
use generic_derive::*;

#[derive(Clone, Debug, IntoGeneric)]
pub struct Test {
    a: String,
    b: String,
}

#[derive(Clone, Debug, IntoGeneric)]
pub enum Alpha {
    One(String),
    Two,
    Three,
}

#[derive(Clone, Debug, IntoGeneric)]
pub struct Beta(String);

fn main() {
    let value = Beta(String::from("test"));
    let value = value.into_generic();
    println!("{:#?}", value);
}
