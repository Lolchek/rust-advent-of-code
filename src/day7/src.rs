use std::fs;
use regex::Regex;


fn parse_from_file(filename: &str){
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let data: Vec<&str> = contents.split("\n").collect();
    // println!("{:?}", data);

    let first_two_words_regex = Regex::new(r"^(?:\w+\s){2}").unwrap();
    let contain_colors_regex = Regex::new(r"[0-9]+\s*(\w+\s\w+)").unwrap();

    for row in data{
        // println!{"{}", row};
        let contain_colors: Vec<regex::Captures> = contain_colors_regex.captures_iter(row).collect();
        println!{"LHS: {:?}", &first_two_words_regex.captures(row).unwrap()[0].trim()};
        for c in contain_colors{
            println!{"RHS: {:?}", &c[1]};
        }
        println!("");
    }
}

pub fn day7(filepath: &str){
    parse_from_file(filepath);
}