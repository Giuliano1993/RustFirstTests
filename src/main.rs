use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


#[derive(Debug)]
struct Direction{
    bivio: [String; 2]
}
impl Direction{
    fn get_dir(&self, direction: char) -> String{

        match direction {
            'R' => self.bivio[1].clone(),
            'L' => self.bivio[0].clone(),
            _ => panic!("not a valid direction")
        }
    }
}

fn parse_line(mut input: &str) -> Direction{

    input = rem_first_and_last(input);

    let out  : Vec<&str> = input.split(", ").collect();
    let output_array : [String; 2] = [out[0].to_string(), out[1].to_string()];
    return Direction{
        bivio: output_array
    };
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    println!("Opening file {}", path);
    
    //let content = fs::read_to_string(path).expect("non stai leggendo bene il file");

    // unwrap gestisce all in one sia lo stato OK che il Panic di una determinata funzione
    // lines restituisce un iterator, quindi va looppato subito perché poi libera subito la memotia
    // lines non riguarda idrettamene la lettura dei file ma splitta una stringa secondo i break line


    /*for line in fs::read_to_string(path).unwrap().lines(){
        println!("{line}")
    };*/

    /*for line in read_lines(path){
        println!("{}",line);
    }*/
    let mut directions = String::new();

    let mut directions_change : HashMap<String, Direction> = Default::default();
    if let Ok(lines) = read_lines(path) {
        println!("{}",path);
        // Consumes the iterator, returns an (Optional) String
        for (i, line) in lines.enumerate() {
            if let Ok(doc_line) = line {
                if i == 0 {
                    directions = doc_line.clone();
                }else if i > 1 {
                    let splitted : Vec<&str> = doc_line.split(" = ").collect();
                    directions_change.insert(splitted[0].to_string(), parse_line(splitted[1]));   
                }
            }
        }
    }
    let final_steps = follow_path(directions, directions_change);
    println!("Steps for the end = {}",final_steps);
    //let content_secon = fs::read(path).expect("non stai leggendo bene il file");
    //println!("{content}");
    //for i in content_secon{
    //    println!("{i}")
    //}

}


fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

fn follow_path(directions: String, changes_map : HashMap<String, Direction>) -> usize{
    let mut next_step = String::from("AAA");
    let mut steps = 0;
    let mut direction: char;
    let mut possible_steps: &Direction;
    let mut dir_index = 0;
    while next_step != "ZZZ" {

        possible_steps = changes_map.get(&next_step).unwrap();

        direction = directions.chars().nth(dir_index).unwrap();

        next_step = possible_steps.get_dir(direction);


        steps = steps + 1;
        if dir_index < directions.len() -1 {
            dir_index = dir_index + 1;
        }else{
            dir_index = 0;
        }
        
    }
    return steps;
}


// -> the arrow express the return type of a function
// returning a value also gives ownership of the returned variable to the scope from whch it was called
// so the ownershp is transferred
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {

    let file = File::open(filename)?;
    //this is the short for: 
        /*let file = match File::open(filename){
            Ok(file)=>file,
            Err(e)=>return Err(e),
        };*/
    // questiokn mark implicitly manage the Ok and the Error pattern which are always present and handled

    // in rust, the return is implicit and what is returned is the last expression, like this
    // we can still return early in the function using the return keyworkd
    Ok(io::BufReader::new(file).lines())
}
