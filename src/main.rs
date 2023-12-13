use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }

    /*for line in content{
        print!("{line}")
    }*/
    //let content_secon = fs::read(path).expect("non stai leggendo bene il file");
    //println!("{content}");
    //for i in content_secon{
    //    println!("{i}")
    //}
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
