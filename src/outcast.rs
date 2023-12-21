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
fn  stack_size(){let builder = thread::Builder::new().stack_size(80 * 1024 * 1024);
let handler = builder.spawn(|| {
    // thread code
}).unwrap();
handler.join().unwrap();}