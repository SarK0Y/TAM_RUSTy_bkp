pub const initSession: i64 = 1;
pub const find_files: i64 = 2;
pub const  check_substr: i64 = 3;
pub const  mk_cmd_file: i64 = 4;
pub const  run_cmd: i64 = 5;
pub const  build_page: i64 = 6;
pub const  init_page_struct: i64 = 7;
pub fn get_func_name(func_id: i64) -> &'static str {
    //let max = i64::MAX;
    let ret = match func_id {
        initSession => "initSession",
        find_files => "find_files",
        check_substr => "check_substr",
        mk_cmd_file => "mk_cmd_file",
        run_cmd => "run_cmd",
        build_page => "build_page",
        init_page_struct => "init_page_struct",
        _ => "unknown func",
    };
    return ret;
}
