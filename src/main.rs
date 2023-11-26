use std::env;
use once_cell::unsync::OnceCell;
use std::cell::Cell;
fn set(item: i64) -> i64{
    return item * -1;
}
unsafe fn page_struct(val: &str, id_of_val: i64, id_of_caller: i64) -> String
{
    let vec_str: Vec<String> = vec![val.to_string()];
    static mut KONSOLE_TITLE: OnceCell<String> = OnceCell::new();
    static mut LEFT_SHIFT_4_CUR: Cell<i64> = Cell::new(0);
    static mut CUR_CUR_POS: Cell<i64> = Cell::new(0);
    static mut STOP_CODE: OnceCell<String> = OnceCell::new();//Lazy::new(||);
    let _ = STOP_CODE.set("∇\n".to_string());
    //let fn_ptr_get_string: fn(&str) -> String = get_string;
    if id_of_val == 1 {
        return STOP_CODE.get().unwrap().to_string();
    }
     if id_of_val == -1 {STOP_CODE.take(); let _ = STOP_CODE.set(val.to_string()); return "ok".to_string();}
    return "none".to_string();
}
struct ret0 {
    s: [char; 512],
    res: bool
}
#[inline(always)]
fn find_files(path: &str, in_name: &str, path_2_tmp_file: &str) -> Vec<String>{
let mut list_of_found_files: Vec<String> = vec![];
return list_of_found_files;
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
return;
}
