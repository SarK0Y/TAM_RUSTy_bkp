use cli_table::TableStruct;

use crate::{exts::pg_uses, ps18::{set_prnt, get_cur_cur_pos, set_prompt, get_prnt, shift_cursor_of_prnt, set_full_path, set_ask_user, get_col_width, where_is_last_pg, get_num_files, child2run}, core18::{achtung, errMsg_dbg, ins_newlines, checkArg, popup_msg, calc_num_files_up2_cur_pg}, globs18::{ins_last_char_to_string1_from_string1, rm_char_from_string, ins_last_char_to_string1_from_string1_ptr, len_of_front_list}, split_once, swtch::{run_viewer, swtch_fn, local_indx}, update18::lets_write_path};
self::pg_uses!();

fn cpy_row(row: &mut Vec<String>) -> Vec<CellStruct>{
    let mut row_: Vec<CellStruct> = Vec::new(); 
    for i in 0..row.len(){
        row_.push(row[i].as_str().cell());
    }
    row.clear();
    row_
}

pub(crate) 
fn build_page(ps: &mut crate::_page_struct){
    let func_id = crate::func_id18::build_page;
    let mut try_entry = 0usize;
    let mut count_down_files = get_num_files(func_id);
    while try_entry < 1_000_000 {
        if get_num_files(func_id) == 0i64 {continue;}
        try_entry += 1; 
    }
    if get_num_files(func_id) == 0i64 {println!("No files found"); unsafe {libc::exit(-1);}}
    let mut num_page; if ps.num_page != i64::MAX{num_page = ps.num_page;}else{num_page = crate::get_num_page(func_id);}
    let mut num_cols; if ps.num_cols != i64::MAX{num_cols = ps.num_cols;}else{num_cols = crate::get_num_cols(func_id);}
    let mut num_rows; if ps.num_rows != i64::MAX{num_rows = ps.num_rows;}else{num_rows = crate::get_num_rows(func_id);}
    if ps.col_width != i64::MAX{crate::set_col_width(ps.col_width, func_id);}
    let num_items_on_pages = num_cols * num_rows; let stopCode: String = crate::getStop_code__!();
    num_page = calc_num_files_up2_cur_pg(); let mut filename_str: String; let mut time_to_stop = false;
    let mut row: Vec<CellStruct> = Vec::new(); let mut row_cpy: Vec<String> = Vec::new();
    //let mut row: OnceCell<Vec<CellStruct>> = OnceCell::new(); row.set(row_nested);
   // pg.table().forecolor(Color::red());
    crate::swtch::print_viewers();
    crate::swtch::print_pg_info();
    println!("Full path: {}", crate::get_full_path(func_id));
    for j in 0..num_rows{
        for i in 0..num_cols{
            let mut indx = i + num_cols * j + num_page;
            //indx = num_files - count_down_files;
            let mut res: String ="".to_string();
            let full_path_fn = move || -> String {for i in 0..1_000_000_000 {
              res = crate::globs18::get_item_from_front_list(indx, false);
              if res != "front list is empty"{return res;}
            // println!("build_page - probe 0");
            } return "".to_string()};
            let full_path = full_path_fn();
            if !unsafe {local_indx(false)}{indx -= num_page;}
            let err_ret = std::ffi::OsString::from("");
            let mut end_all_loops = || -> &std::ffi::OsString{time_to_stop = true; achtung("end all_loops"); return &err_ret};
            //println!("build_page - probe 1");
            let filename = Path::new(&full_path);
            macro_rules! filename_str0{
                () => {String::from(match filename.file_name(){
                    Some(f) => f,
                    _ => end_all_loops()
                }.to_str().unwrap()).as_str()};
            }
            if crate::globs18::eq_str(stopCode.as_str(), filename.as_os_str().to_str().unwrap()) == 0 && stopCode.len() == filename.as_os_str().to_str().unwrap().len() {println!("{}", "caught".bold().green()); 
             time_to_stop = true; break;}
            if crate::dirty!(){
               println!("cmp_str res {}", crate::globs18::eq_str(stopCode.as_str(), filename.as_os_str().to_str().unwrap()));
               println!("stop code {}, len {}; str {}, len {}", stopCode, stopCode.as_str().len(), filename.as_os_str().to_str().unwrap(), filename.as_os_str().to_str().unwrap().len());
               println!("{:?}", filename.file_name());
            }
            let mut fixed_filename: String = filename_str0!().to_string();
            ins_newlines(get_col_width(func_id).to_usize().unwrap(), &mut fixed_filename);
            if filename.is_dir(){filename_str =format!("{}: {}/", indx, fixed_filename);}
            else{filename_str = format!("{}: {}", indx, fixed_filename);}
            if filename_str == stopCode{return;}
            row_cpy.push(filename_str);
       //     count_down_files -= 1;
         //   if count_down_files <= 0 {time_to_stop = true; break;}
        }
        let count_pages = crate::get_num_files(func_id) / num_items_on_pages;
        let mut new_row: Vec<Vec<CellStruct>> = Vec::new();
        new_row.push(cpy_row(&mut row_cpy));
        print_stdout(new_row.table().bold(true).foreground_color(Some(cli_table::Color::Blue)));
        if time_to_stop {break;}
    }
    //println!("{}", pg.table().display().unwrap());
    println!("{}", crate::get_ask_user(func_id));
}

