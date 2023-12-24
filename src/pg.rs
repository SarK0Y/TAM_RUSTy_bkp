use crate::{exts::pg_uses, ps18::{set_prnt, get_cur_cur_pos, set_prompt}};
self::pg_uses!();

fn move_out_of_scope(row: &mut Vec<String>) -> Vec<CellStruct>{
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
    let mut num_page; if ps.num_page != i64::MAX{num_page = ps.num_page;}else{num_page = crate::get_num_page(func_id);}
    let mut num_cols; if ps.num_cols != i64::MAX{num_cols = ps.num_cols;}else{num_cols = crate::get_num_cols(func_id);}
    let mut num_rows; if ps.num_rows != i64::MAX{num_rows = ps.num_rows;}else{num_rows = crate::get_num_rows(func_id);}
    let num_items_on_pages = num_cols * num_rows; let stopCode: String = crate::getStop_code__!();
    num_page *= num_cols * num_rows; let mut filename_str: String; let mut time_to_stop = false;
    let mut pg: Vec<Vec<CellStruct>> = Vec::new();  let mut row: Vec<CellStruct> = Vec::new(); let mut row_cpy: Vec<String> = Vec::new();
    //let mut row: OnceCell<Vec<CellStruct>> = OnceCell::new(); row.set(row_nested);
   // pg.table().forecolor(Color::red());
    for j in 0..num_rows{
        for i in 0..num_cols{
            let cell_num = j + num_cols * i + num_page;
            let mut res: String ="".to_string();
            let full_path_fn = move || -> String {for i in 0..1_000_000_000 {
              res = crate::globs18::get_item_from_front_list(cell_num);
              if res != "front list is empty"{return res;}
            // println!("build_page - probe 0");
            } return "".to_string()};
            let full_path = full_path_fn();
            //println!("build_page - probe 1");
            let filename = Path::new(&full_path);
            macro_rules! filename_str0{
                () => {String::from(filename.file_name().unwrap().to_str().unwrap()).as_str()};
            }
            if crate::globs18::eq_str(stopCode.as_str(), filename.as_os_str().to_str().unwrap()) == 0 && stopCode.len() == filename.as_os_str().to_str().unwrap().len() {println!("{}", "caught".bold().green()); 
             time_to_stop = true; break;}
            if crate::dirty!(){
               println!("cmp_str res {}", crate::globs18::eq_str(stopCode.as_str(), filename.as_os_str().to_str().unwrap()));
               println!("stop code {}, len {}; str {}, len {}", stopCode, stopCode.as_str().len(), filename.as_os_str().to_str().unwrap(), filename.as_os_str().to_str().unwrap().len());
               println!("{:?}", filename.file_name());
            }
            if filename.file_name() == None{println!("{}", "caught".bold().blue()); break;}
            if filename.is_dir(){filename_str =format!("{}: {}", cell_num, filename_str0!());}
            else{filename_str = format!("{}: {}", cell_num, filename_str0!());}
            if filename_str == stopCode{return;}
            row_cpy.push(filename_str);
        }
        let count_pages = crate::get_num_files(func_id) / num_items_on_pages;
        println!("num page {}, num pages {}", num_page, count_pages);
        pg.push(move_out_of_scope(&mut row_cpy));
        if time_to_stop {break;}
    }
    //println!("{}", pg.table().display().unwrap());
    print_stdout(pg.table().bold(true).foreground_color(Some(cli_table::Color::Blue)));
    
}

fn clear_screen(){
    crate::run_cmd_str("clear");
}
pub(crate) 
fn hotKeys() -> String{
    let mut Key =String::from("");
    let func_id = crate::func_id18::hotKeys;
   
    let mut cmd = String::new();

    Key.push(crate::getch());
    if crate::globs18::eq_ansi_str(&kcode::F12, Key.as_str()) == 0{crate::run_cmd_str("notify-send F12"); 
        crate::set_cur_cur_pos(0, func_id); set_prnt("", func_id); return "go2 0".to_string();} 
    let ansiKey: u8 = match Key.as_str().bytes().next(){
        Some(val) => val,
        _ => 0
    };
    if ansiKey == 0{return crate::get_prnt(func_id);}
    if crate::dirty!(){println!("ansi {}, Key {:?}", ansiKey, Key);}
    if kcode::ENTER == ansiKey{return crate::get_prnt(func_id);} 
    if kcode::BACKSPACE == ansiKey{crate::press_BKSP(); return "go2 0".to_string();} 
    if kcode::ESCAPE == ansiKey{println!("esc pressed");}
    if kcode::TAB == ansiKey{println!("tab pressed");}  
   crate::INS(&Key);
       // enter();
    let cur_cur_pos = get_cur_cur_pos(func_id);
    let prompt = format!("{} {}", crate::get_prompt(func_id), cur_cur_pos);
    set_prompt(&prompt, func_id);
return "ok".to_string();
}
pub fn manage_pages(mut ps: crate::_page_struct){
let mut Key: String = "".to_string(); 
let mut count: u64 = 0;
let mut bal =String::new();
    loop{
        //set_prnt(&bal, -1);
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
    let prompt = crate::get_prompt(func_id); let prnt = crate::get_prnt(func_id);
    let whole_line_len = prompt.len() + prnt.len() + 2;
    wipe_cmd_line(whole_line_len);
    form_cmd_line(prompt, prnt)
}
pub(crate) fn custom_input() -> String{
    form_cmd_line_default();
    return hotKeys();
}
fn exec_cmd(cmd: String){
    let func_id = crate::func_id18::exec_cmd;
    //println!("cmd {} func {}, prnt {}", cmd, crate::func_id18::get_func_name(func_id), crate::get_prnt(func_id));
    if cmd == "np"{
        let num_page = crate::get_num_page(func_id) + 1;
        crate::set_num_page(num_page, func_id);
    }
      let num_page = crate::get_num_page(func_id) + 1;
        crate::set_num_page(num_page, func_id);
}