use tide::http::other::Date;
use tide::prelude::*;
use tide::Request;
use tide::Body;
use tide::convert::Deserialize;
use tide::http::{mime, StatusCode};
use tide::Response;
use std::fs;

use std::io::Write;
use std::mem::replace;
use std::str::FromStr;
use std::{env};
use std::io::{BufRead, BufReader};

use chrono::{DateTime, Utc, Local, offset::{FixedOffset, TimeZone}, NaiveDateTime};

extern crate log;

use rand::Rng;

//データ構造
#[derive(Deserialize)]
struct Message {
    date: String,   //日付
    name: String,   //ユーザ名
    content: String //コメント内容
}

//Implement clone function for Message Object
impl Clone for Message {
    fn clone(& self) -> Self{
        Self {
            content: self.content.clone(),
            name: self.name.clone(),
            date: self.date.clone()
           }
    }
}

#[macro_use]

fn get_path_from_webroot(filename: &str) -> String{
    let WEBROOT: &str = "/webroot";
    let path = format!("{}{}{}", env::current_dir().unwrap().display(), WEBROOT, filename);
    return path;
}

//index.htmlを送る
pub async fn handler_index(_req: Request<()>) -> Result<tide::Response, tide::Error>  {
    let path = get_path_from_webroot("/index.html");
    let mut res = tide::Response::new(StatusCode::Ok);
    let body = Body::from_file(path).await?;
    res.set_body(body);
    Ok(res)
}

//それ以外要求されたファイルを送る
pub async fn handler_file(_req: Request<()>) -> Result<tide::Response, tide::Error>  {
    let path = get_path_from_webroot(_req.url().path());
    let mut res = tide::Response::new(StatusCode::Ok);
    let body = Body::from_file(path).await?;
    res.set_body(body);
    Ok(res)
}

//新しいコメントを受信したとき、データベース（ＣＳＶ）に記入し、変換したものを返信
pub async fn handler_message(mut _req:  Request<()>) -> Result<tide::Response, tide::Error>  {
    let mut res = tide::Response::new(StatusCode::Ok);
    let req_body: Message = _req.body_json().await?;
    add_comment( req_body.clone())?;

    let encoded = encode(&req_body.content);

    let body = format!(r#"{{ "date": "{}", "name": "{}", "content" : "{}" }}"#,req_body.date,req_body.name,encoded);

    res.set_body(body);

    Ok(res)
}

//CSVファイルに書き込み
fn add_comment(msg: Message)-> std::io::Result<()> {
    let path = get_path_from_webroot("/msg.csv");
    let line = format!("{},{},{}\n",&msg.date,&msg.name,&msg.content);
    let mut file = fs::OpenOptions::new().write(true).append(true).open(path).unwrap();

    write!(file,"{}",line);

    file.flush()?;
    Ok(())
}

//クライエント側のdisplayAllMessagesに対応（全てのコメントを読み込んで送信）
pub async fn send_all_messages(mut _req:  Request<()>) -> Result<tide::Response, tide::Error>  {
    let path = get_path_from_webroot("/msg.csv");
    let mut res = Response::new(StatusCode::Ok);
    let mut body = String::from_str("[")?;

    let mut file = fs::File::open(path).expect("unable to open");
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Unable to read line");
        let data = line.split(',').collect::<Vec<&str>>();
        if index != 0 {
            body = format!("{},",body);
        }
        body = format!(r#"{}{{ "date": "{}", "name": "{}", "content" : "{}" }}"#,body,data[0],data[1],encode(data[2]));
    }
    body = format!("{}]",body);

    res.set_body(body);

    Ok(res)
}

const MAX_WORDS: i32 = 15;
fn get_word(i: i32) -> &'static str {
    return match i {
        0 => "：） ",
        1 => "(泣) ",
        2 => "ｗｗｗ草 ",
        3 => "（笑） ",
        4 => "(´∇`)bｸﾞｯ ",
        5 => "(･ρ･)ﾉ ",
        6 => " orz ",
        7 => "( ✌'ω')✌",
        8 => "((( ；ﾟДﾟ)) ",
        9 => "( ･･)σ ",
        10 => "(´;ω;｀) ",
        11 => "わ～ ",
        12 => "(>_<) ",
        13 => "(*^^)v ",
        14 => "(>д<*)ｺﾜｲ ",
        _ => ""
    }
}

fn encode(msg: &str) -> String {
    let mut rng = rand::thread_rng();
    let y = rng.gen_range(0,MAX_WORDS) as i32;
    if msg.contains("。"){
        let replacing = get_word(y % MAX_WORDS);
        let new_str = msg.replacen('。',replacing,1);
        return encode(new_str.as_str());
    } else if msg.contains("<br>"){
        let sign = get_word(y % MAX_WORDS);
        let replacing = format!("{}<BR>",sign);
        let new_str = msg.replacen("<br>",&replacing,1);
        return encode(new_str.as_str());
    } else {
        return  String::from(msg.replace("<BR>", "<br>"));
    }
}