fn clear_screen(){
    if checkArg("-dbg") || checkArg("-dirty"){return;}
    let run_command = Command::new("clear")
    .output()
    .expect("can't clear screen");
if run_command.status.success(){
    io::stdout().write_all(&run_command.stdout).unwrap();
    io::stderr().write_all(&run_command.stderr).unwrap();
}
}
pub(crate) 
fn hotKeys() -> String{
    let mut Key =String::new();
    let func_id = crate::func_id18::hotKeys;
    let mut cmd = String::new();
    Key.push_str(crate::getkey().as_str());
    if crate::globs18::eq_ansi_str(&kcode::F1, Key.as_str()) == 0 {
        let msg = get_prnt(func_id);
        crate::achtung(msg.as_str());
        return "go2 0".to_string();
    } 
    if crate::globs18::eq_ansi_str(&kcode::DOWN_ARROW, Key.as_str()) == 0 {
        return "pp".to_string();
    }
    if crate::globs18::eq_ansi_str(&kcode::UP_ARROW, Key.as_str()) == 0 {
        return "np".to_string();
    }
    if crate::globs18::eq_ansi_str(&kcode::RIGHT_ARROW, Key.as_str()) == 0 {
        achtung(Key.as_str());
        unsafe {shift_cursor_of_prnt(1, func_id).shift};
        return "cr".to_string();
    }
    if crate::globs18::eq_ansi_str(&kcode::LEFT_ARROW, Key.as_str()) == 0 {
    unsafe {shift_cursor_of_prnt(-1, func_id).shift};
    //io::stdout().lock().flush().unwrap();
    achtung("left arrow");
    return "cl".to_string();}
    if "/" == Key.as_str() {let mut Key_cpy =String::from(&Key); let mut Key_ = String::from(&Key); lets_write_path(Key_cpy); crate::INS(&Key_);
    return "/".to_string();}
    if crate::globs18::eq_ansi_str(&kcode::Alt_0, Key.as_str()) == 0 {
    unsafe {
        local_indx(true);};
        let msg = format!("alt_0 num page {}", crate::get_num_page(-1));
        popup_msg(&msg);
    return "alt_0".to_string();}
    if crate::globs18::eq_ansi_str(&kcode::F12, Key.as_str()) == 0{
        unsafe {shift_cursor_of_prnt(0, func_id)};
        crate::run_cmd_str("notify-send F12"); 
        set_prnt("", func_id); return "f12".to_string();} 
    if crate::globs18::eq_ansi_str(&kcode::DELETE, Key.as_str()) == 0{
        let shift = unsafe {shift_cursor_of_prnt(1, func_id).shift};
        let mut indx = get_prnt(func_id).chars().count();
        if shift <= indx {indx -= shift;}
        let prnt = rm_char_from_string(indx, &get_prnt(func_id));
        set_prnt(prnt.as_str(), func_id);
        return "del".to_string();} 
    let ansiKey: u8 = match Key.as_str().bytes().next(){
        Some(val) => val,
        _ => 0
    };
    if ansiKey == 0{return crate::get_prnt(func_id);}
    if crate::dirty!(){println!("ansi {}, Key {:?}", ansiKey, Key);}
    if kcode::ENTER == ansiKey{return crate::get_prnt(func_id);} 
    if kcode::BACKSPACE == ansiKey{crate::press_BKSP(); return "bksp".to_string();} 
    if kcode::ESCAPE == ansiKey{println!("esc pressed");}
    if kcode::TAB == ansiKey{println!("tab pressed");}  
   crate::INS(&Key);
       // enter();
return Key.to_string();
}
pub fn manage_pages(){
let mut Key: String = "".to_string(); 
let mut count: u64 = 0;
let mut bal =String::new();
    loop{
        //set_prnt(&bal, -1);
        let mut ps: crate::_page_struct = unsafe {crate::swtch::swtch_ps(-1, None)};
        build_page(&mut ps);
        exec_cmd(custom_input());
        clear_screen();
        //crate::run_cmd0("clear".to_string());
    }
}
pub(crate) fn repeat_char(num_of_times: usize, this_char: &str) -> String{
    let mut ret = String::new();
    for i in 1..num_of_times {ret.push_str(this_char);}
    ret
}
pub(crate) fn wipe_cmd_line(len_2_wipe: usize){
    let many_spaces = repeat_char(len_2_wipe, " ");
    println!("\r{}", many_spaces);
}
pub(crate) fn form_cmd_line(prompt: String, prnt: String){
    let whole_line_len = prompt.len() + prnt.len() + 2;
    let print_whole_line = format!("\r{}: {}", prompt, prnt);
    print!("{}", print_whole_line);
}
pub(crate) fn form_cmd_line_default(){
    let func_id = crate::func_id18::form_cmd_line_default;
    let prompt = crate::get_prompt(func_id); let mut ret = unsafe {crate::shift_cursor_of_prnt(0, func_id)};
    let mut prnt = ret.str__;
    let len = get_prnt(func_id).chars().count();
    if ret.shift == len {prnt = format!("👉{}", prnt)}
    else if ret.shift < len {ret.shift = len - ret.shift;
    prnt.push('👈');
    prnt = ins_last_char_to_string1_from_string1(ret.shift, prnt);}
    let whole_line_len = prompt.len() + prnt.len() + 2;

    wipe_cmd_line(whole_line_len);
    form_cmd_line(prompt, prnt)
}
pub(crate) fn custom_input() -> String{
    form_cmd_line_default();
    return hotKeys();
}
pub(crate) unsafe fn exec_cmd_cnt(count_: bool) -> u64{
    static mut count: u64 = 0;
    if count_ {count += 1;}
    count
}
fn exec_cmd(cmd: String){
    let func_id = crate::func_id18::exec_cmd;
    //println!("cmd {} func {}, prnt {}", cmd, crate::func_id18::get_func_name(func_id), crate::get_prnt(func_id));
    
    if cmd == "np"{
        unsafe{exec_cmd_cnt(true)};
        let num_page = crate::get_num_page(func_id) + 1;
        crate::set_num_page(num_page, func_id);
        return;
    }
    if crate::globs18::eq_ansi_str(cmd.as_str().substring(0, 2), "pp") == 0{
       // unsafe{exec_cmd_cnt(true)};
        let mut num_page = crate::get_num_page(func_id);
        if num_page > 0{num_page -= 1;}
        crate::set_num_page(num_page, func_id);
        return;
    }
    if crate::globs18::eq_ansi_str(cmd.as_str().substring(0, 3), "go2") == 0{
        let (_, opt) = split_once(cmd.as_str(), " ");
        if opt == "none" {set_ask_user("wrong use of go2: go2 <indx of page>", func_id); return;}
        let pg_num: i64 = match i64::from_str_radix(&opt, 10){
            Ok(val) => val,
            _ => {set_ask_user("wrong use of go2: go2 <indx of page>", func_id); return}
        };
        crate::set_num_page(pg_num, func_id);
        return;
    }
      if cmd.as_str().substring(0, 2) == "fp"{
        let (_, opt) = split_once(cmd.as_str(), " ");
        if opt == "none" {set_full_path("wrong use of fp: fp <indx of file>", func_id); return;}
        let file_indx: i64 = match i64::from_str_radix(&opt, 10){
            Ok(val) => val,
            _ => {set_full_path("wrong use of fp: fp <indx of file>", func_id); return}
        };
        let file_full_name =  crate::globs18::get_item_from_front_list(file_indx, true);
        set_full_path(&file_full_name, func_id);
        return;
    }
    unsafe {swtch_fn(-1, cmd)}
}