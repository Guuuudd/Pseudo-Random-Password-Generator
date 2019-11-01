mod password;

extern crate termion;

use termion::{color, clear, cursor, event::Key, input::TermRead, raw::IntoRawMode};
use std::io::{Write, stdout, stdin};

fn main() {

       let stdin = stdin();
       let mut stdout = stdout().into_raw_mode().unwrap();

       println!("{color_lgreen}PSEUDO RANDOM PASSWORD GENERATOR", color_lgreen = color::Fg(color::LightGreen));

       println!("{cur}{color_lgreen}https://github.com/Guuuudd/Pseudo-Random-Password-Generator", cur = cursor::Goto(1, 3), color_lgreen = color::Fg(color::LightGreen));

       stdout.flush().unwrap();

       let mut x = 4;
       let mut y = 0;
       let mut edit1 = false;
       let mut edit2 = false;
       let mut edit3 = false;
       let mut edit4 = false;
       let mut edit5 = false;
       let mut edit6 = false;

       let mut lenght: usize = 32;
              println!("{cur}{cur_hide}{color_white}Lenght: {color_lenght}{len}", cur = cursor::Goto(1, 5),
                     cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_lenght = color::Fg(color::LightCyan), len = lenght);

       let mut numbers = true;
       println!("{cur}{cur_hide}{color_white}Numbers: {color_num}{num}", cur = cursor::Goto(1, 7),
              cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_num = color::Fg(color::LightCyan), num = numbers);

       let mut low_letters = true;
       println!("{cur}{cur_hide}{color_white}Lowercase letters: {color_ll}{ll}", cur = cursor::Goto(1, 9),
              cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_ll = color::Fg(color::LightCyan), ll = low_letters);

       let mut upp_letters = true;
       println!("{cur}{cur_hide}{color_white}Uppercase letters: {color_ul}{ul}", cur = cursor::Goto(1, 11),
              cur_hide = cursor::Hide, color_white = color::Fg(color::White),  color_ul = color::Fg(color::LightCyan), ul = upp_letters);

       let mut symbols = true;
       println!("{cur}{cur_hide}{color_white}Symbols: {color_sym}{sym}", cur = cursor::Goto(1, 13),
              cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_sym = color::Fg(color::LightCyan), sym = symbols);

       let strict = true;
       /*
       println!("{cur}{cur_hide}{color_white}Strict: {color_stri}{stri}", cur = cursor::Goto(1, 15),
              color_white = color::Fg(color::White), color_stri = color::Fg(color::LightCyan), stri = strict);
       */

       let mut pw_number: usize = 1;
       println!("{cur}{cur_hide}{color_white}How many passwords: {color_pwn}{pwn}", cur = cursor::Goto(1, 15),
              cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_pwn = color::Fg(color::LightCyan), pwn = pw_number);


       println!("{}{}Move between the options using DOWN arrow key.", cursor::Goto(1, 18), color::Fg(color::LightRed));
       println!("{}{}Select the values of the options using LEFT/RIGHT arrow keys.", cursor::Goto(1, 19), color::Fg(color::LightRed));
       println!("{}{}Generate the passwords (without saving) using G key.", cursor::Goto(1, 20), color::Fg(color::LightRed));
       println!("{}{}Generate and Save the passwords using S key.", cursor::Goto(1, 21), color::Fg(color::LightRed));
       println!("{}{}Exit the program using ESC key.", cursor::Goto(1, 22), color::Fg(color::LightRed));

       for c in stdin.keys() {
              match c.unwrap() {
              Key::Left      => {
                     match edit1 {
                            true => {
                                   match x { 
                                          1 => {
                                                 x = 10;
                                                 lenght = 80;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8     16     24     32     40     48     56     64     72     {color_arrow}>{color_lenght}{len} ", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          2 => {
                                                 x = 1;
                                                 lenght = 8;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_arrow}>{color_lenght}{len}      {color_gray}16     24     32     40     48     56     64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          3 => {
                                                 x = 2;
                                                 lenght = 16;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      {color_arrow}>{color_lenght}{len}     {color_gray}24     32     40     48     56     64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          4 => {
                                                 x = 3;
                                                 lenght = 24;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     {color_arrow}>{color_lenght}{len}     {color_gray}32     40     48     56     64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          5 => {
                                                 x = 4;
                                                 lenght = 32;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     {color_arrow}>{color_lenght}{len}     {color_gray}40     48     56     64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          6 => {
                                                 x = 5;
                                                 lenght = 40;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     32     {color_arrow}>{color_lenght}{len}     {color_gray}48     56     64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          7 => {
                                                 x = 6;
                                                 lenght = 48;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     32     40     {color_arrow}>{color_lenght}{len}     {color_gray}56     64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          8 => {
                                                 x = 7;
                                                 lenght = 56;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     32     40     48     {color_arrow}>{color_lenght}{len}     {color_gray}64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          9 => {
                                                 x = 8;
                                                 lenght = 64;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     32     40     48     56     {color_arrow}>{color_lenght}{len}     {color_gray}72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          10 => {
                                                 x = 9;
                                                 lenght = 72;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     32     40     48     56     64     {color_arrow}>{color_lenght}{len}     {color_gray}80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          _ => (),
                                   }
                            },
                            false => (),
                     }
                     match edit2 {
                            true => {
                                   match numbers {
                                          true => {
                                                 numbers = false;
                                                 println!("{cur}{cur_hide}{color_white}Numbers:     {color_gray}true     {color_arrow}>{color_num}{num}", cur = cursor::Goto(1, 7),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_num = color::Fg(color::Cyan), num = numbers, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          false => {
                                                 numbers = true;
                                                 println!("{cur}{cur_hide}{color_white}Numbers:     {color_arrow}>{color_num}{num}     {color_gray}false", cur = cursor::Goto(1, 7),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_num = color::Fg(color::Cyan), num = numbers, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                   }
                            },
                            false => (),
                     }
                     match edit3 {
                            true => {
                                   match low_letters {
                                          true => {
                                                 low_letters = false;
                                                 println!("{cur}{cur_hide}{color_white}Lowercase letters:     {color_gray}true     {color_arrow}>{color_ll}{ll}", cur = cursor::Goto(1, 9),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_ll = color::Fg(color::Cyan), ll = low_letters, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          false => {
                                                 low_letters = true;
                                                 println!("{cur}{cur_hide}{color_white}Lowercase letters:     {color_arrow}>{color_ll}{ll}     {color_gray}false", cur = cursor::Goto(1, 9),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_ll = color::Fg(color::Cyan), ll = low_letters, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                   }
                            }
                            false => (),
                     }
                     match edit4 {
                            true => {
                                   match upp_letters {
                                          true => {
                                                 upp_letters = false;
                                                 println!("{cur}{cur_hide}{color_white}Uppercase letters:     {color_gray}true     {color_arrow}>{color_up}{up}", cur = cursor::Goto(1, 11),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_up = color::Fg(color::Cyan), up = upp_letters, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          false => {
                                                 upp_letters = true;
                                                 println!("{cur}{cur_hide}{color_white}Uppercase letters:     {color_arrow}>{color_up}{up}     {color_gray}false", cur = cursor::Goto(1, 11),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_up = color::Fg(color::Cyan), up = upp_letters, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                   }
                            }
                            false => (),
                     }
                     match edit5 {
                            true => {
                                   match symbols {
                                          true => {
                                                 symbols = false;
                                                 println!("{cur}{cur_hide}{color_white}Symbols:     {color_gray}true     {color_arrow}>{color_sym}{sym}", cur = cursor::Goto(1, 13),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_sym = color::Fg(color::Cyan), sym = symbols, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          false => {
                                                 symbols = true;
                                                 println!("{cur}{cur_hide}{color_white}Symbols:     {color_arrow}>{color_sym}{sym}     {color_gray}false", cur = cursor::Goto(1, 13),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_sym = color::Fg(color::Cyan), sym = symbols, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                   }
                            },
                            false => (),
                     }
                     match edit6 {
                            true => {
                                   match pw_number {
                                          1 =>  {
                                                 pw_number = 10;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     4     5     6     7     8     9     {color_arrow}>{color_pwn}{pwn}", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          10 => {
                                                 pw_number = 9;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     4     5     6     7     8     {color_arrow}>{color_pwn}{pwn}     {color_gray}10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          9 => {
                                                 pw_number = 8;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     4     5     6     7     {color_arrow}>{color_pwn}{pwn}     {color_gray}9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          8 => {
                                                 pw_number = 7;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     4     5     6     {color_arrow}>{color_pwn}{pwn}     {color_gray}8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          7 => {
                                                 pw_number = 6;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     4     5     {color_arrow}>{color_pwn}{pwn}     {color_gray}7     8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          6 => {
                                                 pw_number = 5;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     4     {color_arrow}>{color_pwn}{pwn}     {color_gray}6     7     8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          5 => {
                                                 pw_number = 4;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     {color_arrow}>{color_pwn}{pwn}     {color_gray}5     6     7     8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          4 => {
                                                 pw_number = 3;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     {color_arrow}>{color_pwn}{pwn}     4     5     6     7     8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          3 => {
                                                 pw_number = 2;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     {color_arrow}>{color_pwn}{pwn}     {color_gray}3     4     5     6     7     8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          2 => {
                                                 pw_number = 1;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_arrow}>{color_pwn}{pwn}     {color_gray}2     3     4     5     6     7     8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          _ => (),
                                   }
                            },
                            false => (),
                     }
              }
              Key::Right     => {
                     match edit1 {
                            true => {
                                   match x {
                                          1 => {
                                                 x = 2;
                                                 lenght = 16;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      {color_arrow}>{color_lenght}{len}     {color_gray}24     32     40     48     56     64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          2 => {
                                                 x = 3;
                                                 lenght = 24;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     {color_arrow}>{color_lenght}{len}     {color_gray}32     40     48     56     64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          3 => {
                                                 x = 4;
                                                 lenght = 32;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     {color_arrow}>{color_lenght}{len}     {color_gray}40     48     56     64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          4 => {
                                                 x = 5;
                                                 lenght = 40;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     32     {color_arrow}>{color_lenght}{len}     {color_gray}48     56     64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          5 => {
                                                 x = 6;
                                                 lenght = 48;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     32     40     {color_arrow}>{color_lenght}{len}     {color_gray}56     64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          6 => {
                                                 x = 7;
                                                 lenght = 56;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     32     40     48     {color_arrow}>{color_lenght}{len}     {color_gray}64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          7 => {
                                                 x = 8;
                                                 lenght = 64;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     32     40     48     56     {color_arrow}>{color_lenght}{len}     {color_gray}72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          8 => {
                                                 x = 9;
                                                 lenght = 72;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     32     40     48     56     64     {color_arrow}>{color_lenght}{len}     {color_gray}80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          9 => {
                                                 x = 10;
                                                 lenght = 80;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     32     40     48     56     64     72     {color_arrow}>{color_lenght}{len}", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          10 => {
                                                 x = 1;
                                                 lenght = 8;
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_arrow}>{color_lenght}{len}      {color_gray}16     24     32     40     48     56     64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          _ => (),
                                   }
                            },
                            false => (),
                     }
                     match edit2 {
                            true => {
                                   match numbers {
                                          true => {
                                                 numbers = false;
                                                 println!("{cur}{cur_hide}{clear}{color_white}Numbers:     {color_gray}true     {color_arrow}>{color_num}{num}", cur = cursor::Goto(1, 7), clear = clear::CurrentLine,
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_num = color::Fg(color::Cyan), num = numbers, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          false => {
                                                 numbers = true;
                                                 println!("{cur}{cur_hide}{clear}{color_white}Numbers:     {color_arrow}>{color_num}{num}     {color_gray}false", cur = cursor::Goto(1, 7), clear = clear::CurrentLine,
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_num = color::Fg(color::Cyan), num = numbers, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                   }
                            },
                            false => (),
                     }
                     match edit3 {
                            true => {
                                   match low_letters {
                                          true => {
                                                 low_letters = false;
                                                 println!("{cur}{cur_hide}{color_white}Lowercase letters:     {color_gray}true     {color_arrow}>{color_ll}{ll}", cur = cursor::Goto(1, 9),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_ll = color::Fg(color::Cyan), ll = low_letters, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          false => {
                                                 low_letters = true;
                                                 println!("{cur}{cur_hide}{color_white}Lowercase letters:     {color_arrow}>{color_ll}{ll}     {color_gray}false", cur = cursor::Goto(1, 9),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_ll = color::Fg(color::Cyan), ll = low_letters, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                   }
                            },
                            false => (),
                     }
                     match edit4 {
                            true => {
                                   match upp_letters {
                                          true => {
                                                 upp_letters = false;
                                                 println!("{cur}{cur_hide}{color_white}Uppercase letters:     {color_gray}true     {color_arrow}>{color_up}{up}", cur = cursor::Goto(1, 11),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_up = color::Fg(color::Cyan), up = upp_letters, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          false => {
                                                 upp_letters = true;
                                                 println!("{cur}{cur_hide}{color_white}Uppercase letters:     {color_arrow}>{color_up}{up}     {color_gray}false", cur = cursor::Goto(1, 11),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_up = color::Fg(color::Cyan), up = upp_letters, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                   }
                            },
                            false => (),
                     }
                     match edit5 {
                            true => {
                                   match symbols {
                                          true => {
                                                 symbols = false;
                                                 println!("{cur}{cur_hide}{color_white}Symbols:     {color_gray}true     {color_arrow}>{color_sym}{sym}", cur = cursor::Goto(1, 13),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_sym = color::Fg(color::Cyan), sym = symbols, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          false => {
                                                 symbols = true;
                                                 println!("{cur}{cur_hide}{color_white}Symbols:     {color_arrow}>{color_sym}{sym}     {color_gray}false", cur = cursor::Goto(1, 13),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_sym = color::Fg(color::Cyan), sym = symbols, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                   }
                            },
                            false => (),
                     }
                     match edit6 {
                            true => {
                                   match pw_number {
                                          1 =>  {
                                                 pw_number = 2;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     {color_arrow}>{color_pwn}{pwn}     {color_gray}3     4     5     6     7     8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          2 => {
                                                 pw_number = 3;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     {color_arrow}>{color_pwn}{pwn}     {color_gray}4     5     6     7     8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          3 => {
                                                 pw_number = 4;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     {color_arrow}>{color_pwn}{pwn}     {color_gray}5     6     7     8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          4 => {
                                                 pw_number = 5;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     4     {color_arrow}>{color_pwn}{pwn}     {color_gray}6     7     8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          5 => {
                                                 pw_number = 6;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     4     5     {color_arrow}>{color_pwn}{pwn}     {color_gray}7     8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          6 => {
                                                 pw_number = 7;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     4     5     6     {color_arrow}>{color_pwn}{pwn}     {color_gray}8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          7 => {
                                                 pw_number = 8;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     4     5     6     7     {color_arrow}>{color_pwn}{pwn}     {color_gray}9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          8 => {
                                                 pw_number = 9;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     4     5     6     7     8     {color_arrow}>{color_pwn}{pwn}     {color_gray}10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          9 => {
                                                 pw_number = 10;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     4     5     6     7     8     9     {color_arrow}>{color_pwn}{pwn}", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          10 => {
                                                 pw_number = 1;
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_arrow}>{color_pwn}{pwn}     {color_gray}2     3     4     5     6     7     8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          _ => (),
                                   }
                            },
                            false => (),
                     }
              }
              Key::Down        => {
                     match y {
                            0 => {
                                   y = 1;
                                   edit1 = true;
                                   println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     {color_arrow}>{color_lenght}{len}     {color_gray}40     48     56     64     72     80", cur = cursor::Goto(1, 5),
                                          cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                            },
                            1 => {
                                   y = 2;
                                   edit1 = false;
                                   println!("{cur}{cur_hide}{clear}{color_white}Lenght: {color_lenght}{len}", cur = cursor::Goto(1, 5), clear = clear::CurrentLine,
                                          cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_lenght = color::Fg(color::LightCyan), len = lenght);
                                   edit2 = true;
                                   match numbers {
                                          true => {
                                          println!("{cur}{cur_hide}{color_white}Numbers:     {color_arrow}>{color_num}{num}     {color_gray}false ", cur = cursor::Goto(1, 7),
                                                 cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_num = color::Fg(color::Cyan), num = numbers, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          
                                          },
                                          false => {
                                          println!("{cur}{cur_hide}{color_white}Numbers:     {color_gray}true     {color_arrow}>{color_num}{num} ", cur = cursor::Goto(1, 7),
                                                 cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_num = color::Fg(color::Cyan), num = numbers, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                   }
                            },
                            2 => {
                                   y = 3;
                                   edit2 = false;
                                   println!("{cur}{cur_hide}{clear}{color_white}Numbers: {color_num}{num}", cur = cursor::Goto(1, 7), clear = clear::CurrentLine,
                                          cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_num = color::Fg(color::LightCyan), num = numbers);
                                   edit3 = true;
                                   match low_letters {
                                          true => {
                                          println!("{cur}{cur_hide}{color_white}Lowercase letters:     {color_arrow}>{color_ll}{ll}     {color_gray}false", cur = cursor::Goto(1, 9),
                                                 cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_ll = color::Fg(color::Cyan), ll = low_letters, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          false => {
                                          println!("{cur}{cur_hide}{color_white}Lowercase letters:     {color_gray}true     {color_gray}{color_arrow}>{color_ll}{ll}", cur = cursor::Goto(1, 9),
                                                 cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_ll = color::Fg(color::Cyan), ll = low_letters, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                   }
                            },
                            3 => {
                                   y = 4;
                                   edit3 = false;
                                   println!("{cur}{cur_hide}{clear}{color_white}Lowercase letters: {color_ll}{ll}", cur = cursor::Goto(1, 9), clear = clear::CurrentLine,
                                          cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_ll = color::Fg(color::LightCyan), ll = low_letters);
                                   edit4 = true;
                                   match upp_letters {
                                          true => {
                                          println!("{cur}{cur_hide}{color_white}Uppercase letters:     {color_arrow}>{color_up}{up}     {color_gray}false", cur = cursor::Goto(1, 11),
                                                 cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_up = color::Fg(color::Cyan), up = upp_letters, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          false => {
                                          println!("{cur}{cur_hide}{color_white}Uppercase letters:     {color_gray}true     {color_gray}{color_arrow}>{color_up}{up}", cur = cursor::Goto(1, 11),
                                                 cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_up = color::Fg(color::Cyan), up = upp_letters, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                   }
                            },
                            4 => {
                                   y = 5;
                                   edit4 = false;
                                   println!("{cur}{cur_hide}{clear}{color_white}Uppercase letters: {color_ul}{ul}", cur = cursor::Goto(1, 11), clear = clear::CurrentLine,
                                          cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_ul = color::Fg(color::LightCyan), ul = upp_letters);
                                   edit5 = true;
                                   match symbols {
                                          true => {
                                          println!("{cur}{cur_hide}{color_white}Symbols:     {color_arrow}>{color_sym}{sym}     {color_gray}false", cur = cursor::Goto(1, 13),
                                                 cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_sym = color::Fg(color::Cyan), sym = symbols, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          false => {
                                          println!("{cur}{cur_hide}{color_white}Symbols:     {color_gray}true     {color_gray}{color_arrow}>{color_sym}{sym}", cur = cursor::Goto(1, 13),
                                                 cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_sym = color::Fg(color::Cyan), sym = symbols, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                   }
                            },
                            5 => {
                                   y = 6;
                                   edit5 = false;
                                   println!("{cur}{cur_hide}{clear}{color_white}Symbols: {color_sym}{sym}", cur = cursor::Goto(1, 13), clear = clear::CurrentLine,
                                          cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_sym = color::Fg(color::LightCyan), sym = symbols);
                                   edit6 = true;
                                   match pw_number {
                                          1 => {
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_arrow}>{color_pwn}{pwn}     {color_gray}2     3     4     5     6     7     8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          2 => {
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     {color_arrow}>{color_pwn}{pwn}     {color_gray}3     4     5     6     7     8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          3 => {
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     {color_arrow}>{color_pwn}{pwn}     {color_gray}4     5     6     7     8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          4 => {
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     {color_arrow}>{color_pwn}{pwn}     {color_gray}5     6     7     8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          5 => {
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     4     {color_arrow}>{color_pwn}{pwn}     {color_gray}6     7     8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          6 => {
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     4     5     {color_arrow}>{color_pwn}{pwn}     {color_gray}7     8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          7 => {
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     4     5     6     {color_arrow}>{color_pwn}{pwn}     {color_gray}8     9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          8 => {
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     4     5     6     7     {color_arrow}>{color_pwn}{pwn}     {color_gray}9     10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          9 => {
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     4     5     6     7     8     {color_arrow}>{color_pwn}{pwn}     {color_gray}10", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          10 => {
                                                 println!("{cur}{cur_hide}{color_white}How many passwords:     {color_gray}1     2     3     4     5     6     7     8     9     {color_arrow}>{color_pwn}{pwn}", cur = cursor::Goto(1, 15),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_arrow = color::Fg(color::LightCyan), color_pwn = color::Fg(color::Cyan), pwn = pw_number, color_gray = color::Fg(color::Rgb(169, 169, 169)));
                                          },
                                          _ => (),
                                   }
                            },
                            6 => {
                                   y = 1;
                                   edit6 = false;
                                   println!("{cur}{cur_hide}{clear}{color_white}How many passwords: {color_pwn}{pwn}", cur = cursor::Goto(1, 15), clear = clear::CurrentLine,
                                          cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_pwn = color::Fg(color::Cyan), pwn = pw_number);
                                   edit1 = true;
                                   match lenght {
                                          8 => {
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_arrow}>{color_lenght}{len}      {color_gray}16     24     32     40     48     56     64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          16 => {
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      {color_arrow}>{color_lenght}{len}     {color_gray}24     32     40     48     56     64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          },
                                          24 => {
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     {color_arrow}>{color_lenght}{len}     {color_gray}32     40     48     56     64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          }
                                          32 =>  {
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     {color_arrow}>{color_lenght}{len}     {color_gray}40     48     56     64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          
                                          },
                                          40 => {
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     32     {color_arrow}>{color_lenght}{len}     {color_gray}48     56     64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          
                                          },
                                          48 => {
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     32     40     {color_arrow}>{color_lenght}{len}     {color_gray}56     64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          
                                          },
                                          56 => {
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     32     40     48     {color_arrow}>{color_lenght}{len}     {color_gray}64     72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          
                                          },
                                          64 => {
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     32     40     48     56     {color_arrow}>{color_lenght}{len}     {color_gray}72     80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          
                                          },
                                          72 => {
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     32     40     48     56     64     {color_arrow}>{color_lenght}{len}     {color_gray}80", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          
                                          },
                                          80 => {
                                                 println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     32     40     48     56     64     72     {color_arrow}>{color_lenght}{len} ", cur = cursor::Goto(1, 5),
                                                        cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                                          
                                          },
                                          _ => (),
                                   }
                            },
                            _ => (),
                     }
              }
              Key::Up      => {
                     match y {
                            0 => {
                                   y = 1;
                                   edit1 = true;
                                   println!("{cur}{cur_hide}{color_white}Lenght:     {color_gray}8      16     24     {color_arrow}>{color_lenght}{len}     {color_gray}40     48     56     64     72     80", cur = cursor::Goto(1, 5),
                                          cur_hide = cursor::Hide, color_white = color::Fg(color::White), color_gray = color::Fg(color::Rgb(169, 169, 169)), color_arrow = color::Fg(color::LightCyan), color_lenght = color::Fg(color::Cyan), len = lenght);
                            },
                            _ => (),
                     }
              }
              Key::Char('g') => password::gen(lenght, numbers, low_letters, upp_letters, symbols, strict, pw_number),
              Key::Char('s') => password::gen_save(lenght, numbers, low_letters, upp_letters, symbols, strict, pw_number),
              Key::Char('G') => password::gen(lenght, numbers, low_letters, upp_letters, symbols, strict, pw_number),
              Key::Char('S') => password::gen_save(lenght, numbers, low_letters, upp_letters, symbols, strict, pw_number),
              Key::Esc       => break,
              _              => (),
              }
              stdout.flush().unwrap();
       }

}