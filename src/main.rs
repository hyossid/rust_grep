use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let target_binary_name = &args[0];
    let query = &args[1];
    let file_path = &args[2];   // String does not implement copy so cannot transfer ownership here
    

    println!("query is :{}",query);
    println!("file_path is :{}", file_path);

    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}