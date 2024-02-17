use chrono::format;
use num_traits::ToPrimitive;
use crate::{exts::globs_uses, run_cmd0, ps18::{shift_cursor_of_prnt, get_prnt, set_ask_user}, swtch::{local_indx, front_list_indx, check_mode, SWTCH_USER_WRITING_PATH, SWTCH_RUN_VIEWER, swtch_fn, set_user_written_path_from_prnt, set_user_written_path_from_strn, user_wrote_path}, core18::calc_num_files_up2_cur_pg, func_id18, ln_of_found_files, read_prnt, get_path_from_strn, repeat_char, set_prnt, rm_file, file_prnt, get_mainpath, run_cmd_str, get_tmp_dir, read_file, mark_front_lst, split_once, fix_num_files};
self::globs_uses!();
pub const MAIN0_: i64 =  1;
pub const FRONT_: i64 =  2;
pub const FILTERED_: i64 =  3;
pub const MERGE_: i64 =  4;
pub const LS_: i64 =  5;
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
pub(crate) fn exclude_strn_from_list(strn: String, list: &str){
    let list_tmp = format!("{}/{}.tmp", get_tmp_dir(18551), list);
    let list = format!("{}/{}", get_tmp_dir(18551), list);
    let cmd = format!("grep -Ev '{}' {} > {}", strn, list, list_tmp);
    run_cmd_str(cmd.as_str());
    let cmd = format!("mv {} {}", list_tmp, list);
    run_cmd_str(cmd.as_str());
}
pub(crate) fn sieve_list(data: String){
    let data = data.replace("sieve ", "");
    let (opts, data) = split_once(&data, " ");
    if opts == "none".to_string() || data == "none".to_string(){
        set_ask_user("example: sieve -Ei some\\shere", 5977871);
        return 
    }
    let found_files_path = format!("{}/found_files", get_tmp_dir(18441));
    let filter_file_path_tmp = format!("{}/filter.tmp", get_tmp_dir(18441));
    let filter_file_path = format!("{}/filter", get_tmp_dir(18441));
    let cmd = format!("echo '' > {}", filter_file_path_tmp);
    run_cmd_str(cmd.as_str());
    let cmd = format!("grep {} '{}' {} > {}", opts, data, found_files_path, filter_file_path_tmp);
    run_cmd_str(cmd.as_str());
    let cmd = format!("mv {} {}", filter_file_path_tmp, filter_file_path);
    run_cmd_str(cmd.as_str());
    let cmd = format!("ln -sf {} {}", filter_file_path, found_files_path);
    run_cmd_str(cmd.as_str());
    mark_front_lst("filter");
    fix_num_files(5977871);
}
pub(crate) fn show_ls(){
    unsafe{set_ls_as_front(); front_list_indx(crate::globs18::LS_);}
    crate::ps18::fix_num_files(-13972);
}
pub(crate) fn get_num_pg_4_main0() -> i64{
    let num_pg = read_file("main0.pg");
    match i64::from_str_radix(&num_pg, 10){
        Ok(num) => return num,
        _ => return 0
    };
}
pub(crate) fn set_valid_list_as_front(){
    let tmp_dir = get_tmp_dir(-6015);
    let active_lst = format!("{}/{}", &tmp_dir, read_file("front_list"));
    let front_list_link = format!("{}/found_files", &tmp_dir);
    let cmd = format!("ln -sf {} {}", active_lst, front_list_link);
    run_cmd_str(&cmd);
}
pub(crate) fn F1_key() -> String{
    let mut prnt: String = read_prnt();
   set_main0_as_front();
   crate::ps18::fix_num_files(-13971);
format!("go2 {}", read_file("main0.pg"))
}
pub(crate) fn F3_key() -> String{
    unsafe{set_ls_as_front(); front_list_indx(crate::globs18::LS_);}
    let mut prnt: String = read_prnt();
    let orig_path = get_path_from_strn(crate::cpy_str(&prnt));
    let mut ret_2_F1_key = || -> String{prnt = prnt.replace("/", ""); set_prnt(&prnt, -2317712); crate::C!(swtch_fn(0, "".to_string())); return F1_key()};
    let mut path = format!("{}/", match Path::new(&orig_path).parent(){
        Some(path) => path,
        _ => return ret_2_F1_key()
    }.to_str().unwrap());
    path = path.replace("//", "/");
    prnt = prnt.replace(&orig_path, &path);
    set_prnt(&prnt, -1405);
    /*let user_wrote_path = user_wrote_path();
    rm_file(&user_wrote_path);*/
    set_user_written_path_from_strn(path.to_string());
    prnt
}
pub(crate) fn Ins_key() -> String{
    let mut prnt: String = read_prnt();
    let path = get_path_from_strn(crate::cpy_str(&prnt));
    let mut file_indx = String::new();
    let spaces = repeat_char(63, " ");
    println!(" \rPlease, enter indx of dir/file name to autocomplete: {}", spaces);
    io::stdin().read_line(&mut file_indx).expect("Ins_key failed to read console");
    let file_indx = file_indx.as_str().substring(0, file_indx.len() -1);
    let mut err_msg = "".to_string();
    let mut handle_err =|e: std::num::ParseIntError| -> i64 {err_msg = format!("{:?}", e); -1i64};
    let file_indx = match i64::from_str_radix(&file_indx, 10){
        Ok(int) => int,
        Err(e) => handle_err(e)
    };
    if file_indx == -1i64{set_ask_user(&err_msg, -1); return "none done".to_string();}
    let mut file = get_item_from_front_list(file_indx, true);
    let is_dir = crate::Path::new(&file).is_dir();
    if is_dir {file.push('/');}
    prnt = prnt.replace(&path, &file);
    crate::set_prnt(&prnt, -1);
    prnt
}
pub(crate) fn Enter(){
    let mut mode = 0i64;
    unsafe{check_mode(&mut mode)}
    if mode == SWTCH_USER_WRITING_PATH{mode = SWTCH_RUN_VIEWER}
    unsafe {crate::swtch::swtch_fn(mode, "".to_string());}
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
pub fn add_2_front_list(val: &str, func_id: i64) -> String{
    let mut list_id: (i64, bool) = (1i64, false);
    for i in 0..1000_000{
        list_id = unsafe {front_list_indx(i64::MAX)};
        if list_id.1{break;}
    }
    if !list_id.1{set_ask_user("Can't access to Front list", func_id); return "!!no¡".to_string()}
    return unsafe{lists(val, list_id.0, 0, ADD)}
}
pub fn add_2_main0_list(val: &str) -> String{
    return unsafe{lists(val, MAIN0_, 0, ADD)}
}
pub fn len_of_main0_list() -> String{
    return unsafe{lists("", MAIN0_, 0, LEN)}
}
pub fn len_of_front_list() -> String{
      let mut list_id: (i64, bool) = (1i64, false);
    for i in 0..1000_000{
        list_id = unsafe {front_list_indx(i64::MAX)};
        if list_id.1{break;}
    }
    if !list_id.1{set_ask_user("Can't access to Front list", -1); return "!!no¡".to_string()}
    return unsafe{lists("", list_id.0, 0, LEN)}
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
      let mut list_id: (i64, bool) = (1i64, false);
    for i in 0..1000{
        list_id = unsafe {front_list_indx(i64::MAX)};
        if list_id.1{break;}
    }
    if !list_id.1{set_ask_user("Can't access to Front list", -1); return "!!no¡".to_string()}
    return unsafe{lists("", list_id.0, proper_indx.0, GET)}
}
pub fn set_main0_as_front(){mark_front_lst("main0"); unsafe{lists("", MAIN0_, 0, SET_FRONT_LIST);}}
pub fn set_ls_as_front() -> String{
      let mut list_id: (i64, bool) = (1i64, false);
    for i in 0..1000{
        list_id = unsafe {front_list_indx(LS_)};
        if list_id.1{break;}
    }
    if !list_id.1{set_ask_user("Can't access to Front list", -1); return "!!no¡".to_string()}
    mark_front_lst("ls");
    unsafe{lists("", LS_, 0, SET_FRONT_LIST); return "ok".to_string();}}
pub unsafe fn lists(val: &str, list: i64, indx: usize, op_code: i64) -> String{
static mut MAIN0: OnceCell<Vec<String>> = OnceCell::new();
static mut FRONT: OnceCell<&Vec<String>> = OnceCell::new();
//static mut LS: RwLock<Lazy<Vec<String>>> = RwLock::new(Lazy::new(||{vec!["".to_string()]})); // OnceCell<Vec<String>> = OnceCell::new();
if Some(MAIN0.get()) != None{
    let mut main0_vec: Vec<String> = Vec::new();
    MAIN0.set(main0_vec);
}
/*if Some(LS.get()) != None{
    let mut ls_vec: Vec<String> = Vec::new();
    LS.set(ls_vec);
}*/
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
       FRONT.take(); FRONT.take();
       FRONT.set(&MAIN0.get().unwrap());
       let main_path = get_tmp_dir(-13374);
       let main0_path = format!("{}/{}", &main_path, "main0");
       let found_files_path = format!("{}/{}", &main_path, "found_files");
       if !Path::new(&main0_path).exists(){
        let cmd = format!("touch -f {}", &main0_path);
        run_cmd_str(&cmd);
       }
       let cmd = format!("ln -sf {} {}", main0_path, found_files_path);
       run_cmd_str(&cmd);
       return "main0 gets set as front".to_string();
    }
}
if list == LS_ {
    if op_code == GET{
        let ret = ln_of_found_files(indx).0;
        return ret.to_string()
    }
    if op_code == ADD{
        /*LS.write().expect("Can't write into LS").push(val.to_string());
        let mut len = LS.read().unwrap().len() - 1;
        if len > 2{len -= 2;}
        set_ask_user(& LS.read().unwrap()[len], -1);*/
       return "dummy ls ADD".to_string()
    }
    if op_code == LEN{return ln_of_found_files(usize::MAX).1.to_string()}
    if op_code == SET_FRONT_LIST {
       let main_path = get_tmp_dir(-13314);
       let ls_path = format!("{}/{}", &main_path, "ls");
       let found_files_path = format!("{}/{}", &main_path, "found_files");
       let cmd = format!("touch -f {}", &ls_path);
       run_cmd_str(&cmd);
       let cmd = format!("ln -sf {} {}", ls_path, found_files_path);
       run_cmd_str(&cmd);
       return "ls gets set as front".to_string();
    }
}
if list == FRONT_ {
    if op_code == GET{let ret = ln_of_found_files(indx).0;
        return ret.to_string()}//return FRONT.get().unwrap()[indx].to_string()}
    if op_code == LEN{return ln_of_found_files(usize::MAX).1.to_string()}//return FRONT.get().unwrap().len().to_string()}
}
"wrong".to_string()
}