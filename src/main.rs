use std::env;
use std::mem::MaybeUninit;
#[derive(Clone)]
struct ret0 {
    s: [char; 512],
    res: bool
}
fn get_arg_in_cmd(key: &str) -> ret0{
let mut s: [char; 512] = ['\0'; 512];
let mut ret: ret0 = ret0{s, res: false};
let args: Vec<_> = env::args().collect();
let args_2_str = args.as_slice();
for i in 1..args.len(){
    if args_2_str[i] == key {
        let arr: Vec<char> = (args[i + 1]).chars().collect();
        if arr.len() > 512 {
            for i in 0..511{
               ret.s[i] = arr[i];
            }
        } else{
            for i in 0..arr.len(){
               ret.s[i] = arr[i];
            }
        }
        ret.res = true;
        return ret;
}
}
ret.res = false;
return ret;
}
fn main (){
let x: i64= 5;
let out: ret0 = get_arg_in_cmd("-tst");
let out1: ret0 = get_arg_in_cmd("-тст");
println!("argument from cmd (-tst) {}", String::from_iter(out.s));
println!("argument from cmd (-тст) {}", String::from_iter(out1.s));
return;
}
