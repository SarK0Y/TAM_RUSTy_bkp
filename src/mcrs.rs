//use crate::exts::*;
#[macro_use]
//#[macro_export]
macro_rules! getStop_code__ {
    () => {
        unsafe{ps::page_struct("", STOP_CODE_, -1).str_}
    };
}