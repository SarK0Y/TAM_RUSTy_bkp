use chrono::format;
use num_traits::ToPrimitive;

use crate::{exts::globs_uses, run_cmd0, ps18::{shift_cursor_of_prnt, get_prnt}, swtch::local_indx, core18::calc_num_files_up2_cur_pg};
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
    let len = origString.chars().count(); let mut ret = String::new();
    for i in 0..len{
        if i != indx{
         let char1: char = match origString.chars().nth(i){
            Some(ch) => ch,
            _ => {return ret}
        };
        ret.push(char1);
        }
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
pub fn bksp() -> String{
       let mut len = get_prnt(-3).chars().count(); if len == 0 {return get_prnt(-3).to_string();}
     let mut ret = String::new();
     let prnt = get_prnt(-3);
     if len > 0 {len -= 1;}
    let mut indx = unsafe {shift_cursor_of_prnt(2, -2).shift};
    if indx <= len {indx = len - indx;}
    ret = rm_char_from_string(indx, &get_prnt(-3));
        ////println!("ret {}", ret);
    ret
}
pub fn ins_last_char_to_string1_from_string1(indx: usize, origString: String) -> String{
    let mut len = origString.chars().count(); if len == 0 || len == 1 || indx == len -1 {return origString.to_string();}
     let mut ret = String::new();
     len -= 1; 
    let char0: char =match origString.chars().nth(len){
        Some(ch) => ch,
        _ => {/*println!("kkkkkkkkk");*/ return origString.to_string();}
    };
    if crate::dirty!(){
    let msg = format!("'ins_last_char_to_string1_from_string1 indx {} orig{} char0 {} orig len {}'", indx, origString, char0, len);
    //run_cmd0(&msg);
    println!("{}", &msg);
    }
    for i in 0..len {
        let char1: char = origString.chars().nth(i).unwrap();
        if i == indx{ret.push(char0);}
        ret.push(char1);
       // println!("{}", char1);
    }
    ////println!("ret {}", ret);
    ret
}
pub fn ins_last_char_to_string1_from_string1_ptr(indx: usize, origString: &mut String) {
    let mut len = origString.chars().count(); if len == 0 || len == 1 || indx == len -1 {return}
     let mut ret = String::new();
     len -= 1; 
    let char0: char =match origString.chars().nth(len){
        Some(ch) => ch,
        _ => {/*println!("kkkkkkkkk");*/ return;}
    };
    if crate::dirty!(){
    let msg = format!("'ins_last_char_to_string1_from_string1 indx {} orig{} char0 {} orig len {}'", indx, origString, char0, len);
    //run_cmd0(&msg);
    println!("{}", &msg);
    }
    for i in 0..len {
        let char1: char = origString.chars().nth(i).unwrap();
        if i == indx{ret.push(char0);}
        ret.push(char1);
       // println!("{}", char1);
    }
    origString.clear(); origString.push_str(ret.as_str());
    ////println!("ret {}", ret);
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
pub(crate) fn get_proper_indx(indx: i64, fixed_indx: bool) -> (usize, i64){
    let mut fix_inputed_indx = indx;
    if !unsafe {local_indx(false)} && fixed_indx {fix_inputed_indx += calc_num_files_up2_cur_pg();}
    let indx = fix_inputed_indx;
    let mut proper_indx: i64 = 0;
    let mut len: i64 = 0;
    if indx > 0{proper_indx = indx;}
    len =i64::from_str_radix(len_of_front_list().as_str(), 10).unwrap();
    if len == 0{return (usize::MAX, 0i64)}
    if indx > len {proper_indx = (indx - len);}
    if proper_indx < len {return (proper_indx.to_usize().unwrap(), proper_indx)}
    if proper_indx > len {let ret = proper_indx - (proper_indx/len) * len; return (ret.to_usize().unwrap(), ret) }
    return (usize::MAX, 0);
}
pub(crate) fn get_item_from_front_list(indx: i64, fixed_indx: bool) -> String{
    let proper_indx = get_proper_indx(indx, fixed_indx);
    if proper_indx.0 == usize::MAX{return "front list is empty".to_string()}
    return unsafe{lists("", FRONT_, proper_indx.0, GET)}
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