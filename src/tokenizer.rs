use std::rc::Rc;

use crate::{app::Token, logger::{Logger, NoteFor}};

static OPERATORS: [ u8; 30 ] = [
    b'+', 
    b'-', 
    b'*', 
    b'/', 
    b'~', 
    b'`', 
    b'!', 
    b'@', 
    b'#', 
    b'$', 
    b'%', 
    b'^', 
    b'&', 
    b'*', 
    b'(', 
    b')', 
    b'=', 
    b'[', 
    b']', 
    b'{', 
    b'}', 
    b'|', 
    b';', 
    b':', 
    b'?', 
    b'.', 
    b',', 
    b'<', 
    b'>', 
    b'\\', 
];

static BLANKS: [ u8; 3 ] = [
    b' ',
    b'\t',
    b'\r',
];

static NEXTLINE: [ u8; 1 ] = [
    b'\n',
];

static STRING_SYMBOL: [ u8; 2 ] = [
    b'\'',
    b'"',
];

pub struct Tokenizer<'a> {
    logger: &'a mut Logger,
    bytes: Vec<u8>,
    curr: isize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(logger: &'a mut Logger, bytes: Vec<u8>) -> Tokenizer {
        Tokenizer { logger, bytes, curr: -1 }
    }
    #[inline]
    fn has_next(&self) -> bool {
        self.curr + 1 < self.bytes.len() as isize
    }
    #[inline]
    fn next(&mut self) -> u8 {
        self.curr += 1;
        *self.bytes.get(self.curr as usize).unwrap()
    }
    #[inline]
    fn peek(&self) -> u8 {
        *self.bytes.get(self.curr as usize + 1).unwrap()
    }
    #[inline]
    fn forward(&mut self) {
        self.curr += 1;
    }
    // the normal tokenizer
    pub fn tokenize(&mut self) -> (Vec<Token>, usize) {
        let mut tokens = Vec::new();
        let mut line = 1;
        while self.has_next() {
            let curr = self.next();
            if OPERATORS.contains(&curr) {
                let begin = self.curr as usize;
                while self.has_next() && OPERATORS.contains(&self.peek()) {
                    self.forward();
                }
                let vec: Vec<u8> = self.bytes[begin..=(self.curr as usize)].iter().cloned().collect();
                match String::from_utf8(vec) {
                    Ok(token_string) => {
                        let token = Token::new(token_string, (begin, self.curr as usize));
                        tokens.push(token);
                    },
                    Err(e) => {
                        self.logger.error(format!("UTF8 Err: Unexpected UTF8 Char."));
                        self.logger.info(format!("Raw Err: {}", e));
                    }
                }
            } else if BLANKS.contains(&curr) {
                let begin = self.curr as usize;
                while self.has_next() && BLANKS.contains(&self.peek()) {
                    self.forward();
                }
                let vec: Vec<u8> = self.bytes[begin..=(self.curr as usize)].iter().cloned().collect();
                match String::from_utf8(vec) {
                    Ok(token_string) => {
                        let token = Token::new(token_string, (begin, self.curr as usize));
                        tokens.push(token);
                    },
                    Err(e) => {
                        self.logger.error(format!("UTF8 Err: Unexpected UTF8 Char."));
                        self.logger.note(format!("Raw Err: {}", e), NoteFor::Error);
                    }
                }
            } else if NEXTLINE.contains(&curr) {
                let begin = self.curr as usize;
                let mut token = Token::new(String::from("\n"), (begin, self.curr as usize));
                token.color(Rc::new(String::from("nextline")));
                tokens.push(token);
                line += 1;
            } else if STRING_SYMBOL.contains(&curr) {
                let begin = self.curr as usize;
                let token = Token::new(String::from_utf8(vec! [curr]).unwrap(), (begin, self.curr as usize));
                tokens.push(token);
            } else {
                let begin = self.curr as usize;
                while self.has_next() && (!OPERATORS.contains(&self.peek()) && !BLANKS.contains(&self.peek())) {
                    self.forward();
                }
                let vec: Vec<u8> = self.bytes[begin..=(self.curr as usize)].iter().cloned().collect();
                match String::from_utf8(vec) {
                    Ok(token_string) => {
                        let token = Token::new(token_string, (begin, self.curr as usize));
                        tokens.push(token);
                    },
                    Err(e) => {
                        self.logger.error(format!("UTF8 Err: Unexpected UTF8 char."));
                        self.logger.note(format!("Raw Err: {}", e), NoteFor::Error);
                    }
                }
            }
            //self.forward();
        }
        (tokens, line)
    }
}
