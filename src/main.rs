use std::io::Write;
use std::fmt;

#[derive(Debug)]
pub struct Frac {
   pub numerator: i128,
   pub denominator: i128
}

impl fmt::Display for Frac {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{}/{}", self.numerator, self.denominator) 
    }
}

#[derive(Debug)]
pub struct GroupData {
    pub val: Box<Value>,
    pub next: Box<Group>
}

impl fmt::Display for GroupData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[derive(Debug)]
pub enum Group {
    Data(GroupData),
    None
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Group::Data(data) => write!(f, "[{}]", data),
            Group::None => write!(f, "[]")
        }
    }
}

#[derive(Debug)]
pub enum Value {
   Frac(Frac),
   String(String),
   Group(Group),
   Func(Func)
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Frac(fr) => write!(f, "{}", fr),
            Value::String(s) => write!(f, "{}", s),
            Value::Group(g) => write!(f, "{}", g),
            Value::Func(c) => write!(f, "{}", c)
        }
    }
}

pub fn eval(code: String) -> Value {
    Value::String(code.to_string())
}

pub fn run(code: String) {
    println!("Running:\n{}", code);
}

fn main() {
    println!("You are a plumber, now.\nDo some plumbing.\nAnd by that I mean code.\n\n");
    loop {
        print!("\n> \x1b[1m\x1b[38;02;255;255;100m");
        std::io::stdout().flush().unwrap();
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).expect("A command to run");
        print!("\x1b[0m");
        std::io::stdout().flush().unwrap();
        s = s.trim().to_string();
        if s == "".to_string() {
            print!("\x1b[2J\x1b[1;1H");
            std::io::stdout().flush().unwrap();
        } else if s == "quit".to_string() || s == "exit".to_string() {
            break;
        } else {
            let v: Value = eval(s);
            println!("Result: \x1b[38;02;100;255;100m{}\x1b[0m", v);
        }
    }
}
