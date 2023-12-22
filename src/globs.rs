use chrono::format;

use crate::{exts::globs_uses, run_cmd0};
self::globs_uses!();
pub const MAIN0_: i64 =  1;
pub const FRONT_: i64 =  2;
pub const FILTERED_: i64 =  3;
pub const MERGE_: i64 =  4;
pub const LS: i64 =  5;
pub const ADD: i64 =  6;
pub const INS: i64 =  7;
pub const DEL: i64 =  8;
pub const GET: i64 =  9;
pub const LEN: i64 =  10;
pub const SET_FRONT_LIST: i64 =  11;
pub fn rm_char_from_string(indx: usize, origString: &String) -> String{
    let len = origString.len(); let mut ret = String::new();
    for i in 0..len{
        if i != indx{let char1 = origString.chars().nth(i).unwrap(); ret.push(char1);}
    }
    ret
}
pub fn unblock_fd(fd: RawFd) -> io::Result<()> {
    let flags = unsafe { fcntl(fd, F_GETFL, 0) };
    if flags < 0 {return Err(io::Error::last_os_error());}
    let flags = flags | O_NONBLOCK;
    let res = unsafe { fcntl(fd, F_SETFL, flags) };
    if res != 0 {return Err(io::Error::last_os_error());}
    Ok(())
}
pub fn ins_last_char_to_string1_from_string1(indx: usize, origString: String) -> String{
    let mut len = origString.len(); if len == 0 {return origString.to_string();}
     let mut ret = String::new();
    let char0: char =match origString.chars().nth(len - 1){
        Some(ch) => ch,
        _ => {/*println!("kkkkkkkkk");*/ return origString.to_string();}
    };
    if crate::dirty!(){
    let msg = format!("'ins_last_char_to_string1_from_string1 indx {} orig{} char0 {} orig len {}'", indx, origString, char0, len);
    //run_cmd0(&msg);
    println!("{}", &msg);
    }
    for i in 0..len{
        let char1: char = origString.chars().nth(i).unwrap();
        if i != indx{ret.push(char1);}
        else {ret.push(char0);}
       // println!("{}", char1);
    }
    ////println!("ret {}", ret);
    ret
}
pub(crate) fn print_type_of<T>(_: &T) {
println!("{}", std::any::type_name::<T>())
}
pub fn eq_str(str1: &str, str2: &str) -> i64{
let str1_len = str1.len();
let str2_len = str2.len();
if str1_len == 0 || str2_len == 0{return i64::MIN}
let mut result: i64 = 0;
let mut i: usize = 0;
while i < str1_len && i < str2_len {
    if str1.chars().nth(i) == None || str1.chars().nth(i) == None{break;}
    if crate::dirty!(){ println!("eq_str char1 {:?} to char2 {:?} i {}", str1.chars().nth(i), str2.chars().nth(i), i);}
    let a = str1.chars().nth(i).unwrap();
    let b = str2.chars().nth(i).unwrap();
    if a < b {
        result = -1;
        break;
    } 
    if a > b {
        result = 1;
        break;
    }
    i += 1;
}
result
}
pub fn eq_ansi_str(str1: &str, str2: &str) -> i64{
let mut ansi_str1 = str1.bytes(); //ANSIString::from(str1);
let mut ansi_str2 = str2.bytes(); //ANSIString::from(str2);
let str1_len = str1.len();
let str2_len = str2.len();
if str1_len == 0 || str2_len == 0{return i64::MIN}
// Loop over the strings and compare each character
let mut result: i64 = 0;
let mut i: usize = 0;
while i < str1_len && i < str2_len {
    let char_of_str1 = ansi_str1.next();
    let char_of_str2 = ansi_str2.next();
    if char_of_str1 == None || char_of_str2 == None{println!("char is None"); break;}
    if crate::dirty!(){ println!("eq_ansi_str char1 {:?} to char2 {:?} i {}", str1.chars().nth(i), str2.chars().nth(i), i);}
    let a = char_of_str1.unwrap();
    let b = char_of_str2.unwrap();
    if a < b {
        result = -1;
        break;
    } 
    if a > b {
        result = 1;
        break;
    }
    i += 1;
}
result
}
pub fn add_2_main0_list(val: &str) -> String{
    return unsafe{lists(val, MAIN0_, 0, ADD)}
}
pub fn len_of_main0_list() -> String{
    return unsafe{lists("", MAIN0_, 0, LEN)}
}
pub fn len_of_front_list() -> String{
    return unsafe{lists("", FRONT_, 0, LEN)}
}
fn get_proper_indx(indx: i64) -> usize{
    let mut proper_indx: usize = 0;
    let mut len: i64 = 0;
    if indx > 0{proper_indx = indx as usize;}
    len =i64::from_str_radix(len_of_front_list().as_str(), 10).unwrap();
    if len == 0{return usize::MAX}
    if indx > len {proper_indx = (indx - len) as usize;}
    if proper_indx < len as usize{return proper_indx}
    if proper_indx > len as usize{let len: usize = len as usize; return proper_indx - (proper_indx/len) * len }
    return usize::MAX;
}
pub(crate) fn get_item_from_front_list(indx: i64) -> String{
    let proper_indx = get_proper_indx(indx);
    if proper_indx == usize::MAX{return "front list is empty".to_string()}
    return unsafe{lists("", FRONT_, proper_indx, GET)}
}
pub fn set_main0_as_front(){unsafe{lists("", MAIN0_, 0, SET_FRONT_LIST);}}
pub unsafe fn lists(val: &str, list: i64, indx: usize, op_code: i64) -> String{
static mut MAIN0: OnceCell<Vec<String>> = OnceCell::new();
static mut FRONT: OnceCell<&Vec<String>> = OnceCell::new();
if Some(MAIN0.get()) != None{
    let mut main0_vec: Vec<String> = Vec::new();
    MAIN0.set(main0_vec);
}
if list == MAIN0_ {
    if op_code == GET{
        let ret = &MAIN0.get().unwrap()[indx];
        return ret.to_string()
    }
    if op_code == ADD{
        MAIN0.get_mut().unwrap().push(val.to_string());
       return "ok".to_string()
    }
    if op_code == LEN{return MAIN0.get().unwrap().len().to_string()}
    if op_code == SET_FRONT_LIST {
       if Some(FRONT.get()) != None{FRONT.take();}
       FRONT.set(&MAIN0.get().unwrap());
    }
}
if list == FRONT_ {
    if op_code == GET{return FRONT.get().unwrap()[indx].to_string()}
    if op_code == LEN{return FRONT.get().unwrap().len().to_string()}
}
"wrong".to_string()
}