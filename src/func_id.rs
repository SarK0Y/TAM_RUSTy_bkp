pub const initSession: u64 = 1;
pub const find_files: u64 = 2;
pub const  check_substr: u64 = 3;
pub const  mk_cmd_file: u64 = 4;
pub const  run_cmd: u64 = 5;
pub fn get_func_name(func_id: u64) -> &'static str {
    //let max = u64::MAX;
    let ret = match func_id {
        initSession => "initSession",
        find_files => "find_files",
        check_substr => "check_substr",
        mk_cmd_file => "mk_cmd_file",
        run_cmd => "run_cmd",
        _ => "unknown func",
    };
    return ret;
}
