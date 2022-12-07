use std::rc::Rc;

use crate::{app::Token, logger::{Logger, NoteFor}, language_pattern::LangHighlighter};

pub struct Highlighter<'a> {
    logger: &'a mut Logger,
    tokens: Vec<Token>,
    highlighter: LangHighlighter,
}

impl<'a> Highlighter<'a> {
    pub fn new(logger: &'a mut Logger, tokens: Vec<Token>, highlighter: LangHighlighter) -> Highlighter {
        Highlighter { logger, tokens, highlighter }
    }
    pub fn color(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut begin_color: Option<Rc<String>> = None;
        let mut begin_id = 0;
        for token in &self.tokens {
            if token.colored() {
                tokens.push(token.clone());
                continue
            }
            match &begin_color {
                Some(begin_c) => {
                    let matched = self.highlighter.end_pattern(token.as_str(), begin_id);
                    if matched {
                        if self.highlighter.include_end(begin_id) {
                            let mut colored_token = token.clone();
                            let color = self.highlighter.try_getcolor(begin_color.as_ref().unwrap().clone());
                            colored_token.color(Rc::new(color.unwrap_or(&*begin_color.as_ref().unwrap().clone()).to_string()));
                            tokens.push(colored_token);
                            begin_color = None;
                            continue
                        } else {
                            begin_color = None;
                        }
                    } else {
                        let mut colored_token = token.clone();
                        let color = self.highlighter.try_getcolor(begin_c.clone());
                        colored_token.color(Rc::new(color.unwrap_or(&*begin_c.clone()).to_string()));
                        tokens.push(colored_token);
                        continue
                    }
                },
                None => {
                    let matched = self.highlighter.begin_patterns(token.as_str());
                    let len = matched.len();
                    if len > 0 {
                        if len > 1 {
                            self.logger.warn(format!("There are more than 1 begin patterns can match the token `{}`.", token));
                            self.logger.note(format!("Matched patterns: {:?}", matched), NoteFor::Warn);
                        }
                        begin_color = Some(matched[0].0.clone());
                        begin_id = matched[0].1;
                        if self.highlighter.include_first(begin_id) {
                            let mut colored_token = token.clone();
                            let color = self.highlighter.try_getcolor(begin_color.as_ref().unwrap().clone());
                            colored_token.color(Rc::new(color.unwrap_or(&*begin_color.as_ref().unwrap().clone()).to_string()));
                            tokens.push(colored_token);
                            continue
                        }
                    }
                    
                },
            }
            let matched = self.highlighter.patterns(token.as_str());
            let len =  matched.len();
            if len == 0 {
                tokens.push(token.clone());
            } else {
                if len > 1 {
                    self.logger.warn(format!("There are more than 1 patterns can match the token `{}`.", token));
                    self.logger.note(format!("Matched patterns: {:?}", matched), NoteFor::Warn);
                }
                let mut colored_token = token.clone();
                let color = self.highlighter.try_getcolor(matched[0].clone());
                colored_token.color(Rc::new(color.unwrap_or(&*matched[0]).clone()));
                tokens.push(colored_token)
            }
        }
        tokens
    }
}
