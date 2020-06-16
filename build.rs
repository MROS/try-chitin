#[path = "src/model.rs"]
mod model;
#[path = "src/query.rs"]
mod query;
use chitin::{ChitinCodegen, CodegenOption};
use query::RootQuery;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("src/api_trait.rs")?;
    file.write_all(b"use async_trait::async_trait;")?;
    file.write_all(b"use crate::query::*;")?;
    file.write_all(b"use serde_json::error::Error;")?;
    file.write_all(RootQuery::codegen(&CodegenOption::Server).as_bytes())?;

    // let args: Vec<_> = std::env::args().collect();
    // if args[1] == "server" {
    //     println!("use async_trait::async_trait;");
    //     println!("use crate::query::*;");
    //     println!("use serde_json::error::Error;");
    //     println!("{}",);
    // } else if args[1] == "client" {
    //     println!("{}", RootQuery::codegen(&CodegenOption::Client));
    // } else {
    //     panic!("未知的指令：{}", args[1]);
    // }
    Ok(())
}
