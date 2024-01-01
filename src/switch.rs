use once_cell::sync::OnceCell;
pub(crate) unsafe fn swtch_fn(indx: i8, cmd: String){
    static mut fst_run: bool = true;
    static mut fn_indx: u8 = 0;
    static mut fn_: OnceCell<Vec<fn(String) -> bool>> = OnceCell::new();
    if fst_run{
        let fn_vec: Vec<fn(String) -> bool> = Vec::new();
        fn_.set(fn_vec); fst_run = false;
    }
}
pub(crate) fn viewer(cmd: String) -> bool{
    let func_id = crate::func_id18::viewer_;
    let  (app_indx, file_indx) = crate::split_once(&cmd, " ");
    if app_indx == "none" || file_indx == "none"{
        crate::core18::errMsg("To run file w/ viewer, You need to type '<indx of viewer> <index of file>'", func_id);
        return false
    }
    
    false
}
pub(crate) fn get_viewer(indx: usize, func_id: i64) -> String{
    let ret = unsafe {set_o_get_usize(indx, func_id)};
    if ret.1{return unsafe{crate::page_struct("", crate::VIEWER_, func_id).str_};}
"locked".to_string()
}
pub(crate) fn add_viewer(val: &str, func_id: i64) -> String{return unsafe{crate::page_struct(val, crate::set(crate::VIEWER_), func_id).str_}}
pub(crate) unsafe fn set_o_get_usize(val: usize, func_id: i64) -> (usize, bool){
    static mut owner_id: i64 = i64::MIN;
    static mut actual_val: usize = 0;
    if owner_id == func_id && val == usize::MAX{
        owner_id = i64::MIN;
        return (actual_val, true)
     }
    if owner_id == i64::MIN{owner_id = func_id; actual_val = val; return (val, true);}
    (usize::MAX, false)
}