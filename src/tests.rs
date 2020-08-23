use super::{parse_args, parse_and_replace};


#[test]
fn good_args_good_file() {
    let args = r#"-v col1="'names'" -v col2=ages'' -v col3='"names2"' -v limit=14 -f d:\fuzz.txt"#;
    let (_placeholders_map, sql_file_path)  = parse_args(args.split_whitespace().map(|s| s.to_owned()).collect());
    assert_eq!(sql_file_path, r#"d:\fuzz.txt"#);
    // Check hashmap
}


#[test]
#[should_panic(expected="Expected -v/-f for argument, bug got -g")]
fn bad_flag() {
    let args = r#"-g col1="names" -f d:\fuzz.txt"#;
    let (_placeholders_map, _sql_file_path)  = parse_args(args.split_whitespace().map(|s| s.to_owned()).collect());
}

#[test]
#[should_panic(expected="Expected -v/-f for argument, bug got")]
fn non_flag() {
    let args = r#"col1="names" -g -f d:\fuzz.txt"#;
    let (_placeholders_map, _sql_file_path)  = parse_args(args.split_whitespace().map(|s| s.to_owned()).collect());
}

#[test]
#[should_panic(expected=r#"Argument after -v flag should be of form arg1=FIELD1, got col1"names""#)]
fn no_equals_sign() {
    let args = r#"-v col1"names" -f d:\fuzz.txt"#;
    let (_placeholders_map, _sql_file_path)  = parse_args(args.split_whitespace().map(|s| s.to_owned()).collect());
}

#[test]
#[should_panic(expected=r#"The system cannot find the file specified"#)]
fn bad_filepath() {
    let args = r#"-v col1="'names'" -f d:\fudd.txt"#;
    parse_and_replace(args.split_whitespace().map(|s| s.to_owned()).collect());
}

#[test]
fn test_replacement() {
    let args = r#"-v col1="'names'" -v col2=ages'' -v col3='"names2"' -v limit=14 -f d:\fuzz.txt"#;
	// let query = r#"select $(col1) $(col2) $(col3) from table limit $(limit)"#;
	let res = parse_and_replace(args.split_whitespace().map(|s| s.to_owned()).collect());
    assert_eq!(res, r#"select 'names' ages'' 'names2' from table limit 14"#);
}