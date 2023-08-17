struct Roll {
    amount: usize,
    sides: usize,
    modif: usize
}

struct Scanner {
    cursor: usize,
    characters: Vec<char>
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
    
}

fn main() {
    let reader = std::io::stdin();
    
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let scanner = Scanner::new(&input);
    
    println!("{}", amount);
}
