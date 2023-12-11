macro_rules! use_all {
    () => {
use std::env;
use colored::Colorize;
use substring::Substring;
use std::str::{self, from_utf8};
use std::string;
use chrono::{DateTime, Local};
use std::io::{self, Write};
use std::any::{self, type_name};
use std::fmt;
use std::fs;
use std::fs::File;
use cli_table::format;
use once_cell::unsync::OnceCell;
use std::cell::Cell;
use std::str::Split;
use std::thread;
use threadpool::{ThreadPool, Builder};
use std::sync::mpsc;
use std::ffi::CString;
use std::process::{Command, Stdio};
use std::path::Path;
use num_traits::cast::ToPrimitive;
use std::io::{BufRead, BufReader};
#[path = "func_id.rs"]
mod func_id;
#[path = "globs.rs"]
mod globs;
use crate::globs::*;   
#[path = "page_struct_.rs"]
mod ps;    
use ps::page_struct_ret;
use ps::page_struct;
#[path = "pg.rs"]
mod pg;
/*#[path = "mcrs.rs"]
mod mcrs;
use mcrs::*;*/
    };
}
macro_rules! pg_uses {
    () => {
use std::env;
use colored::Colorize;
use substring::Substring;
use std::str::{self, from_utf8};
use std::string;
use chrono::{DateTime, Local};
use std::io::{self, Write};
use std::any::{self, type_name};
use std::fmt;
use std::fs;
use std::fs::File;
use cli_table::format;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use std::str::Split;
use std::thread;
use threadpool::{ThreadPool, Builder};
use std::sync::mpsc;
use std::ffi::CString;
use std::process::{Command, Stdio};
use std::path::Path;
use num_traits::cast::ToPrimitive;
use std::io::{BufRead, BufReader};
#[path = "func_id.rs"]
mod func_id;
#[path = "globs.rs"]
mod globs;
use crate::globs::*;
#[path = "page_struct_.rs"]
mod ps;    
use crate::ps::*;
#[macro_use]
#[path = "mcrs.rs"]
mod mcrs;
use getStop_code__;
use cli_table::CellStruct;
use once_cell::sync::OnceCell;   
    };
}
macro_rules! page_struct_uses {
    () => {
use std::env;
use colored::Colorize;
use substring::Substring;
use std::str::{self, from_utf8};
use std::string;
use chrono::{DateTime, Local};
use std::io::{self, Write};
use std::any::{self, type_name};
use std::fmt;
use std::fs;
use std::fs::File;
use cli_table::format;
use once_cell::unsync::OnceCell;
use std::cell::Cell;
use std::str::Split;
use std::thread;
use threadpool::{ThreadPool, Builder};
use std::sync::mpsc;
use std::ffi::CString;
use std::process::{Command, Stdio};
use std::path::Path;
use num_traits::cast::ToPrimitive;
use std::io::{BufRead, BufReader};
#[path = "func_id.rs"]
mod func_id;
#[path = "globs.rs"]
mod globs;
#[path = "page_struct_.rs"]
//mod ps;    
use crate::globs::*;   
    };
}

pub(crate) use use_all;
pub(crate) use page_struct_uses;
pub(crate) use pg_uses;