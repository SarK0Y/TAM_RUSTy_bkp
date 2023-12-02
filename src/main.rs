#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
const STOP_CODE_: i64 = 1;
const KONSOLE_TITLE_: i64 = 2;
const LEFT_SHIFT_4_CUR_: i64 = 3;
const CUR_CUR_POS_: i64 = 4;
const NUM_PAGE_: i64 = 5;
const NUM_COLS_: i64 = 6;
const COL_WIDTH_: i64 = 7;
const NUM_ROWS_: i64 = 8;
const NUM_SPACES_: i64 = 9;
const NUM_FILES_: i64 = 10;
const COUNT_PAGES_: i64 = 11;
const NEWS_BAR_: i64 = 12;
const ASK_USER_: i64 = 13;
const dontDelFromTableJustMark_: i64 = 14;
const RUNNING_: i64 = 15;
const VIEWER_: i64 = 16;
const MODE2RUN_: i64 = 17;
const PRNT_: i64 = 18;
const FULL_PATH_: i64 = 19;
const MAINPATH_: i64 = 20;
use std::env;
use colored::Colorize;
use substring::Substring;
use std::str::{self, from_utf8};
use std::string;
use chrono::{DateTime, Local};
use std::io::{self, Write};
use std::any::{self, type_name};
use std::fmt;
use std::fs::File;
use std::fs;
use cli_table::format;
use once_cell::unsync::OnceCell;
use std::cell::Cell;
use std::thread;
use threadpool::{ThreadPool, Builder};
use std::sync::mpsc;
use std::ffi::CString;
use std::process::{Command, Stdio};
use std::path::Path;
struct child2run{
    running: i64,
    viewer: i64,
    mode2run: i64,
    prnt: String,
    full_path: String
}
struct page_struct{
    stop_code: String,
    konsole_title: String,
    left_shift_4_cur: i64,
    cur_cur_pos: i64,
    num_page: i64,
    num_cols: i64,
    col_width: i64,
    num_rows: i64,
    num_spaces: i64,
    num_files: i64,
    count_pages: i64,
    news_bar: String,
    ask_User: String,
    c2r: child2run
}

