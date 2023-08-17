struct Roll {
    amount: u32,
    sides: u32,
    modif: u32,
}

impl Roll {
    fn parse(scanner: &mut Scanner) -> Self {
        let mut roll: Roll = Roll {amount: 0, sides: 0, modif: 0};
        let mut sign: Option<char> = None;
        while scanner.peek().is_some(){
            match scanner.pop() {
                Some(character) =>{
                    if !character.is_numeric() {
                        match character {
                            &'d' => {
                                println!("Roll!");
                                roll.amount = scanner.extract();
                                println!("amount {}", roll.amount);
                            }
                            &'\n' => {
                                println!("Enter");
                                if sign.is_some() {
                                    roll.modif = scanner.extract();
                                    println!("roll modfi {}", roll.modif);
                                } else {
                                    roll.sides = scanner.extract();
                                    println!("roll sides {}", roll.sides);
                                }
                            }
                            &'+'|&'-' => {
                                println!("+ or -");
                                sign = Some(*character);
                                roll.sides = scanner.extract();
                                println!("roll sides {}", roll.sides);
                            }
                            _ => (),
                        }
                    }
                }
                None => println!("end of input"),
            }
        }
        roll
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
    fn extract(&mut self) -> u32 {
        let num_str: String = self.characters[0..self.cursor-1].iter().collect();
        self.characters = self.characters[self.cursor..].to_vec();
        self.cursor = 0;
        num_str.parse().unwrap_or(0)
    }
    
}

fn main() {
    let reader = std::io::stdin();
    
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut scanner = Scanner::new(&input);
    
    let roll = Roll::parse(&mut scanner);
}
