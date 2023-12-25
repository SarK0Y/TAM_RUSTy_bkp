#[macro_use]
#[path = "exts.rs"]
mod exts;
use exts::*;
core_use!();
pub(crate) fn initSession() -> bool{
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
unsafe{crate::page_struct(&mainpath, set(ps21::MAINPATH_), func_id);}
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
unsafe{crate::page_struct(&path_2_found_files_list, set(crate::TMP_DIR_), func_id);}
let path_2_found_files_list_dot = format!("{}/TAM_{}/.", path_2_shm, proper_timestamp);
let err_msg = format!("{} permission denied", path_2_found_files_list_dot);
let run_shell3 = Command::new("chmod").arg("700").arg(&path_2_found_files_list_dot).output().expect(&err_msg.bold().red());
if checkArg("-dbg"){println!("shell out = {:?}", run_shell3)};
let path_2_found_files_list = format!("{}/TAM_{}/found_files", path_2_shm, proper_timestamp);
let err_msg = format!("{} can't be created", path_2_found_files_list);
let run_shell4 = Command::new("touch").arg("-f").arg(&path_2_found_files_list).output().expect(&err_msg.bold().red());
if checkArg("-dbg"){println!("shell out = {:?}", run_shell4)};
unsafe{crate::page_struct(&path_2_found_files_list, set(crate::FOUND_FILES_), func_id);
       crate::page_struct("empty", set(crate::KONSOLE_TITLE_), func_id);}
    crate::globs18::set_main0_as_front();
    crate::set_prnt("test typing", func_id);
    return true;
}
pub(crate) fn errMsg_dbg(msg: &str, val_func_id: i64, delay: f64) {
    if !checkArg("-dbg") {return}
    if delay == -1.0{
        let msg = format!("{} said: {}", crate::func_id18::get_func_name(val_func_id), msg);
        println!("{}", msg.bold().red());}
}
pub(crate) fn checkArg(key: &str) -> bool{
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
pub(crate) fn set(item: i64) -> i64{
    return item * -1;
}
pub(crate) fn this_item_takes_global_val(id: i64) -> i64 {
    return set(id);
}

pub(crate) struct ret0 {
   pub s: [char; 512],
   pub res: bool
}
pub(crate) fn escape_symbs(str0: &String) -> String{
    let strr = str0.as_str();
    let strr = strr.replace("-", r"\-");
    return strr.to_string();
}

pub(crate) fn check_substr(orig: &String, probe: &str, start_from: usize) -> bool{
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

fn end_termios(termios: &Termios){
  let res = match tcsetattr(0, TCSANOW, &termios){
        Err(e) => {format!("{}", e)},
        Ok(len) => {format!("{:?}", len)}
    };
    io::stdout().flush().unwrap();
}
pub(crate) fn getkey() -> String{
let mut Key: String ="".to_string();
 let mut stdin = io::stdin();
    let stdin_fd = 0;
    let mut stdout = io::stdout();
    let mut stdin_buf: [u8; 6] =[0;6];
    let termios = Termios::from_fd(stdin_fd).unwrap();
    let mut new_termios = termios.clone();
    stdout.lock().flush().unwrap(); 
    new_termios.c_lflag &= !(ICANON | ECHO); 
    let enter = ||
{
    let enter: [u8; 1] = [13; 1];
    let mut writeIn_stdin = unsafe {File::from_raw_fd(0/*stdin*/)};
    writeIn_stdin.write(&enter);
    println!("gotta enter");
};
loop {
    let res = match tcsetattr(stdin_fd, TCSANOW, &new_termios){
        Err(e) => {format!("{}", e)},
        Ok(len) => {format!("kkkkkkkkkkk {:#?}", len)}
    };
    let red_stdin = stdin.read(&mut stdin_buf);
    stdout.lock().flush().unwrap();
    end_termios(&termios);
    if crate::dirty!() {println!("len of red {:?}", red_stdin.unwrap());}
    let str0 = match str::from_utf8(&stdin_buf){
        Ok(s) => s,
        _ => ""
    };
    let msg = format!("getch {} {:?}", str0, stdin_buf);
    achtung(&msg);
    if stdin_buf != [0; 6]{
        let mut i = 0;
        loop{
            let ch = match str0.chars().nth(i){
                Some(ch) => ch,
                _ => return Key
            };
        Key.push(ch);
        i += 1;}}
}
Key
}
pub(crate) fn getch() -> char{
let mut ch: char ='\0';
 let mut stdin = io::stdin();
    let stdin_fd = 0;
    let mut stdout = io::stdout();
    let mut stdin_buf: [u8; 6] =[0;6];
    let termios = Termios::from_fd(stdin_fd).unwrap();
    let mut new_termios = termios.clone();
    stdout.lock().flush().unwrap(); 
    new_termios.c_lflag &= !(ICANON | ECHO); 
    let enter = ||
{
    let enter: [u8; 1] = [13; 1];
    let mut writeIn_stdin = unsafe {File::from_raw_fd(0/*stdin*/)};
    writeIn_stdin.write(&enter);
    println!("gotta enter");
};
loop {
    let res = match tcsetattr(stdin_fd, TCSANOW, &new_termios){
        Err(e) => {format!("{}", e)},
        Ok(len) => {format!("kkkkkkkkkkk {:#?}", len)}
    };
    let red_stdin = stdin.read(&mut stdin_buf);
    stdout.lock().flush().unwrap();
    end_termios(&termios);
    if crate::dirty!() {println!("len of red {:?}", red_stdin.unwrap());}
    let str0 = match str::from_utf8(&stdin_buf){
        Ok(s) => s,
        _ => ""
    };
    let msg = format!("getch {} {:?}", str0, stdin_buf);
    achtung(&msg);
    if stdin_buf != [0; 6]{return str0.chars().nth(0).unwrap()}
}
ch
}
pub(crate) fn achtung(msg: &str){
    if !crate::checkArg("-dbg") | !crate::checkArg("-use-achtung"){return;}
    let msg = format!("notify-send '{}'", msg);
    crate::run_cmd_str(&msg);
}
pub(crate)
fn check_substring(orig: String, probe: String, start_from: usize) -> bool{
    let substr: &str= &orig.as_str();
    let substr = substr.substring(start_from, probe.len() - 1).to_string();
     if probe.ne(&substr){
        return false;
     }
     return true;
}
pub(crate) fn put_in_name() -> String{
    let mut ret: String = String::new();
    let len_of_cmd_line = env::args().len() -1;
    let args: Vec<String> = env::args().collect();
    let i: i64 = 0;
    for i in 0..len_of_cmd_line{
        if args[i] == "-in_name".to_string() || args[i] == "-in-name".to_string(){
            let cmd = format!("|{}", crate::form_grep_cmd(&args[i+1]));
            ret.push_str(&cmd);
        }
    }
    return ret;
}