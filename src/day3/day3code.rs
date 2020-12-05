use std::fs;

#[derive(Debug)]
struct TreeMap {
    map: Vec<Vec<char>>,
    len_row: i32,
    len_col: i32,
}

impl TreeMap{
    fn get_point(&self, row: i32, column: i32) -> char{
        let mut column_index: i32 = column;
        if column >= self.len_row {
            column_index = column % self.len_row
        }
        return self.map[row as usize][column_index as usize]
    }

    fn is_end(&self, row: i32) -> bool{
        if row >= self.len_col {return true;}
        return false;
    }

    fn is_tree(character: char) -> bool{
        if character == '#' {
            return true;
        }
        return false;
    }
}

fn build_map_from_file(filename: &str) -> TreeMap{
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    // println!("{}", contents);
    let data: Vec<&str> = contents.split_whitespace().collect();

    let mut len_row: i32 = 0;
    let len_col: i32 = data.len() as i32;

    let mut map_vec: Vec<Vec<char>> = vec![];

    for row in data {
        let r: Vec<char> = row.chars().collect();
        len_row = row.len() as i32;
        map_vec.push(r);
    }

    return TreeMap{
        map: map_vec, 
        len_row: len_row,
        len_col: len_col
    };
}

fn run_with_steps(map: &TreeMap, right: i32, down: i32) -> i32{
    let mut row = 0;
    let mut col = 0;

    let mut current_char;
    let mut number_of_trees = 0;

    loop {
        row += down;
        col += right;

        if map.is_end(row){
            break;
        }
        current_char = map.get_point(row, col);
        if TreeMap::is_tree(current_char){
            number_of_trees += 1;
        }
    }

    return number_of_trees;
}

pub fn day3(filename: &str) {
    let map = build_map_from_file(filename);

    let oneone = run_with_steps(&map, 1, 1);
    let threeone = run_with_steps(&map, 3, 1);
    let fiveone = run_with_steps(&map, 5, 1);
    let sevenone = run_with_steps(&map, 7, 1);
    let onetwo = run_with_steps(&map, 1, 2);

    println!("{}", oneone*threeone*fiveone*sevenone*onetwo);

    


}