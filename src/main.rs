extern crate rand;
extern crate clap;

use rand::Rng;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// lenght of the password
    #[clap(short, long, default_value_t = 10)]
    lenght: u32,
    /// activates special characthers
    #[clap(short, long)]
    special_char: bool,
    /// activates numbers
    #[clap(short, long)]
    numbers: bool,
}


fn main() {
    let args = Args::parse();

    // define possible values for password
    let _letters =  ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let _special_char = ['/','#','(',')','-','_','%','+'];
    let _numbers = ['0','1','2','3','4','5','6','7','8','9'];

    // constructing possible values
    let mut possible_values: Vec<char> = Vec::new();
    // by default _letters
    for i in _letters {
        possible_values.push(i);
    }
    // if special_char
    if args.special_char {
        for i in _special_char {
            possible_values.push(i);
        }
    }
    // if numbers
    if args.numbers {
        for i in _numbers {
            possible_values.push(i);
        }
    }
    // generating password
    let mut rng = rand::thread_rng();
    let mut password = String::new();
    for _i in 1..=args.lenght {
        password.push(possible_values[rng.gen_range(1..possible_values.len())]);
    }
    println!("Generated password: {:?}", password);
}