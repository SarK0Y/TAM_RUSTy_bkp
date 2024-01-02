use num_traits::{FloatErrorKind, ToPrimitive};
use once_cell::sync::OnceCell;
use std::{
    fs::File,
    path::Path,
    io::{
        ErrorKind,
        Read
    }
};

use crate::core18::errMsg;
pub(crate) unsafe fn swtch_fn(indx: i8, cmd: String){
    static mut fst_run: bool = true;
    static mut fn_indx: u8 = 0;
    static mut fn_: OnceCell<Vec<fn(String) -> bool>> = OnceCell::new();
    if fst_run{
        let fn_vec: Vec<fn(String) -> bool> = Vec::new();
        fn_.set(fn_vec); fst_run = false;
    }
}
pub(crate) fn run_viewer(cmd: String) -> bool{
    let func_id = crate::func_id18::viewer_;
    let  (app_indx, file_indx) = crate::split_once(&cmd, " ");
    if app_indx == "none" || file_indx == "none"{
        crate::core18::errMsg("To run file w/ viewer, You need to type '<indx of viewer> <index of file>'", func_id);
        return false
    }
    
    false
}
pub(crate) fn get_viewer(indx: usize, func_id: i64, thread_safe: bool) -> String{
    let mut func_id_loc = func_id;
    if thread_safe {
        let rnd = get_rnd_u64();
        if !rnd.1{errMsg("/dev/urandom and /dev/random either don't exist or aren't achivable on Your system", func_id); return "none".to_string();}
        func_id_loc = rnd.0.to_i64().unwrap();
    }
    let ret = unsafe {set_o_get_usize(indx, func_id_loc)};
    if ret.1{return unsafe{crate::page_struct("", crate::VIEWER_, func_id_loc).str_};}
"locked".to_string()
}
pub(crate) fn get_num_of_viewers(func_id: i64) -> i64{return unsafe{crate::page_struct("", crate::NUM_OF_VIEWERS, func_id).int}}
pub(crate) fn add_viewer(val: &str, func_id: i64) -> String{return unsafe{crate::page_struct(val, crate::set(crate::VIEWER_), func_id).str_}}
pub(crate) unsafe fn set_o_get_usize(val: usize, func_id: i64) -> (usize, bool){
    static mut owner_id: i64 = i64::MIN;
    static mut actual_val: usize = 0;
    if owner_id == func_id && val == usize::MAX{
        owner_id = i64::MIN;
        return (actual_val, true)
     }
    if owner_id == i64::MIN{owner_id = func_id; actual_val = val; return (val, true);}
    (usize::MAX, false)
}
pub(crate) fn get_rnd_u64() -> (u64, bool){
    let mut get_rnd_device: File = match File::open("/dev/urandom"){
        Ok(File) => File,
        Err(e) => match e.kind(){
           ErrorKind::NotFound => match File::open("/dev/random"){
            Ok(File) => File,
            _ => return (0, false)
        }
        _ => return (0, false)}
    };
    let mut buf: [u8; 8] = [0; 8]; get_rnd_device.read(&mut buf);
    let mut rnd_u64: u64 = 0;
    for i in 0..buf.len()-1{
        rnd_u64 += u64::from(buf.get(i).unwrap() << i*8);
    }
    return (rnd_u64, true)
}
pub(crate) fn form_list_of_viewers(){
let args: Vec<_> = crate::env::args().collect();
let arg = args.as_slice();
for i in 1..args.len(){
    if arg[i] == "-view_w" || arg[i] == "-view-w" {
        let viewer: String = (args[i + 1]).chars().collect();
        add_viewer(&viewer, -1);
    }
}
}
pub(crate) fn print_viewers(){
let num_of_viewers = get_num_of_viewers(-1).to_usize().unwrap();
    for i in 0..num_of_viewers{
        println!("{}", get_viewer(i, -1, true));
    }
}
