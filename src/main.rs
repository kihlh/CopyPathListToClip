#![allow(
    dead_code,
    unused_imports,
    unused_parens,
    unused_variables,
    unused_mut,
    unused_must_use,
    unused_assignments,
    non_snake_case
)]

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::ops::Add;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;
use std::sync::Mutex;
use clipboard::{ClipboardProvider, ClipboardContext};
// use regex::Regex;
use serde_json::json;
use serde_json::{Error, Result, Value};
use std::io;
use std::time::Duration;
use std::{process, thread};

fn main() -> std::io::Result<()> {
    // 获取命令行内容并转为数组
    let args: std::env::Args = std::env::args();
    let mut args: Vec<String> = Vec::new();
    std::env::args().for_each(|value| {
        args.push(value);
    });

    let mut text = format!("");
    let mut index = 0 ;

    let mut arr = Vec::new();

    for path  in args.clone() {
        if(index!=0){
            arr.push(Value::String((path)));
            // text.push_str(format!("\"{}\",",path).as_str());
        }
        index +=1;
    }   
    let path_list_array: Value = Value::Array(arr);

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(path_list_array.to_string().to_owned()).unwrap();
    
    Ok(())
}

/**
 * 文本是否相等
 */
fn str_eq_str(str: String, eq_str: String) -> bool {
    if (str.len() != eq_str.len()) {
        return false;
    };

    // 转为二进制再判断
    let str_buf = str.as_bytes();
    let eq_str_buf = eq_str.as_bytes();
    return str_buf.eq(eq_str_buf);
}

/**
 * 文本是否相等
 */
fn str_eq_ostr(str: String, eq_str: &str) -> bool {
    return str_eq_str(str, String::from(eq_str));
}
// json 数据的文本是否重合
fn json_eq_str(value: &serde_json::Value, key: &str, eq_str: &str) -> bool {
    return value[key].is_string() && value[key] == eq_str;
}

// JSON 的数字数据是否重合
fn json_eq_num(value: &serde_json::Value, key: &str, eq_i32: i32) -> bool {
    return value[key].is_number() && value[key] == eq_i32;
}
