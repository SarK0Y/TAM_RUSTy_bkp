use crate::exts::mcrs_uses;
self::mcrs_uses!();
#[macro_use]
//#[macro_export]
macro_rules! getStop_code__ {
    () => {
        unsafe{crate::page_struct("", crate::STOP_CODE_, -1).str_}
    };
}
#[macro_use]
macro_rules! dirty {
    () => {
        crate::checkArg("-dirty")
    };
}
macro_rules! set_prnt_ {
    ($x: expr) => {
       // PRNT = RwLock::new(String::new());
        //*PRNT.write().unwrap() = $x.to_string();
        PRNT = UnsafeCell::new($x.to_string());
        crate::popup_msg(&*PRNT.get()); //PRNT.get().as_ref().expect("set_prnt_!() can't unwrap PRNT").clear();
        //*PRNT.get_mut().as_ref().expect("set_prnt_!() can't unwrap PRNT").push_str(cpy_str($x).as_str());
    };
}
macro_rules! close_termios__ {
    (termios: expr) => {
       let res = match tcsetattr(stdin_fd, TCSANOW, &$termios){
        Err(e) => {format!("{}", e)},
        Ok(len) => {format!("{:?}", len)}
    }
    };
}