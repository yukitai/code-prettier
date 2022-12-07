use std::{collections::HashMap, rc::Rc};
use regex::Regex;
use serde::{Deserialize};
use serde_json::Result;

#[derive(Deserialize, Debug, Clone)]
struct PatternRegex {
    name: String,
    regex: String,
}

#[derive(Deserialize, Debug, Clone)]
struct PatternBeginEnd {
    name: String,
    begin: String,
    end: String,
    include_first: bool,
    include_end: bool,
}

#[derive(Deserialize, Debug)]
pub struct LangPatterns {
    color_map: HashMap<String, String>,
    pattern_regex: Vec<PatternRegex>,
    pattern_begin_end: Vec<PatternBeginEnd>,
}

impl LangPatterns {
    pub fn try_parse(json: &str) -> Result<LangPatterns> {
        serde_json::from_str(json)
    }
}

pub struct LangHighlighter {
    regex_map: HashMap<String, Regex>,
    lang_patterns: LangPatterns,
}

impl LangHighlighter {
    pub fn try_parse(json: &str) -> Result<LangHighlighter> {
        Ok(LangHighlighter {
            regex_map: HashMap::new(),
            lang_patterns: LangPatterns::try_parse(json)?,
        })
    }
    #[inline]
    fn regex(&mut self, regex: &String) -> &Regex {
        if !self.regex_map.contains_key(regex) {
            let reg = Regex::new(regex.as_str()).unwrap();
            self.regex_map.insert(regex.clone(), reg);
        }
        self.regex_map.get(regex).unwrap()
    }
    pub fn try_getcolor(&self, color: Rc<String>) -> Option<&String> {
        self.lang_patterns.color_map.get(&*color)
    }
    #[inline]
    fn regex_patterns(&mut self, matched: &mut Vec<Rc<String>>, token: &str) {
        for pattern in self.lang_patterns.pattern_regex.clone() {
            let reg = self.regex(&pattern.regex);
            if reg.is_match(token) {
                matched.push(Rc::new(pattern.name));
            }
        }
    }
    /* #[inline]
    fn begin_end_patterns(&mut self, matched: &mut Vec<Rc<String>>, token: &str) {
        for pattern in self.lang_patterns.pattern_begin_end.clone() {
            let reg_begin = self.regex(&pattern.begin);
            if !reg_begin.is_match(token) {
                continue
            }
            let reg_end = self.regex(&pattern.end);
            if reg_end.is_match(token) {
                matched.push(Rc::new(pattern.name));
            }
        }
    } */
    #[inline]
    fn begin_patterns2(&mut self, matched: &mut Vec<(Rc<String>, usize)>, token: &str) {
        for (i, pattern) in self.lang_patterns.pattern_begin_end.clone().iter().enumerate() {
            let reg = self.regex(&pattern.begin);
            if reg.is_match(token) {
                matched.push((Rc::new(pattern.name.clone()), i));
            }
        }
    }
    /* #[inline]
    fn end_patterns2(&mut self, matched: &mut Vec<Rc<String>>, token: &str) {
        for pattern in self.lang_patterns.pattern_begin_end.clone() {
            let reg = self.regex(&pattern.end);
            if reg.is_match(token) {
                matched.push(Rc::new(pattern.name));
            }
        }
    } */
    pub fn patterns(&mut self, token: &str) -> Vec<Rc<String>> {
        let mut matched = Vec::new();
        // self.begin_end_patterns(&mut matched, token);
        self.regex_patterns(&mut matched, token);
        matched
    }
    pub fn begin_patterns(&mut self, token: &str) -> Vec<(Rc<String>, usize)> {
        let mut matched = Vec::new();
        self.begin_patterns2(&mut matched, token);
        matched
    }
    pub fn end_pattern(&mut self, token: &str, pattern_id: usize) -> bool {
        let reg = self.regex(&self.lang_patterns.pattern_begin_end[pattern_id].clone().end);
        reg.is_match(token)
    }
    pub fn include_first(&self, pattern_id: usize) -> bool {
        self.lang_patterns.pattern_begin_end[pattern_id].clone().include_first
    }
    pub fn include_end(&self, pattern_id: usize) -> bool {
        self.lang_patterns.pattern_begin_end[pattern_id].clone().include_end
    }
}