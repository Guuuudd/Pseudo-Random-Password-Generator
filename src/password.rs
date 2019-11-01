extern crate passwords;
extern crate termion;

use passwords::PasswordGenerator;
use termion::{color, clear, cursor};
use std::fs::File;
use std::io::prelude::*;

pub fn gen (_lenght: usize, _numbers: bool, _lowercase_letters: bool, _uppercase_letters: bool, _symbols: bool, _strict: bool, _how_many: usize) {

    let pg = PasswordGenerator {
        length: _lenght,
        numbers: _numbers,
        lowercase_letters: _lowercase_letters,
        uppercase_letters: _uppercase_letters,
        symbols: _symbols,
        strict: _strict,
    };

    let _x = pg.generate(_how_many).unwrap();
    
    let mut b = 1;

    println!("{cur}{clear}{color_lgreen}Passwords generated:", cur = cursor::Goto(1, 25), clear = clear::CurrentLine, color_lgreen = color::Fg(color::LightGreen));

    for _clearlines in 1..12{
        println!("{clear}", clear = clear::CurrentLine);
    }

    for a in &_x {
        println!("{cur}{clear}{color_lcyan}{pw}", cur = cursor::Goto(1, 26 + b), clear = clear::CurrentLine, color_lcyan = color::Fg(color::LightCyan), pw = a);
        b += 1;
    }

    println!("{cur}{color_reset}", cur = cursor::Goto(1, 27 + b), color_reset = color::Fg(color::Reset));

}


pub fn gen_save (_lenght: usize, _numbers: bool, _lowercase_letters: bool, _uppercase_letters: bool, _symbols: bool, _strict: bool, _how_many: usize) {

    let pg = PasswordGenerator {
        length: _lenght,
        numbers: _numbers,
        lowercase_letters: _lowercase_letters,
        uppercase_letters: _uppercase_letters,
        symbols: _symbols,
        strict: _strict,
    };

    let _x = pg.generate(_how_many).unwrap();
    
    let mut b = 1;

    println!("{cur}{clear}{color_lgreen}Passwords generated and saved:", cur = cursor::Goto(1, 25), clear = clear::CurrentLine, color_lgreen = color::Fg(color::LightGreen));

    for _clearlines in 1..12{
        println!("{clear}", clear = clear::CurrentLine);
    }

    for a in &_x {
        println!("{cur}{clear}{color_lcyan}{pw}", cur = cursor::Goto(1, 26 + b), clear = clear::CurrentLine, color_lcyan = color::Fg(color::LightCyan), pw = a);
        b += 1;
    }

    println!("{cur}{color_reset}", cur = cursor::Goto(1, 27 + b), color_reset = color::Fg(color::Reset));

    create_write(&_x);

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