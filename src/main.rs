use std::fs;

fn task1() {
    let filename = "src/day1/data/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let v: Vec<&str> = contents.split_whitespace().collect();
    let mut numbers: Vec<i32> = Vec::new();
    for number in v{
        numbers.push(number.parse::<i32>().unwrap());
    }
    for number in &numbers{
        for number2 in &numbers{
            if number + number2 == 2020{
                println!("{}", number*number2);
            }
            for number3 in &numbers{
                if number + number2 + number3 == 2020{
                    println!("{}", number*number2*number3);
                }
            }
        }
    }
}

fn main() {
    task1();
}
