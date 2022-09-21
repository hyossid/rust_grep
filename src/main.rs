use std::env;
use std::process;
use rust_grep::Config;  // better to mention struct if it is the case, for function just the module 

fn main() {
    let args: Vec<String> = env::args().collect();

    //let config = parse_config(&args); // String does not implement copy so cannot transfer ownership here
    //let config = Config::new_config(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Provlem parsing arguments: {err}");
        process::exit(1);
    });
    println!("query is :{}",config.query);
    println!("file_path is :{}", config.file_path);

    if let Err(e) = rust_grep::run(config) {   // Better to use returned 'Result'
        println!("Application error: {e}");

        process::exit(1);
    }

}
