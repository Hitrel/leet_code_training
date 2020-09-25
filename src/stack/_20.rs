impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::<Brackets>::new();

        for c in s.chars() {
            match Brackets::from(c) {
                Brackets(Towards::Right) => { stack.}
            }
        }

        false
    }
}

enum Towards {
    Left,
    Right,
}

enum Brackets {
    Parentheses(Towards),
    Braces(Towards),
    SquareBrackets(Towards),
    None,
}

impl From<char> for Brackets {
    fn from(c: char) -> Self {
        match c {
            '(' => Brackets::Parentheses(Towards::Right),
            ')' => Brackets::Parentheses(Towards::Left),
            '[' => Brackets::SquareBrackets(Towards::Right),
            ']' => Brackets::SquareBrackets(Towards::Left),
            '{' => Brackets::Braces(Towards::Right),
            '}' => Brackets::Braces(Towards::Left),
            _ => Brackets::None,
        }
    }

}