use std::fs;
use std::convert::TryInto;

mod day3;
use day3::day3code::day3;

mod day4;
use day4::src::day4;

mod day5;
use day5::src::day5;

mod day7;
use day7::src::day7;

fn task1() {
    let filename = "src/day1/data/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let v: Vec<&str> = contents.split_whitespace().collect();
    let mut numbers: Vec<i32> = vec![];
    for number in v{
        numbers.push(number.parse().unwrap());
    }
    for number in numbers.iter(){
        for number2 in numbers.iter(){
            if number + number2 == 2020{
                println!("{}", number*number2);
            }
            for number3 in numbers.iter(){
                if number + number2 + number3 == 2020{
                    println!("{}", number*number2*number3);
                }
            }
        }
    }
}

fn task2() {
    let filename = "src/day2/data/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect(); 

    let mut correct: i32 = 0;
    let mut correct2: i32 = 0;
    for line in lines{
        if line != "" {
            let v: Vec<&str> = line.split_whitespace().collect();
            let minmax: Vec<&str> = v[0].split("-").collect();
            let min: i32 = minmax[0].parse().unwrap();
            let max: i32 = minmax[1].parse().unwrap();
            
            let letter: char;
            match v[1].chars().nth(0){
                None => letter = '\0',
                Some(x) => letter = x,
            };
            let password: &str = v[2];
            println!("{}-{}, {}, {}", min, max, letter, password);

            let mut count :i32 = 0;
            for c in password.chars(){
                if c == letter{
                    count += 1;
                }
                
            }
            if count >= min && count <= max{
                correct += 1;
                println!("Good");
            }
            let mut first = false;
            let mut second = false;
            let minsize: usize = min.try_into().unwrap();
            let maxsize: usize = max.try_into().unwrap();
            if password.chars().nth(minsize-1) == Some(letter){
                first = true;
            }

            if password.chars().nth(maxsize-1) == Some(letter){
                second = true;
            }
            if (first || second) && !(first && second){
                correct2 += 1;
                println!("Good2")
            }
        }
    }
    println!("{}", correct);
    println!("{}", correct2);
}

fn task3(){
    let filepath = "src/day3/data/input.txt";
    day3(filepath);
}

fn task4(){
    let filepath = "src/day4/data/input.txt";
    // let filepath = "src/day4/data/input_small_invalid.txt";
    // let filepath = "src/day4/data/input_small_valid.txt";
    // let filepath = "src/day4/data/input_small_mix.txt";
    day4(filepath);
}

// TODO: finish this
fn task5(){
    let filepath = "src/day5/data/input.txt";
    day5(filepath);
}

// TODO: finish this
fn task6(){
    let filepath = "src/day6/data/input.txt";
    // day6(filepath);
}

fn task7(){
    let filepath = "src/day7/data/input.txt";
    day7(filepath);
}

fn main() {
    task7();
}
