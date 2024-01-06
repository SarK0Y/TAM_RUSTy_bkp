use crate::exts::update_uses;
use self::func_id17::find_files;
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
    main_update();
println!("len of main0 list {}", globs17::len_of_main0_list());
    let builder = thread::Builder::new().stack_size(8 * 1024 * 1024).name("manage_page".to_string());
let handler = builder.spawn(|| {
let mut ps__: crate::_page_struct = crate::init_page_struct();
ps__.num_cols = i64::MAX; ps__.num_page = i64::MAX; ps__.num_rows = i64::MAX;
crate::manage_pages(ps__);
println!("stop manage_page");
}).unwrap();
    handler.join().unwrap();
    println!("len of main0 list {}", globs17::len_of_main0_list());
}