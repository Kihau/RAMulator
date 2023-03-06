// Line is expected to be:
// (multiple labels) (instruction) (comment)
//  ^ single string   ^ string list   ^ single string
// All are optional 
//
// Instruction expected:
// (opcode) (value_string) (...)
//  other arguments should ^^^^^ be an error
// TODO: Just hold immutable str ?
#[derive(Debug)]
pub enum Token {
    Label(String),
    InstrName(String),
    InstrType(char),
    InstrValue(String),
    Comment(String),
    NewLine
}

#[derive(Default)]
pub struct NewParser {
    pub tokens: Vec<Token>,
}

impl NewParser {
    fn tokenize_word(&mut self, mut word: String, found_opcode: &mut bool) {
        if word.is_empty() {
            return;
        }

        if word.ends_with(':') {
            word.pop();
            self.tokens.push(Token::Label(word))
        } else if !*found_opcode {
            *found_opcode = true;
            self.tokens.push(Token::InstrName(word))
        } else {
            self.tokens.push(Token::InstrValue(word))
        }
    }

    pub fn parse_line(&mut self, line: &str) {
        let mut found_opcode = false;

        let mut word = String::new();
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            match c {
                ';' => {
                    let comment = String::from(chars.as_str());
                    self.tokens.push(Token::Comment(comment));
                    break;
                }
                '=' | '*' if word.is_empty() => self.tokens.push(Token::InstrType(c)),
                _ if c.is_whitespace() => {
                    self.tokenize_word(word, &mut found_opcode);
                    word = String::new();
                }
                _ => word.push(c),
            }
        }

        self.tokenize_word(word, &mut found_opcode);
    }

    pub fn parse_source(&mut self, source: &str) {
        for line in source.lines() {
            self.parse_line(line);
            self.tokens.push(Token::NewLine);
        }
    }
}
