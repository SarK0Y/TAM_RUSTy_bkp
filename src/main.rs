#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(while_true)]
mod exts;
use exts::*;
use globs::add_2_main0_list;

use crate::func_id::{build_page, init_page_struct};
use_all!();
fn set(item: i64) -> i64{
    return item * -1;
}
fn this_item_takes_global_val(id: i64) -> i64 {
    return set(id);
}

struct ret0 {
    s: [char; 512],
    res: bool
}
fn escape_symbs(str0: &String) -> String{
    let strr = str0.as_str();
    let strr = strr.replace("-", r"\-");
    return strr.to_string();
}
fn initSession() -> bool{
    let func_id = 1;
    let run_command = Command::new("bash").arg("-c").arg("cd ~;pwd")
    .output()
    .expect("something wrong with cd command");
errMsg_dbg(from_utf8(&run_command.stderr).unwrap(), func_id, -1.0);  
if !run_command.status.success(){
    io::stdout().write_all(&run_command.stdout).unwrap();
    io::stderr().write_all(&run_command.stderr).unwrap();
    return false;
}
let mut proper_output: Vec<u8> = run_command.stdout;
let last: usize = proper_output.len() - 1;
let _ =proper_output.pop();
errMsg_dbg(from_utf8(&proper_output).unwrap(), func_id, -1.0);  
let timestamp = Local::now();
let proper_timestamp = format!("{}", timestamp.format("%Y-%mm-%dd_%H-%M-%S_%f"));
let mainpath: String = format!("{}/.TAM_SESSIONS/{proper_timestamp}/", from_utf8(&proper_output).unwrap().to_string());
//let mainpath = escape_symbs(&mainpath);
errMsg_dbg(&mainpath, func_id, -1.0);
let run_shell = Command::new("mkdir").arg("-p").arg(&mainpath).output();
if checkArg("-dbg"){println!("shell out = {:?}", run_shell)};
if !Path::new(&mainpath).exists(){
   let res = fs::create_dir(&mainpath);
   println!("res = {:?}", res);
}
if !Path::new(&mainpath).exists(){
    println!("{mainpath} cannot be made");
    return false;
}
unsafe{page_struct(&mainpath, set(ps::MAINPATH_), func_id);}
let mut path_2_shm = "";
while true{
    if Path::new("/dev/shm").exists(){path_2_shm = "/dev/shm"; break;}
    if Path::new("/run/shm").exists(){path_2_shm = "/run/shm"; break;}
    if Path::new("/sys/shm").exists(){path_2_shm = "/sys/shm"; break;}
    if Path::new("/proc/shm").exists(){path_2_shm = "/proc/shm"; break;}
    if Path::new("/opt/shm").exists(){path_2_shm = "/opt/shm"; break;}
    if Path::new("/var/shm").exists(){path_2_shm = "/var/shm"; break;}
    panic!("no way for shared memory: device /dev/shm and its analogs don't exist or maybe Your system ain't common Linux");
}
let path_2_found_files_list = format!("{}/TAM_{}", path_2_shm, proper_timestamp);
let err_msg = format!("{} wasn't created", path_2_found_files_list);
let run_shell1 = Command::new("mkdir").arg("-p").arg(&path_2_found_files_list).output().expect(&err_msg.bold().red());
if checkArg("-dbg"){println!("shell out = {:?}", run_shell1)};
let err_msg = format!("{} permission denied", path_2_found_files_list);
let run_shell2 = Command::new("chmod").arg("700").arg(&path_2_found_files_list).output().expect(&err_msg.bold().red());
if checkArg("-dbg"){println!("shell out = {:?}", run_shell2)};
unsafe{page_struct(&path_2_found_files_list, set(ps::TMP_DIR_), func_id);}
let path_2_found_files_list_dot = format!("{}/TAM_{}/.", path_2_shm, proper_timestamp);
let err_msg = format!("{} permission denied", path_2_found_files_list_dot);
let run_shell3 = Command::new("chmod").arg("700").arg(&path_2_found_files_list_dot).output().expect(&err_msg.bold().red());
if checkArg("-dbg"){println!("shell out = {:?}", run_shell3)};
let path_2_found_files_list = format!("{}/TAM_{}/found_files", path_2_shm, proper_timestamp);
let err_msg = format!("{} can't be created", path_2_found_files_list);
let run_shell4 = Command::new("touch").arg("-f").arg(&path_2_found_files_list).output().expect(&err_msg.bold().red());
if checkArg("-dbg"){println!("shell out = {:?}", run_shell4)};
unsafe{page_struct(&path_2_found_files_list, set(ps::FOUND_FILES_), func_id);
       page_struct("empty", set(ps::KONSOLE_TITLE_), func_id);}
    set_main0_as_front();
    return true;
}
fn errMsg_dbg(msg: &str, val_func_id: i64, delay: f64) {
    if delay == -1.0{
        let msg = format!("{} said: {}", func_id::get_func_name(val_func_id), msg);
        println!("{}", msg.bold().red());}
}
fn check_substr(orig: &String, probe: &str, start_from: usize) -> bool{
    let func_id = 3;
    let probe: String = String::from(probe.to_string());
    let substr: &str= &orig.as_str();
    let substr = substr.substring(start_from, probe.len()).to_string();
    errMsg_dbg(&substr, func_id, -1.0);
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
    let len_of_cmd_line = env::args().len() -1;
    let args: Vec<String> = env::args().collect();
    let i: i64 = 0;
    for i in 0..len_of_cmd_line{
        if args[i] == "-in_name".to_string() || args[i] == "-in-name".to_string(){
            let cmd = format!("|{}", form_grep_cmd(&args[i+1]));
            ret.push_str(&cmd);
        }
    }
    return ret;
}
fn split_once(in_string: &str, delim: &str) -> (String, String) {
let mut splitter = in_string.splitn(2, delim);
let first = splitter.next().unwrap();
let second = splitter.next().unwrap();
return  (first.to_string(), second.to_string());
}
fn form_grep_cmd(in_name: &String) -> String{
    let mut ret: String = String::new();
    ret.push_str("grep ");
    let split = "go go";
    if check_substr(in_name, "pass==", 0){
     //  let mut in_name = in_name.as_str();
       let in_name = in_name.replace("pass==", "");
       let (opts, name) = split_once(&in_name.as_str(), " ");
       let val4grep = format!("{} '{}'", opts, name);
       ret.push_str(&val4grep);
    }else{let val4grep = format!("'{}'", &in_name); ret.push_str(&val4grep)}
    return ret;
}
fn mk_cmd_file(cmd: String) -> String{
let func_id = 4;
let path_2_cmd = format!("{}/cmd.sh", unsafe{page_struct("", ps::TMP_DIR_, func_id).str_});
let err_msg = format!("failed create {}", &path_2_cmd);
let mut make_cmd_file = File::create(&path_2_cmd).expect(&err_msg.bold().red());
errMsg_dbg(&path_2_cmd, func_id, -1.0);
make_cmd_file.write_all(&cmd.as_bytes());
Command::new("chmod").arg("700").arg(&path_2_cmd).output().expect("");
errMsg_dbg(&cmd, func_id, -1.0);
path_2_cmd.to_string()
}
fn run_cmd(cmd: String) -> bool{
let func_id = 5;
let fstdout: String; 
let path_2_cmd = mk_cmd_file(cmd);
let mut stderr_path = "stderr".to_string();
stderr_path = format!("{}stderr", unsafe{page_struct("", ps::MAINPATH_, -1).str_});
let path_2_list_of_found_files = format!("{}", unsafe{page_struct("", ps::FOUND_FILES_, -1).str_});
fstdout = String::from(path_2_list_of_found_files); 
errMsg_dbg(&stderr_path, func_id, -1.0);
errMsg_dbg(&fstdout, func_id, -1.0);
let fstderr = File::create(stderr_path).unwrap();
let fstdout0 = File::create(fstdout).unwrap();
//let mut fstdout0 = io::BufReader::new(fstdout0);
//errMsg_dbg(&in_name, func_id, -1.0);
let run_command = Command::new("bash").arg("-c").arg(path_2_cmd)//.arg(";echo").arg(stopCode)
//let run_command = Command::new(cmd)
    .stdout(fstdout0)
    .stderr(fstderr)
    .output()
    .expect("can't run command in run_cmd");
if run_command.status.success(){
    io::stdout().write_all(&run_command.stdout).unwrap();
    io::stderr().write_all(&run_command.stderr).unwrap();
    return false;
}
true
}
fn read_midway_data() -> bool{
    let filename = format!("{}/found_files", unsafe{page_struct("", ps::TMP_DIR_, -1).str_});
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for (/*indx*/_, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let ret = add_2_main0_list(&line); 
        if ret != "ok".to_string(){return false}
    }
    true
}
#[inline(always)]
fn find_files(path: &str, mut in_name: String, path_2_tmp_file: &str) -> bool{
let func_id: i64 = 2;
let mut list_of_found_files: Vec<String> = vec![]; 
if in_name.len() == 0{in_name = put_in_name();}
else{in_name = format!("|{}", form_grep_cmd(&in_name));}
let stopCode: String = unsafe {page_struct("", ps::STOP_CODE_,-1).str_};
let cmd: String = format!("#!/bin/bash\nfind -L '{path}' -type f{in_name};echo '\n{stopCode}\n';notify-send 'hi the'; touch -f ./tst");
run_cmd(cmd);
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
initSession();
let out: ret0 = get_arg_in_cmd("-tst");
let out1: ret0 = get_arg_in_cmd("-тст");
println!("argument from cmd (-tst) {}", String::from_iter(out.s));
println!("argument from cmd (-тст) {}", String::from_iter(out1.s));
unsafe {println!("get stop code {}", page_struct("", 1, 0).str_);}
//unsafe {println!("set stop code {}", page_struct("777", set(1), 0).str_);}
unsafe {println!("get stop code {}", page_struct("", 1, 0).str_);}
let mut path: String = String::from("~/");
if checkArg("-path"){path = String::from_iter(get_arg_in_cmd("-path").s);}
if checkArg("-path0"){path = String::from_iter(get_arg_in_cmd("-path0").s);}
println!("run find_files status {}", find_files(path.as_str(), "".to_string(), ""));
read_midway_data();
println!("len of main0 list {}", len_of_main0_list());
let ps__ = ps::init_page_struct();
pg::build_page(ps__);
return;
}
