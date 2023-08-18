use rand::Rng;
use std::env;
use colored::*;

#[derive(Debug)]
struct Roll {
    amount: i32,
    sides: i32,
    modif: i32,
}

impl Roll {
    fn parse(scanner: &mut Scanner) -> Self {
        let mut roll: Roll = Roll {amount: 0, sides: 0, modif: 0};
        let mut sign: Option<char> = None;
        loop {
            match scanner.pop() {
                Some(character) => {
                    if !character.is_numeric() {
                        match character {
                            &'d' => {
                                roll.amount = scanner.extract();
                                if roll.amount < 0 {
                                    println!("Error: Negative number of dice");
                                    roll.amount = 0;
                                }
                            }
                            &'\n' => {
                                if sign.is_some() {
                                    roll.modif = scanner.extract();
                                    if sign == Some('-') {
                                        roll.modif = -roll.modif;
                                    }
                                } else {
                                    roll.sides = scanner.extract();
                                }
                            }
                            &'+'|&'-' => {
                                sign = Some(*character);
                                roll.sides = scanner.extract();
                            }
                            _ => (),
                        }
                    }
                }
                None => {
                    if sign.is_some() {
                    roll.modif = scanner.extract();
                        if sign == Some('-') {
                            roll.modif = -roll.modif;
                        }
                    } else {
                        roll.sides = scanner.extract();
                    }
                    break;
                }
            }
        }
        roll
    }

    fn throw(&self) -> i32 {
        println!("Rolling!");
        let mut rng = rand::thread_rng();
        let mut sum = 0;
        for _i in 0..self.amount {
            let die = rng.gen_range(1..self.sides+1);
            let dice_str = format!(" {} ", die);
            print!(" {} ", dice_str.bold().on_red());
            sum += die;
        }
        if self.modif != 0 {
            print!("{}", format!("{:+}",self.modif).bold().blue());
        }
        sum + self.modif
    }
}

struct Scanner {
    cursor: usize,
    characters: Vec<char>,
}

impl Scanner {
    fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            characters: string.chars().collect()
        }
    }
    
    // returns next character without advancing the cursor
    fn peek(&self) -> Option<&char> {
        self.characters.get(self.cursor)
    }

    // returns next character and advances the cursor
    fn pop(&mut self) -> Option<&char> {
        match self.characters.get(self.cursor) {
            Some(character) => {
                self.cursor += 1;
                Some(character)
            }
            None => None
        }
    }

    // returns number up to current cursor position 
    // and removes it from characters vector
    // returns 0 on error
    fn extract(&mut self) -> i32 {
        let num_str : String;
        if self.peek().is_none() {
            num_str = self.characters[0..self.cursor].iter().collect();
        } else {
            if self.cursor-1 != 0 {
                num_str = self.characters[0..self.cursor-1].iter().collect();
            } else {
                return 0;
            }
        }
        self.characters = self.characters[self.cursor..].to_vec();
        self.cursor = 0;
        num_str.parse().unwrap_or(0)
    }
    
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut scanner = Scanner::new(&args[1]);
    
    let roll = Roll::parse(&mut scanner);
    println!("\n= {}", roll.throw());
}
