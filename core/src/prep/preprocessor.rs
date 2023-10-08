pub struct PreProcessor {
    input: String,
    position: usize,
}

impl PreProcessor {
    fn new(input: String) -> Self {
        PreProcessor {
            input,
            position: 0,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn get(&self) -> char {
        // The ambiguidy is resolved by unwrap_or_else(|_| 0)
        self.input.chars().nth(self.pos()).unwrap() // for the sake of lazy evaluation
    }

    fn index(&self, index: isize) -> char {
        // The ambiguidy is resolved by unwrap_or_else(|_| 0)
        self.input
            .chars()
            .nth((TryInto::<isize>::try_into(self.pos()).expect("usize & isize Overflow") as isize + index) as usize)
            .unwrap()
    }

    fn get_exact(&self, index: usize) -> char {
        // The ambiguidy is resolved by unwrap_or_else(|_| 0)
        self.input.chars().nth(index).unwrap() // for the sake of lazy evaluation
    }

    fn pos(&self) -> usize {
        self.position
    }

    fn check(&self, lit: &str) -> bool {
        let mut index = 0isize;
        let mut lit_chars = lit.chars();
        for c in lit_chars {
            if c != self.index(index) {
                return false;
            }
            index += 1;
        }
        true
    }
}

