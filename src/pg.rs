use crate::exts::pg_uses;
pg_uses!();

fn move_out_of_scope(row: &mut Vec<String>) -> Vec<CellStruct>{
    let mut row_: Vec<CellStruct> = Vec::new(); 
    for i in 0..row.len(){
        row_.push(row[i].as_str().cell());
    }
    row.clear();
    row_
}
pub(crate) 
fn build_page(ps: page_struct){
    let func_id = func_id::build_page;
    let mut num_page; if ps.num_page != i64::MAX{num_page = ps.num_page;}else{num_page = get_num_page(func_id);}
    let mut num_cols; if ps.num_cols != i64::MAX{num_cols = ps.num_cols;}else{num_cols = get_num_cols(func_id);}
    let mut num_rows; if ps.num_rows != i64::MAX{num_rows = ps.num_rows;}else{num_rows = get_num_rows(func_id);}
    let count_pages = get_num_pages(func_id); let stopCode: String = getStop_code__!();
    num_page *= num_cols * num_rows; let mut filename_str: String; let mut time_to_stop = false;
    let mut pg: Vec<Vec<CellStruct>> = Vec::new();  let mut row: Vec<CellStruct> = Vec::new(); let mut row_cpy: Vec<String> = Vec::new();
    //let mut row: OnceCell<Vec<CellStruct>> = OnceCell::new(); row.set(row_nested);
   // pg.table().forecolor(Color::red());
    for j in 0..num_cols{
        for i in 0..num_rows{
            let cell_num = j + num_cols * i + num_page;
            let full_path = get_item_from_front_list(cell_num);
            let filename = Path::new(&full_path);
            macro_rules! filename_str0{
                () => {String::from(filename.file_name().unwrap().to_str().unwrap()).as_str()};
            }
            if cmp_str(stopCode.as_str(), filename.as_os_str().to_str().unwrap()) == 0 && stopCode.len() == filename.as_os_str().len() {time_to_stop = true; break;}
            println!("stop code {}, len {}; str {}, len {}", stopCode, stopCode.as_str().len(), filename.as_os_str().to_str().unwrap(), filename.as_os_str().to_str().unwrap().len());
            println!("{:?}", filename.file_name());
            if filename.file_name() == None{break;}
            if filename.is_dir(){filename_str =format!("{}: {}", cell_num, filename_str0!());}
            else{filename_str = format!("{}: {}", cell_num, filename_str0!());}
            if filename_str == stopCode{return;}
            row_cpy.push(filename_str);
        }
        pg.push(move_out_of_scope(&mut row_cpy));
        if time_to_stop {break;}
    }
    println!("{}", pg.table().display().unwrap());
    
}