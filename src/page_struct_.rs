mod exts;
use exts::*;
use globs::add_2_main0_list;
use crate::set;
page_struct_uses!();
pub const STOP_CODE_: i64 = 1;
pub const KONSOLE_TITLE_: i64 = 2;
pub const LEFT_SHIFT_4_CUR_: i64 = 3;
pub const CUR_CUR_POS_: i64 = 4;
pub const NUM_PAGE_: i64 = 5;
pub const NUM_COLS_: i64 = 6;
pub const COL_WIDTH_: i64 = 7;
pub const NUM_ROWS_: i64 = 8;
pub const NUM_SPACES_: i64 = 9;
pub const NUM_FILES_: i64 = 10;
pub const COUNT_PAGES_: i64 = 11;
pub const NEWS_BAR_: i64 = 12;
pub const ASK_USER_: i64 = 13;
pub const dontDelFromTableJustMark_: i64 = 14;
pub const RUNNING_: i64 = 15;
pub const VIEWER_: i64 = 16;
pub const MODE2RUN_: i64 = 17;
pub const PRNT_: i64 = 18;
pub const PROMPT_: i64 = -18;
pub const FULL_PATH_: i64 = 19;
pub const MAINPATH_: i64 = 20;
pub const FOUND_FILES_: i64 = 21;
pub const TMP_DIR_: i64 = 22;
pub (crate) struct page_struct_ret{
   pub str_: String,
   pub int: i64
}
pub(crate) struct child2run{
   pub running: i64,
   pub viewer: i64,
   pub mode2run: i64, // 0 == internal, 1 == extrn
   pub full_path: String
}
pub(crate) struct page_struct{
   pub stop_code: String,
   pub prnt: String,
   pub prompt: String,
   pub konsole_title: String,
   pub left_shift_4_cur: i64,
   pub cur_cur_pos: i64,
   pub num_page: i64,
   pub num_cols: i64,
   pub col_width: i64,
   pub num_rows: i64,
   pub num_spaces: i64,
   pub num_files: i64,
   pub count_pages: i64,
   pub news_bar: String,
   pub ask_User: String,
   pub c2r: child2run
}
pub(crate) fn init_page_struct() -> page_struct{
   let func_id = func_id::init_page_struct;
   let stop_code = unsafe {page_struct("", STOP_CODE_, func_id.to_i64().unwrap()).str_};
   let konsole_title = unsafe {page_struct("", KONSOLE_TITLE_, func_id.to_i64().unwrap()).str_};
   let left_shift_4_cur =0i64; let cur_cur_pos = 0i64; let num_page = 0i64; let num_cols = 3i64;
   let col_width = 70i64; let num_rows = 9i64; let num_spaces = 0i64; let num_files = 0i64;
   let count_pages = 0i64;
   let news_bar = unsafe {page_struct("", NEWS_BAR_, func_id.to_i64().unwrap()).str_};
   let ask_user = unsafe {page_struct("", ASK_USER_, func_id.to_i64().unwrap()).str_};
   let prnt = unsafe {page_struct("", PRNT_, func_id.to_i64().unwrap()).str_};
   let prompt = unsafe {page_struct("", PROMPT_, func_id.to_i64().unwrap()).str_};
   let full_path = unsafe {page_struct("", FULL_PATH_, func_id.to_i64().unwrap()).str_};
   let running = 0i64; let viewer = 0i64; let mode2run = 1i64;
   let c2r = child2run{running: running, viewer: viewer, mode2run: mode2run, full_path: full_path};
   let ps_new = page_struct{stop_code: stop_code, prnt: prnt, prompt: prompt, konsole_title: konsole_title, left_shift_4_cur: left_shift_4_cur,
       cur_cur_pos: cur_cur_pos, num_page: num_page, num_cols: num_cols, col_width: col_width, num_rows: num_rows, num_spaces: num_spaces,
   num_files: num_files, count_pages: count_pages, news_bar: news_bar, ask_User: ask_user, c2r: c2r};
   ps_new
}
pub(crate) fn get_num_cols(func_id: i64) -> i64{return unsafe{page_struct_int(0, NUM_COLS_, func_id)}}
pub(crate) fn set_num_cols(val: i64, func_id: i64) -> i64{return unsafe{page_struct_int(val, set(NUM_COLS_), func_id)}}
pub(crate) fn get_num_page(func_id: i64) -> i64{return unsafe{page_struct_int(0, NUM_PAGE_, func_id)}}
pub(crate) fn get_num_pages(func_id: i64) -> i64{return unsafe{page_struct_int(0, COUNT_PAGES_, func_id)}}
pub(crate) fn get_num_files(func_id: i64) -> i64{return unsafe{page_struct_int(0, NUM_FILES_, func_id)}}
pub(crate) fn get_num_rows(func_id: i64) -> i64{return unsafe{page_struct_int(0, NUM_ROWS_, func_id)}}
pub(crate) fn set_num_rows(val: i64, func_id: i64) -> i64{return unsafe{page_struct_int(val, set(NUM_ROWS_), func_id)}}
pub(crate) unsafe fn page_struct_int(val: i64, val_id: i64, caller_id: i64) -> i64{
    static mut LEFT_SHIFT_4_CUR: i64 = 0; // 3
    static mut CUR_CUR_POS: i64 = 0; //4
    static mut NUM_PAGE: i64 = 0; //5
    static mut NUM_COLS: i64 = 3; //6
    static mut COL_WIDTH: i64 = 70; //7
    static mut NUM_ROWS: i64 = 9; //8
    static mut NUM_SPACES: i64 = 4; //9
    static mut NUM_FILES: i64 = 0; //10
    static mut COUNT_PAGES: i64 = 0; //11
    if val_id == NUM_PAGE_ {return NUM_PAGE}
    if val_id == set(NUM_PAGE_) {NUM_PAGE = val; return val;}
    if val_id == NUM_COLS_ {return NUM_COLS}
    if val_id == set(NUM_COLS_) {NUM_COLS = val; return val;}
    if val_id == NUM_ROWS_ {return NUM_ROWS}
    if val_id == set(NUM_ROWS_) {NUM_ROWS = val; return val}
    if val_id == NUM_FILES_ {return NUM_FILES}
    if val_id == set(NUM_FILES_) {NUM_FILES = val; return val;}
    if val_id ==  COUNT_PAGES_ {return COUNT_PAGES}
    if val_id == set(COUNT_PAGES_) {COUNT_PAGES = val; return val;}
 return -1;  
}
pub(crate) unsafe fn page_struct(val: &str, id_of_val: i64, id_of_caller: i64) -> page_struct_ret
{
    let mut int_:i64 = 0; let mut str__ = String::new();
    let mut ps_ret = page_struct_ret{str_: str__, int: int_};
    let vec_str: Vec<String> = vec![val.to_string()];
    static mut STOP_CODE: OnceCell<String> = OnceCell::new(); // 1
    static mut KONSOLE_TITLE: OnceCell<String> = OnceCell::new(); // 2
    static mut LEFT_SHIFT_4_CUR: i64 = 0; // 3
    static mut CUR_CUR_POS: i64 = 0; //4
    static mut NUM_PAGE: i64 = 0; //5
    static mut NUM_COLS: i64 = 3; //6
    static mut COL_WIDTH: i64 = 70; //7
    static mut NUM_ROWS: i64 = 9; //8
    static mut NUM_SPACES: i64 = 4; //9
    static mut NUM_FILES: i64 = 0; //10
    static mut COUNT_PAGES: i64 = 0; //11
    static mut NEWS_BAR: OnceCell<String> = OnceCell::new(); //12
    static mut ASK_USER: OnceCell<String> = OnceCell::new(); //13
    static mut dontDelFromTableJustMark: bool = true; //14
    static mut RUNNING: OnceCell<Vec<Command>> = OnceCell::new(); //15
    static mut VIEWER: i64 = 0; //16
    static mut MODE2RUN: OnceCell<(String, String)> = OnceCell::new(); //17
    static mut PRNT: OnceCell<String> = OnceCell::new(); //18
    static mut FULL_PATH: OnceCell<String> = OnceCell::new(); //19
    static mut MAINPATH: OnceCell<String> = OnceCell::new(); //20
    static mut FOUND_FILES: OnceCell<String> = OnceCell::new(); //21
    static mut TMP_DIR: OnceCell<String> = OnceCell::new(); //22
    //let mut tst: String = "5".to_string();
    let _ = STOP_CODE.set("∇".to_string());
    //let fn_ptr_get_string: fn(&str) -> String = get_string;
    if id_of_val == NUM_PAGE_ {ps_ret.int = NUM_PAGE; return ps_ret;}
    if id_of_val == set(NUM_PAGE_) {NUM_PAGE = i64::from_str_radix(val, 10).expect("failed number of a page"); return ps_ret;}
    if id_of_val == NUM_COLS_ {ps_ret.int = NUM_COLS; return ps_ret;}
    if id_of_val == set(NUM_COLS_) {NUM_COLS = i64::from_str_radix(val, 10).expect("failed number of columns"); return ps_ret;}
    if id_of_val == NUM_ROWS_ {ps_ret.int = NUM_ROWS; return ps_ret;}
    if id_of_val == set(NUM_ROWS_) {NUM_ROWS = i64::from_str_radix(val, 10).expect("failed number of rows"); return ps_ret;}
    if id_of_val == NUM_FILES_ {ps_ret.int = NUM_FILES; return ps_ret;}
    if id_of_val == set(NUM_FILES_) {NUM_FILES = i64::from_str_radix(val, 10).expect("failed number of files"); return ps_ret;}
    if id_of_val ==  COUNT_PAGES_ {ps_ret.int = COUNT_PAGES; return ps_ret;}
    if id_of_val == set(COUNT_PAGES_) {COUNT_PAGES = i64::from_str_radix(val, 10).expect("failed number of pages"); return ps_ret;}
    if id_of_val == STOP_CODE_ {ps_ret.str_ =STOP_CODE.get().unwrap().to_string(); return ps_ret;}
    if id_of_val == set(STOP_CODE_) {STOP_CODE.take(); let _ = STOP_CODE.set(val.to_string()); ps_ret.str_= "ok".to_string(); return ps_ret;}
    if id_of_val == MAINPATH_ {if MAINPATH.get() != None{ps_ret.str_ = MAINPATH.get().unwrap().to_string(); return ps_ret;}}
    if id_of_val == set(MAINPATH_) {MAINPATH.take(); let _ = MAINPATH.set(val.to_string());  ps_ret.str_= "ok".to_string(); return ps_ret;}
    if id_of_val == FOUND_FILES_ {ps_ret.str_ = FOUND_FILES.get().unwrap().to_string(); return ps_ret;}
    if id_of_val == set(FOUND_FILES_) {FOUND_FILES.take(); let _ = FOUND_FILES.set(val.to_string());  ps_ret.str_= "ok".to_string(); return ps_ret;}
    if id_of_val == TMP_DIR_ {ps_ret.str_ = TMP_DIR.get().unwrap().to_string(); return ps_ret;}
    if id_of_val == set(TMP_DIR_) {TMP_DIR.take(); let _ = TMP_DIR.set(val.to_string()); ps_ret.str_= "ok".to_string(); return ps_ret;}
    if id_of_val == KONSOLE_TITLE_ {ps_ret.str_ =KONSOLE_TITLE.get().unwrap().to_string(); return ps_ret;}
    if id_of_val == set(KONSOLE_TITLE_) {KONSOLE_TITLE.take(); let _ = KONSOLE_TITLE.set(val.to_string()); ps_ret.str_= "ok".to_string(); return ps_ret;}
     ps_ret.str_= "none".to_string(); return ps_ret;
}