fn set(item: i64) -> i64{
    return item * -1;
}
fn this_item_takes_global_val(id: i64) -> i64 {
    return set(id);
}
unsafe fn page_struct(val: &str, id_of_val: i64, id_of_caller: i64) -> String
{
    let vec_str: Vec<String> = vec![val.to_string()];
    static mut STOP_CODE: OnceCell<String> = OnceCell::new(); // 1
    static mut KONSOLE_TITLE: OnceCell<String> = OnceCell::new(); // 2
    static mut LEFT_SHIFT_4_CUR: i64 = 0; // 3
    static mut CUR_CUR_POS: i64 = 0; //4
    static mut NUM_PAGE: i64 = 0; //5
    static mut NUM_COLS: i64 = 3; //6
    static mut COL_WIDTH: i64 = 70; //7
    static mut NUM_ROWS: i64 = 9; //8
    static mut NUM_SPACES: i64 = 4; //9
    static mut NUM_FILES: i64 = 0; //10
    static mut COUNT_PAGES: i64 = 0; //11
    static mut NEWS_BAR: OnceCell<String> = OnceCell::new(); //12
    static mut ASK_USER: OnceCell<String> = OnceCell::new(); //13
    static mut dontDelFromTableJustMark: bool = true; //14
    static mut RUNNING: OnceCell<Vec<Command>> = OnceCell::new(); //15
    static mut VIEWER: i64 = 0; //16
    static mut MODE2RUN: OnceCell<(String, String)> = OnceCell::new(); //17
    static mut PRNT: OnceCell<String> = OnceCell::new(); //18
    static mut FULL_PATH: OnceCell<String> = OnceCell::new(); //19
    static mut MAINPATH: OnceCell<String> = OnceCell::new(); //20
    //let mut tst: String = "5".to_string();
    let _ = STOP_CODE.set("∇\n".to_string());
    //let fn_ptr_get_string: fn(&str) -> String = get_string;
    if id_of_val == STOP_CODE_ {
        return STOP_CODE.get().unwrap().to_string();
    }
    if id_of_val == set(STOP_CODE_) {STOP_CODE.take(); let _ = STOP_CODE.set(val.to_string()); return "ok".to_string();}
    if id_of_val == MAINPATH_ {
        return MAINPATH.get().unwrap().to_string();
    }
     if id_of_val == set(MAINPATH_) {MAINPATH.take(); let _ = MAINPATH.set(val.to_string()); return "ok".to_string();}
    return "none".to_string();
}
struct ret0 {
    s: [char; 512],
    res: bool
}
fn initSession() -> bool{
    let func_id = 1;
    let run_command = Command::new("whoami")
    .output()
    .expect("wrong");
if !run_command.status.success(){
    io::stdout().write_all(&run_command.stdout).unwrap();
    io::stderr().write_all(&run_command.stderr).unwrap();
    return false;
}  
let timestamp = Local::now();
let mainpath: String = format!("/home{}/.TAM_SESSIONS/{timestamp}/", from_utf8(&run_command.stdout).unwrap().to_string());

if !Path::new(&mainpath).exists(){
    fs::create_dir(&mainpath);
}
if !Path::new(&mainpath).exists(){
    println!("{mainpath} cannot be made");
    return false;
}
unsafe{page_struct("", STOP_CODE_, -1);}
    return true;
}
fn errMsg_dbg(msg: &str, val_func_id: i64, delay: f64) {

}
fn check_substr(orig: &String, probe: &str, start_from: usize) -> bool{
    let probe: String = String::from(probe.to_string());
    let substr: &str= &orig.as_str();
    let substr = substr.substring(start_from, probe.len() - 1).to_string();
     if probe.ne(&substr){
        return false;
     }
     return true;
}
fn check_substring(orig: String, probe: String, start_from: usize) -> bool{
    let substr: &str= &orig.as_str();
    let substr = substr.substring(start_from, probe.len() - 1).to_string();
     if probe.ne(&substr){
        return false;
     }
     return true;
}
fn checkArg(key: &str) -> bool{
    let len_of_cmd_line = env::args().len();
    let args: Vec<String> = env::args().collect();
    let i: i64 = 0;
    for i in 0..len_of_cmd_line{
        if args[i] == key.to_string(){
            return true;
        }
    }
    return false;
}
fn put_in_name() -> String{
    let mut ret: String = String::new();
    ret.push_str("|");
    let len_of_cmd_line = env::args().len();
    let args: Vec<String> = env::args().collect();
    let i: i64 = 0;
    for i in 0..len_of_cmd_line{
        if args[i] == "-in_nane".to_string() || args[i] == "-in-nane".to_string(){
            let cmd = format!("{}|", form_grep_cmd(&args[i+1]));
            ret.push_str(&cmd);
        }
    }
    return ret;
}
fn form_grep_cmd(in_name: &String) -> String{
    let mut ret: String = String::new();
    ret.push_str("grep ");
    let split = "go go";
    if check_substr(in_name, "pass==", 0){
     //  let mut in_name = in_name.as_str();
       let in_name = in_name.replace("pass==", "");
       let split = in_name.split(" ");
    }
    let (opts, name) = split.split_once(" ").unwrap();
    let val4grep = format!("{} '{}'", opts, name);
    ret.push_str(&val4grep);
    return ret;
}
#[inline(always)]
fn find_files(path: &str, in_name: &str, path_2_tmp_file: &str) -> bool{
let mut list_of_found_files: Vec<String> = vec![]; 
let stopCode: String; 
unsafe {stopCode = page_struct("", STOP_CODE_,-1);}
let cmd: String = format!("find -L '{path}' -type f{in_name};echo '\n{stopCode}'");
let fstdout: CString; 
let stderr_path = "stderr";
unsafe {
    let stderr_path = format!("{}/stderr", page_struct("", MAINPATH_, -1));
    let path_2_list_of_found_files = format!("{}/found_files", page_struct("", MAINPATH_, -1));
    fstdout = CString::new(path_2_list_of_found_files).unwrap(); 
    libc::mkfifo(fstdout.as_ptr(), 0o644);
}
let fstderr = File::open(stderr_path).unwrap();
let fstdout0 = File::open(fstdout.into_string().expect("cstring stdout to string failed")).unwrap();
//let mut fstdout0 = io::BufReader::new(fstdout0);
let run_command = Command::new(cmd)
    .stdout(fstdout0)
    .stderr(fstderr)
    .output()
    .expect("can't run command in find_files");
if !run_command.status.success(){
    io::stdout().write_all(&run_command.stdout).unwrap();
    io::stderr().write_all(&run_command.stderr).unwrap();
    return false;
}
return true;
}
fn get_arg_in_cmd(key: &str) -> ret0{
let mut s: [char; 512] = ['\0'; 512];
let mut ret: ret0 = ret0{s, res: false};
let args: Vec<_> = env::args().collect();
let args_2_str = args.as_slice();
for i in 1..args.len(){
    if args_2_str[i] == key {
        let arr: Vec<char> = (args[i + 1]).chars().collect();
        if arr.len() > 512 {
            for i in 0..511{
               ret.s[i] = arr[i];
            }
        } else{
            for i in 0..arr.len(){
               ret.s[i] = arr[i];
            }
        }
        ret.res = true;
        return ret;
}
}
ret.res = false;
return ret;
}
fn main (){
let x: i64= 5;
let out: ret0 = get_arg_in_cmd("-tst");
let out1: ret0 = get_arg_in_cmd("-тст");
println!("argument from cmd (-tst) {}", String::from_iter(out.s));
println!("argument from cmd (-тст) {}", String::from_iter(out1.s));
unsafe {println!("get stop code {}", page_struct("", 1, 0));}
unsafe {println!("set stop code {}", page_struct("777", set(1), 0));}
unsafe {println!("get stop code {}", page_struct("", 1, 0));}
println!("run find_files status {}", find_files("/tmp", "tst", ""));
return;
}
