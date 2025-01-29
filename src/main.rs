use tide::prelude::*;
use tide::Request;
use tide::Body;
use tide::http::{mime, StatusCode};
use std::{env};
extern crate log;

mod handler;

#[async_std::main]
async fn main() -> tide::Result<()> {
    env::set_var("RUST_LOG", "debug");

    let args: Vec<String> = env::args().collect();

    tide::log::start();
    let mut app = tide::new();

    app.at("/").get(handler::handler_index);
    app.at("*").get(handler::handler_file);
    app.at("/send").post(handler::handler_message);
    app.at("/getall").post(handler::send_all_messages);

    if args.len() > 1 {
        let addr = args[1].clone();
        app.listen(addr).await?;
    } else {
        app.listen("127.0.0.1:8080").await?;  
    }

    Ok(())
}
