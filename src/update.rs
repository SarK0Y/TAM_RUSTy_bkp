use crate::{exts::update_uses, globs18::{set_main0_as_front, MAIN0_}, swtch::{front_list_indx, swtch_fn, SWTCH_USER_WRITING_PATH}, read_midway_data};
use self::{func_id17::find_files, globs17::set_ls_as_front};
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
        spawn(move||{
            println!("spawn find files");
            crate::find_files(path.as_str(), in_name, "");
        });
        spawn(||{
            println!("spawn midway data");
            crate::read_midway_data()
        });
    }

}
pub(crate) fn prime(){
    crate::initSession();
    unsafe{front_list_indx(MAIN0_)};
    unsafe{set_main0_as_front()};
    main_update();
println!("len of main0 list {}", globs17::len_of_main0_list());
    let builder = thread::Builder::new().stack_size(8 * 1024 * 1024).name("manage_page".to_string());
let handler = builder.spawn(|| {
let mut ps__: crate::_page_struct = crate::init_page_struct();
ps__.num_cols = i64::MAX; ps__.num_page = i64::MAX; ps__.num_rows = i64::MAX;
unsafe {crate::swtch::swtch_ps(0, Some(ps__));}
crate::manage_pages();
println!("stop manage_page");
}).unwrap();
    handler.join().unwrap();
    println!("len of main0 list {}", globs17::len_of_main0_list());
}
pub(crate) fn update_dir_list(dir: &str, opts: &str, no_grep: bool){
    let head = Path::new(dir).file_stem().unwrap().to_str().unwrap();
    let tail = Path::new(dir).parent().unwrap().to_str().unwrap();
    let mut cmd = format!("find -L {} {}|grep -Ei '{}", tail, opts, head);
    if no_grep{cmd = format!("find -L {}/{}", tail, head);}
    crate::custom_cmd_4_find_files(cmd);
    unsafe{set_ls_as_front(); front_list_indx(crate::globs18::LS_);}
    read_midway_data();

}
pub(crate) fn lets_write_path(key: String){
    unsafe {swtch_fn(SWTCH_USER_WRITING_PATH, key)};
    
}