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
        PRNT = RwLock::new(String::new());
        *PRNT.write().unwrap() = $x.to_string();
    };
}