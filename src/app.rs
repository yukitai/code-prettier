use std::fmt::Display;
use std::rc::Rc;
use std::{env, collections::HashMap, fs};
use colored::Colorize;
use regex::Regex;
use serde::Deserialize;
use crate::logger::Logger;
use crate::tokenizer::Tokenizer;
use crate::highlighting::Highlighter;
use crate::language_pattern::LangHighlighter;

lazy_static! {
    static ref SPACE_CHAR: String = String::from(" ");
}

macro_rules! color_map {
    ( $( $k: ident : $r: literal $g: literal $b: literal ) , * , ) => {
        HashMap::from([
            $ ( 
                (stringify!($k), ($r as u8, $g as u8, $b as u8)),
            ) *
        ])
    };
}

macro_rules! colorize {
    ( $self : expr , $str : expr , $color : expr ) => {
        {
            let color = $self.config.color_map.get($color).unwrap();
            $str.truecolor(color.0, color.1, color.2)
        }
    };
}

fn load_file(file_path: &String) -> Result<Vec<u8>, String> {
    if let Ok(bytes) = fs::read(file_path) {
        Ok(bytes)
    } else {
        Err(format!("File IO Error"))
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum LogLevel {
    Never = 3,
    Error = 2,
    Warn = 1,
    All = 0,
}

pub struct Arguments {
    file_paths: Vec<String>,
    log_level: LogLevel,
}

impl Arguments {
    pub fn from_env() -> Result<Arguments, String> {
        let args: Vec<String> = env::args().collect();
        let mut log_level = LogLevel::Error;
        let mut file_paths = Vec::new();
        let mut curr = 1;
        while curr < args.len() {
            if args[curr].eq("--log-level") {
                curr += 1;
                if curr >= args.len() {
                    return Err(format!("Expect log level after `--log-level`."))
                }
                match args[curr].as_str() {
                    "0" | "all" => {
                        log_level = LogLevel::All;
                    },
                    "1" | "warn" => {
                        log_level = LogLevel::Warn;
                    },
                    "2" | "error" => {
                        log_level = LogLevel::Error;
                    },
                    "3" | "never" => {
                        log_level = LogLevel::Never;
                    },
                    _ => {},
                }
            } else {
                file_paths.push(args[curr].clone());
            }
            curr += 1;
        }
        Ok(Arguments {
            file_paths,
            log_level,
        })
    }
}

pub struct Config {
    color_map: HashMap<&'static str, (u8, u8, u8)>,
}

impl Config {
    pub fn from_file(path: String) -> Config {
        let color_map = color_map! {
            title           : 255 107 107,
            file_path       : 107 107 255,
            keyword         : 255 107 107,
            literal_string  : 107 255 107,
            literal_number  : 255 255 107,
            literal_boolean : 107 255 255,
            type            : 255 107 255,
            nextline        : 155 155 155,
            note            : 155 155 155,
            /* info      : 255 107 107,
            note      : 107 107 255,
            warn      : 255 107 107,
            error     : 255 107 107, */ 
        }; // default config
        Config {
            color_map, 
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    token: String,
    range: (usize, usize),
    color: Rc<String>,
}

impl Token {
    pub fn new(token: String, range: (usize, usize)) -> Token {
        Token { token, range, color: Rc::new(String::from("unknown")) }
    }
    pub fn colored(&self) -> bool {
        self.color.as_str() != "unknown"
    }
    pub fn color(&mut self, color: Rc<String>) {
        self.color = color;
    }
    pub fn as_str(&self) -> &str {
        &self.token
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token)
    }
}

lazy_static! {
    static ref RE_FILEEXT: Regex = Regex::new(".([a-zA-Z0-9]+)$").unwrap();
    static ref STR_DEFAULT: String = String::from("default");
}

#[derive(Deserialize)]
struct LanguageMap {
    default: String,
    highlighter_map: HashMap<String, String>,
}

impl LanguageMap {
    fn from_file_path(&self, root: &String, path: &String) -> String {
        let mut highlighting_filename;
        match RE_FILEEXT.captures(path) {
            Some(extname) => {
                let default_extname = &extname[1].to_string();
                highlighting_filename = self.highlighter_map.get(&extname[1]).unwrap_or(default_extname);
                match std::fs::read_to_string(format!("{}/highlighting/{}.json", root, highlighting_filename)) {
                    Ok(res) => {
                        return res
                    },
                    Err(_) => {
                        highlighting_filename = &STR_DEFAULT;
                    }
                }
            },
            None => {
                highlighting_filename = &STR_DEFAULT;
            }
        }
        std::fs::read_to_string(format!("{}/highlighting/{}.json", root, highlighting_filename)).unwrap()
    }
}

pub struct App {
    version: &'static str,
    root: String,
    language_map: LanguageMap,
    args: Arguments,
    config: Config,
}

impl App {
    pub fn new(version: &'static str, root: String, args: Arguments, config: Config) -> App {
        let language_map = serde_json::from_str(std::fs::read_to_string(root.clone() + "/highlighting/language_map.json").unwrap().as_str()).unwrap();
        App { version, root, language_map, args, config }
    }
    pub fn run(&self, nth: usize) -> Result<Logger, String> {
        if let Some(file_path) = self.args.file_paths.get(nth) {
            println!("{}", format!("{}{} - {}", colorize!(self, "Prettier@", "title"), colorize!(self, self.version, "title"), colorize!(self, file_path, "file_path")).bold());
            match load_file(file_path) {
                Ok(bytes) => {
                    let mut logger = Logger::new(self.args.log_level);
                    let mut tokenizer = Tokenizer::new(&mut logger, bytes);
                    let (tokens, lines) = tokenizer.tokenize();
                    let max_line_len = format!("{}", lines).len();
                    // println!("{:#?}", tokens);
                    let mut highlighter = Highlighter::new(&mut logger, tokens, LangHighlighter::try_parse(&self.language_map.from_file_path(&self.root, file_path)).unwrap());
                    let tokens = highlighter.color();
                    // println!("{:#?}", tokens);
                    let mut line: usize = 1;
                    print!("{}{}  ", colorize!(self, format!("{}", line), "nextline").bold(), SPACE_CHAR.repeat(max_line_len - 1));
                    for token in tokens {
                        match token.color.as_str() {
                            "default" | "unknown" => {
                                print!("{}", token);
                            },
                            "bold" => {
                                print!("{}", token.token.bold().bright_black());
                            },
                            "symbol" => {
                                print!("{}", token.token.italic().bright_black());
                            },
                            "nextline" => {
                                line += 1;
                                let line_len = format!("{}", line).len();
                                print!("\n{}{}  ", colorize!(self, format!("{}", line), "nextline").bold(), SPACE_CHAR.repeat(max_line_len - line_len));
                            },
                            _ => {
                                print!("{}", colorize!(self, token.token, token.color.as_str()));
                            }
                        }
                    }
                    println!();
                    Ok(logger)
                },
                Err(e) => Err(e),
            }
        } else {
            Err(format!(""))
        }
    }
}