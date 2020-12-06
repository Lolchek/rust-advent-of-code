use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String
}

impl Passport{
    fn is_valid(&self) -> bool{
        if self.byr == "" || self.iyr == "" || self.eyr == "" 
        || self.hgt == "" || self.hcl == "" || self.ecl == "" || self.pid == "" {
            println!("Field missing, passport: {:?}", self);
            return false;
        }

        // byr
        let byr_number: i32 = self.byr.parse().unwrap();
        if byr_number < 1920 || byr_number > 2002{

            println!("Invalid byr: {}, passport: {:?}", byr_number , self);
            return false;
        }
        
        // iyr
        let iyr_number: i32 = self.iyr.parse().unwrap();
        if iyr_number < 2010 || iyr_number > 2020{
            println!("Invalid iyr: {}, passport: {:?}", iyr_number , self);
            return false;
        }

        // eyr
        let eyr_number: i32 = self.eyr.parse().unwrap();
        if eyr_number < 2020 || eyr_number > 2030{
            println!("Invalid eyr: {}, passport: {:?}", eyr_number , self);
            return false;
        }

        // hgt
        let (height, unit) = self.hgt.split_at(self.hgt.len()-2);

        match unit {
            "in" => {
                let height: i32 = height.parse().unwrap();
                    if height<59 || height>76{
                    return false;
                }
            },
            "cm" => {
                let height: i32 = height.parse().unwrap();
                if height<150 || height>193{

                    println!("Invalid height: {}{}, passport: {:?}", height, unit , self);
                    return false;
                }
            },
            _ => {
                println!("Invalid height NO UNIT: {}{}, passport: {:?}", height, unit , self);
                return false;
            },
        }

        // hcl
        let hcl_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        let hcl: &str = &self.hcl;
        if !hcl_regex.is_match(&hcl){
            println!("Invalid hair color: {}, passport: {:?}", hcl, self);
            
            return false;
        }
        
        // ecl
        let possible_eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        let ecl: &str = &self.ecl;
        if !possible_eye_colors.contains(&ecl){
            println!("Invalid eye color: {}, passport: {:?}", ecl, self);
            return false;
        }

        //pid
        let pid_regex = Regex::new(r"^[0-9]{9}$").unwrap();
        let pid: &str = &self.pid;
        if !pid_regex.is_match(&pid){
            println!("Invalid pid: {}, passport: {:?}", pid, self);
            return false;
        }

        // println!("{:?}", self);
        return true;
    }
}

fn load_passports_from_file(filename: &str) -> Vec<Passport>{
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let data: Vec<&str> = contents.split("\n").collect();
    let mut passports: Vec<Passport> = vec![];

    let (mut byr, mut iyr, mut eyr, mut hgt,mut hcl,mut ecl,mut pid,mut cid) = ("","","","","","","","");
    for row in data.iter(){
        if *row == "" {
            passports.push(Passport{
                byr: byr.to_string(), 
                iyr: iyr.to_string(), 
                eyr: eyr.to_string(), 
                hgt: hgt.to_string(),
                hcl: hcl.to_string(),
                ecl: ecl.to_string(),
                pid: pid.to_string(),
                cid: cid.to_string()
            });
            byr = "";
            iyr = "";
            eyr = "";
            hgt = "";
            hcl = "";
            ecl = "";
            pid = "";
            cid = "";

        }
        let row_splitted: Vec<&str> = row.split_whitespace().collect();
        for element in row_splitted {
            let key_value_pair: Vec<&str> = element.split(":").collect();
            match key_value_pair[0]{
                "byr" => byr = key_value_pair[1],
                "iyr" => iyr = key_value_pair[1],
                "eyr" => eyr = key_value_pair[1],
                "hgt" => hgt = key_value_pair[1],
                "hcl" => hcl = key_value_pair[1],
                "ecl" => ecl = key_value_pair[1],
                "pid" => pid = key_value_pair[1],
                "cid" => cid = key_value_pair[1],
                _ => println!("Unknown field :(")
            }
        }
    }

    return passports;
}

pub fn day4(filepath: &str) {
    let passports = load_passports_from_file(filepath);

    let total = passports.len();
    let mut valid_passport_count = 0;
    for passport in passports.iter(){
        if passport.is_valid(){
            println!("VALID PASSPORT: {:?}", passport);
            valid_passport_count += 1;
        }
    }
    println!("Valid passport count: {}/{}", valid_passport_count, total);
}