pub(crate) 
fn hotKeys( Key: &mut String) -> &'static str{
    let stdin = io::stdin();
    let enter = ||
{
    let enter: [u8; 1] = [13; 1];
    let mut writeIn_stdin = unsafe {File::from_raw_fd(0/*stdin*/)};
    writeIn_stdin.write(&enter);
    println!("gotta enter");
};
enter();
    for line in stdin.lines(){
        let mut readable = true;
        let msg = |readable: &mut bool| -> String { let mut readable = false; return "err".to_string();};
        let mut line = match line {
            Ok(line) => line,
            _ => msg(&mut readable)
        };
        if !readable{break;}
        Key.push_str(line.as_str());
        enter();
        break;
    }
    "none"
}
pub(crate) 
fn hotKeys_tst( Key: &mut String) -> &'static str{
    let mut stdin = io::stdin();
    let stdin_fd = 0;
    let mut stdout = io::stdout();
    let mut stdin_buf: [u8; 4] =[0;4];
     let termios = Termios::from_fd(stdin_fd).unwrap();
    let mut new_termios = termios.clone(); 
    new_termios.c_lflag &= !(ICANON); 
   // let res = tcsetattr(stdin_fd, TCSANOW, &mut new_termios).unwrap();
    //println!("{:?}", res);
    let enter = ||
{
    let enter: [u8; 1] = [13; 1];
    let mut writeIn_stdin = unsafe {File::from_raw_fd(0/*stdin*/)};
    writeIn_stdin.write(&enter);
    println!("gotta enter");
};
    stdout.lock().flush().unwrap();
    let red_stdin = stdin.read(&mut stdin_buf);
    println!("{:?}", red_stdin);
    let str0 = match str::from_utf8(&stdin_buf){
        Ok(s) => s,
        _ => ""
    };
   Key.push_str(&str0);
       // enter();
        let mut len_char: i32;
   /* let res = match tcsetattr(stdin_fd, TCSANOW, &termios){
        Err(e) => {format!("{}", e)},
        Ok(len) => {format!("kkkkkkkkkkk {:#?}", len)}
    };*/
//   println!("{}", res);
    "none"
}
/*perhaps for future */
pub(crate) fn redirect_stdout_to_buf() -> Redirect<File>{
// Open a log
let log = OpenOptions::new()
    .read(true)
    .create(true)
    .write(true)
    .open("/tmp/my_log.log")
    .unwrap();

let print_redirect = Redirect::stdout(log).unwrap();
print_redirect
}
/*perhaps for future */
fn  stack_size(){let builder = thread::Builder::new().stack_size(80 * 1024 * 1024);
let handler = builder.spawn(|| {
    // thread code
}).unwrap();
handler.join().unwrap();}
/*********** dirt ****************** */
fn cpy_tbl(tbl: &Vec<Vec<CellStruct>>) -> Vec<Vec<CellStruct>>{
    let mut tbl0: Vec<Vec<CellStruct>> = Vec::new();
    //let mut get 
    for i in 0..tbl.len(){
        tbl0.push(match tbl.get(i){
            Some(v) => move_out_of_scope(&mut v),
            _ => return tbl0
        });
    }
    tbl0
}
fn cpy_row(row: &Vec<CellStruct>) -> Vec<CellStruct>{
    let mut row_: Vec<CellStruct> = Vec::new(); 
    for i in 0..row.len(){
        let tst: str = row[i];
        //let mut cell = CellStruct::cell(row[i].cell());
        //row_.push(cell);
    }
    row.clear();
    row_
}
fn cpy_tbl(tbl: &Vec<Vec<CellStruct>>) -> Vec<Vec<CellStruct>>{
    let mut tbl0: Vec<Vec<CellStruct>> = Vec::new();
    //let mut get 
    for i in 0..tbl.len(){
        tbl0.push(match tbl.get(i){
            Some(v) => cpy_row(& v),
            _ => return tbl0
        });
    }
    tbl0
}
pub(crate) fn user_writing_path0(key: String) -> bool{
    let mut written_path = String::new();
    let mut x: u64  = 2;
    for i in 0..1000{
        for j in 0..1000{if (u64::MAX - x )/x > x{ x *= x;} x -= (j+i);}
        written_path = set_user_written_path_from_prnt();
        if written_path != ""{break;}
    }
    update_dir_list(&written_path, "-maxdepth 1", false);
    true
}
fn make_rows(rows: &mut Vec<Vec<CellStruct>>, pg: &mut Vec<Vec<CellStruct>>, row_cpy: &mut Vec<String>){
    rows.push(move_out_of_scope(row_cpy));
}
fn print_rows(rows: &mut Vec<TableStruct>){
    let count_rows = rows.len();
    for i in 0..count_rows{
        print_stdout(rows[i].table().bold(true).foreground_color(Some(cli_table::Color::Blue)));
    }
}
