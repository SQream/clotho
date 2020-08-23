// #![allow(warnings)]      // For development/experimenting cycles

use std::collections::HashMap;
use aho_corasick::AhoCorasick;

// Instruct `caro test` command to look for tests in tests.rs
#[cfg(test)] 
mod tests;

const VERSION : &str = "1.0.0"; 

fn print_err(message: &str, sql_file_path: &str) {
    eprintln!("Error: While processing script '{}':", sql_file_path);
    eprintln!("{}", message);
    std::process::exit(1);
}


fn parse_args(args: Vec<String>) -> (HashMap<String, String>, String){
    /* Parse variables and their replacements into a hashmap, return it and the file to open 
       Errors thrown:
       - Unexpected flag
       - Bad argument form after -v
       - Argument contains illegal character/s
    */

    let mut placeholders_map : HashMap<String, String> = HashMap::new();
    let (mut flag, mut sql_file_path) = ("".to_owned(), "".to_owned());
    let mut expecting_flag = false;

    // Start running over the command line args
    for arg in args {
        expecting_flag = !expecting_flag;
        if expecting_flag{ 
            if arg != "-v" && arg != "-f" && arg != "-V" && arg != "-h" && arg != "--help" && arg != "--version" && arg != "--variable" && arg != "--file" {
                print_err(&format!("Invalid option {}\nUsage: clotho [OPTION [...]] -f [FILE]", arg), &sql_file_path);
            }  
            else if arg == "-h" || arg == "--help" { 
                println!(
"clotho v{} 

SQL preprocessor for variable replacement

USAGE: 
    clotho [OPTIONS]

OPTIONS: 
    -h, --help                  prints this message 
    -V, --version               prints version information
    -v, --variable=NAME=VALUE   set variable NAME to VALUE
    -f, --file=FILENAME         file with SQL variables to replace"
             , VERSION);
                std::process::exit(0);
            }
            
            else if arg == "-V" || arg == "--version" {
                println!("clotho v{}", VERSION);
                std::process::exit(0);
            }
            else {
                flag = arg.to_owned();
                continue;
            }
        }
        
        if flag == "-v" || flag == "--variable" {
            let var_and_replacer = arg.splitn(2, "=").collect::<Vec<_>>();
            if var_and_replacer.len() < 2 {
                print_err(&format!("Invalid variable setting {}. Option -v requires a setting.\nVariables are set in the format '-v NAME=VALUE'.", arg), &sql_file_path);
                
                /* In case unsetting is back
                if !placeholders_map.contains_key(&("$(".to_owned() + var_and_replacer[0] + ")")) {
                    // print_err(&format!("Argument after -v flag should be of form arg1=FIELD1, got {}", arg));
                    print_err(&format!("Argument {} does not exist for unsetting, perhaps try form arg1=FIELD1 or", arg));
                }
                placeholders_map.remove(&("$(".to_owned() + var_and_replacer[0] + ")"));
                */
            }
            else if !var_and_replacer[0].chars().all(|x| x.is_alphanumeric() || "-_+@,.-/?#;".contains(x))  {
                print_err(&format!("Invalid variable setting. Variable {} is invalid.\nA valid variable name contains only the following characters: a-zA-Z0-9-_+@,./?#;", var_and_replacer[0]), &sql_file_path);
            }
            else {
                placeholders_map.insert("$(".to_owned() + var_and_replacer[0] + ")", var_and_replacer[1].to_owned());
            }
        }

        if flag == "-f" || flag == "--file" {
            sql_file_path = arg;
            break;
        }
    }
    
    // println!("placeholder map: {:?}", placeholders_map);
    (placeholders_map, sql_file_path)
}


fn parse_and_replace(args: Vec<String>) -> String{
    /* Errors thrown - No file passed, Can't open file, Not all variables in file were set */

    let (placeholders_map, sql_file_path)  = parse_args(args);
    if sql_file_path == "" {
        eprintln!("Error: No file specified for processing.");
        eprintln!("Usage: clotho -v arg1=val1 -v arg2=val2 ... -f [FILE]");
        std::process::exit(1);
    }
    // println!("\n\x1B[38;2;0;0;240mVariables to replace:\x1B[0m\n{:#?}", placeholders_map);
    let queries = std::fs::read_to_string(&sql_file_path).unwrap_or_else(|e| {print_err(&format!("{}", e), &sql_file_path); "".to_owned()});

    // See if all variables in the file were replaced
    let mut vars_in_file: Vec<String> = Vec::new();
    let mut var_idx = 0;
    let mut end_idx;

    loop {
        var_idx = match &queries[var_idx..].find("$(") {
            Some(idx) => var_idx + *idx + 1,
            None => queries.len(),
        };
        // Look for ending ")" for this variable
        end_idx = match &queries[var_idx..].find(")") {
            Some(idx) => var_idx + *idx,
            None => {break;},
        };
        // println!("index: {}", var_idx);

        vars_in_file.push(queries[var_idx-1..end_idx+1].to_owned());
        var_idx = end_idx + 1;
    }
    // println!("vars in file: {:?}", vars_in_file);
    if !vars_in_file.iter().all(|var_in_file| placeholders_map.contains_key(var_in_file)) {
        print_err(&format!("Not all referenced variables in the file - {} - were  set.", vars_in_file.join(",")), &sql_file_path);
    }

    // Replace the placeholders
    let ac = AhoCorasick::new(placeholders_map.keys().collect::<Vec<_>>());
    let replaced_queries = ac.replace_all(&queries, &placeholders_map.values().collect::<Vec<_>>());

    // println!("\n\x1B[38;2;0;0;240mFile to open:\x1B[0m {}", sql_file_path);
    // println!("\n\x1B[38;2;0;0;240mQueries for replacing:\x1B[0m\n{}", queries);
    // println!("\n\x1B[38;2;0;0;240mQueries after replacing:\x1B[0m\n{}", replaced_queries);

    replaced_queries
}


// cargo run -- -v col1="'names'" -v col2=ages'' -v col3='"names2"' -v limit=14 -f d:\fuzz.txt
fn main()  {

    let mut args = std::env::args().collect::<Vec<_>>();
    args.remove(0);  // First argument is the filename

    let replaced_queries = parse_and_replace(args);
    println!("{}", replaced_queries); // Printout for sqream sql
}
