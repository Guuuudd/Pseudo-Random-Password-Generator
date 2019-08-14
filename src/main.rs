extern crate passwords;

use std::io;
use passwords::PasswordGenerator;
use std::fs::File;
use std::io::prelude::*;

fn main(){

        println!("\nPSEUDO RANDOM PASSWORD GENERATOR\n");
        println!("The generated passwords will be saved (as a .TXT file) in the folder where the generator is.\n");

    loop {
        
        println!("\nHow many characters do you want in your password(s)? [number]");

        let mut len = String::new();

        io::stdin().read_line(&mut len).expect("Error");

        let len = len.trim().parse::<usize>().unwrap();

        if len == 0 { println!("\nInvalid operation, you need to insert a valid number!"); continue; }


        println!("\nDoes the password need to contain numbers? [true/false]");

        let mut num = String::new();

        io::stdin().read_line(&mut num).expect("Error");

        if num.trim() != "true" && num.trim() != "false" { println!("\nInvalid operation, you need to insert true or false!"); continue; }

        let num = num.trim().parse::<bool>().unwrap();


        println!("\nDoes the password need to contain symbols? [true/false]");

        let mut sym = String::new();

        io::stdin().read_line(&mut sym).expect("Error");

        if sym.trim() != "true" && sym.trim() != "false" { println!("\nInvalid operation, you need to insert true or false!"); continue; }

        let sym = sym.trim().parse::<bool>().unwrap();


        println!("\nDoes the password need to contain lowercase characters? [true/false]");

        let mut low = String::new();

        io::stdin().read_line(&mut low).expect("Error");

        if low.trim() != "true" && low.trim() != "false" { println!("\nInvalid operation, you need to insert true or false!"); continue; }

        let low = low.trim().parse::<bool>().unwrap();


        println!("\nDoes the password need to contain uppercase characters? [true/false]");

        let mut upp = String::new();

        io::stdin().read_line(&mut upp).expect("Error");

        if upp.trim() != "true" && upp.trim() != "false" { println!("\nInvalid operation, you need to insert true or false!"); continue; }

        let upp = upp.trim().parse::<bool>().unwrap();


        println!("\nHow many passwords do you need? [number]");

        let mut pw_num = String::new();

        io::stdin().read_line(&mut pw_num).expect("Error");

        let pw_num = pw_num.trim().parse::<usize>().unwrap();

        if pw_num == 0 { println!("\nInvalid operation, you need to insert a positive number!"); continue; }


        let pg = PasswordGenerator {
            length: len,
            numbers: num,
            lowercase_letters: low,
            uppercase_letters: upp,
            symbols: sym,
            strict: true,
        };

        let xy = pg.generate(pw_num).unwrap();

        create_write(&xy);

        break;

    }

}

fn create_write(text: &Vec<String>) {

    let pg = PasswordGenerator {
        length: 10,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: false,
        strict: false,
    };

    let x = pg.generate_one().unwrap();
    let y = String::from(".txt");

    let mut file = File::create(x + &y).unwrap();

    for a in text {
        writeln!(&mut file, "{}\n", a).unwrap();
    }   

}
