use once_cell::sync::OnceCell;
pub const MAIN0_: i64 =  1;
pub const FRONT_: i64 =  2;
pub const FILTERED_: i64 =  3;
pub const MERGE_: i64 =  4;
pub const LS: i64 =  5;
pub const ADD: i64 =  6;
pub const INS: i64 =  7;
pub const DEL: i64 =  8;
pub const GET: i64 =  9;
pub const LEN: i64 =  10;
pub fn add_2_main0_list(val: &str) -> String{
    return unsafe{lists(val, MAIN0_, 0, ADD)}
}
pub fn len_of_main0_list() -> String{
    return unsafe{lists("", MAIN0_, 0, LEN)}
}
pub unsafe fn lists(val: &str, list: i64, indx: usize, op_code: i64) -> String{
static mut MAIN0: OnceCell<Vec<String>> = OnceCell::new();
let mut main0_vec: Vec<String> = Vec::new();
if Some(MAIN0.get()) != None{
    MAIN0.set(main0_vec);
}
if list == MAIN0_ {
    if op_code == GET{
        let ret = &MAIN0.get().unwrap()[indx];
        return ret.to_string()
    }
    if op_code == ADD{
        MAIN0.get_mut().unwrap().push(val.to_string());
       return "ok".to_string()
    }
    if op_code == LEN{
       return MAIN0.get().unwrap().len().to_string()
       
    }
}
"wrong".to_string()
}