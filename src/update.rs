use crate::{exts::update_uses, globs18::{set_main0_as_front, MAIN0_}, swtch::{front_list_indx, swtch_fn, SWTCH_USER_WRITING_PATH}, read_midway_data, complete_path, save_file, get_path_from_prnt, drop_ls_mode, from_ls_2_front, popup_msg};
use self::{func_id17::{find_files, read_midway_data_}, globs17::{set_ls_as_front, take_list_adr, len_of_front_list_wc, len_of_main0_list}};
update_uses!();
pub(crate) fn main_update(){
    let func_id = crate::func_id18::main_update;
    let mut no_path =true;
    let mut path: String ="".to_string();
    let mut in_name: String = "".to_string();
    if  crate::checkArg("-find_files") ||  crate::checkArg("-find-files"){
        if  crate::checkArg("-path"){path = get_arg_in_cmd("-path").s.iter().collect(); no_path = false;}
        if  crate::checkArg("-path0"){path = get_arg_in_cmd("-path0").s.iter().collect(); no_path = false;}
        if  crate::checkArg("-in_name"){in_name = get_arg_in_cmd("-in_name").s.iter().collect();}
        if  crate::checkArg("-in-name"){in_name = get_arg_in_cmd("-in-name").s.iter().collect();}
        if no_path {panic!("No path was provided: set flag '-path' or '-path0");}
        if  crate::checkArg("-rows"){let val: i64 = i64::from_str_radix(String::from_iter(get_arg_in_cmd("-rows").s).as_str(), 10).expect(
            "set number of rows as an integer: '-rows 9'"
        ); crate::set_num_rows(val, func_id);}
        if  crate::checkArg("-cols"){let val: i64 = i64::from_str_radix(String::from_iter(get_arg_in_cmd("-cols").s).as_str(), 10).expect(
            "set number of columns as an integer: '-cols 3'"
        ); ps0::set_num_cols(val, func_id);}
        let thr_midway = thread::Builder::new().stack_size(2 * 1024 * 1024).name("read_midway".to_string());
        let thr_find_files = thread::Builder::new().stack_size(2 * 1024 * 1024).name("find_files".to_string());
        thr_find_files.spawn(move||{
            println!("spawn find files");
            crate::find_files(path.as_str(), in_name, "");
            println!("exit find files");
        });
        thr_midway.spawn(||{
            println!("spawn midway data");
            crate::read_midway_data();
            println!("exit midway data");
        });
    }

}
pub(crate) fn prime(){
    crate::initSession();
    C!(front_list_indx(MAIN0_));
    C!(set_main0_as_front());
    main_update();
println!("len of main0 list {}", globs17::len_of_main0_list());
    let builder = thread::Builder::new().stack_size(8 * 1024 * 1024).name("manage_page".to_string());
let handler = builder.spawn(|| {
let mut ps__: crate::_page_struct = crate::init_page_struct();
ps__.num_cols = i64::MAX; ps__.num_page = i64::MAX; ps__.num_rows = i64::MAX;
C_!(crate::swtch::swtch_ps(0, Some(ps__)););
crate::manage_pages();
println!("stop manage_page");
}).unwrap();
background_fixing();
    handler.join().unwrap();
    println!("len of main0 list {}", globs17::len_of_main0_list());
}
pub(crate) fn update_dir_list(dir: &str, opts: &str, no_grep: bool){
    let mut head = String::new();
    let mut tail = String::new();
    let os_str = OsStr::new("");
    let mut dir_len = dir.chars().count();
    if dir_len ==0 {return;}
    if dir.chars().nth(dir_len -1).expect("update_dir_list failed to get file name") != '/'{
    head = match Path::new(dir).file_name(){
        Some(p) => p,
        _ => os_str
    }.to_str().unwrap().to_string();
    tail = match Path::new(dir).parent(){
        Some(p) => p,
        _ => Path::new("/")
    }.to_str().unwrap().to_string();
    
} else {
    head = "".to_string();
    tail = dir.substring(0, dir_len).to_string();
}
    let mut cmd = format!("find -L {} {}|grep -Ei '{}'", tail, opts, head);
    if no_grep{cmd = format!("find -L {}/{}", tail, head);}
    crate::custom_cmd_4_find_files(cmd);
    crate::read_midway_data_4_ls();
}
pub(crate) fn lets_write_path(key: String){
    C_!(set_ls_as_front(); front_list_indx(crate::globs18::LS_););
    let mode: i64 = crate::swtch::SWTCH_USER_WRITING_PATH;
    if mode < 0{return;}
    C!(swtch_fn(mode, key));

}
pub(crate) fn background_fixing(){
    popup_msg("tst0000000");
        
 //loop {
let builder = thread::Builder::new().stack_size(2 * 1024 * 1024).name("background_fixing".to_string());
 builder.spawn(|| {
background_fixing_()
});
 //}
}
fn background_fixing_(){
    let mut check_main0_len = String::new();
let mut drop_ls = true;
let ls_mode = take_list_adr("ls.mode");
loop {
     thread::sleep(Duration::from_millis(5000));
     let main0_len = len_of_main0_list();
     if check_main0_len != main0_len{
        check_main0_len = main0_len;
        save_file(crate::cpy_str(&check_main0_len), "main0.len".to_string());
        //else{drop_ls = !drop_ls}
     }
     let check_ls_mode = get_path_from_prnt();
     if check_ls_mode == ""{from_ls_2_front(ls_mode.clone());}
     save_file(check_ls_mode, "dbg_ls.mode".to_string());
}
}