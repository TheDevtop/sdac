use serde_derive::{Deserialize, Serialize};
use serde_lexpr;
use std::{cmp, env, fs};

const ITRUE: isize = 1;
const IFALSE: isize = 0;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Val {
    Val(isize),
    Exp(Box<Exp>),
}

type Program = Val;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Exp {
    Add(Val, Val), // Addition
    Mul(Val, Val), // Multiplication
    Sub(Val, Val), // Subtraction
    Div(Val, Val), // Division
    Mod(Val, Val), // Modulus
    And(Val, Val), // And
    Ior(Val, Val), // Inclusive Or
    Xor(Val, Val), // Exclusive Or
    Eqa(Val, Val), // Equality
    Neq(Val, Val), // Non Equality
    Min(Val, Val), // Minimum
    Max(Val, Val), // Maximum
}

// Expression evaluation
fn eval(e: Exp) -> isize {
    match e {
        Exp::Add(x, y) => {
            let v = (detect(x), detect(y));
            println!("+ {} {}", v.0, v.1);
            return v.0 + v.1;
        }
        Exp::Mul(x, y) => {
            let v = (detect(x), detect(y));
            println!("* {} {}", v.0, v.1);
            return v.0 * v.1;
        }
        Exp::Sub(x, y) => {
            let v = (detect(x), detect(y));
            println!("- {} {}", v.0, v.1);
            return v.0 - v.1;
        }
        Exp::Div(x, y) => {
            let v = (detect(x), detect(y));
            println!("/ {} {}", v.0, v.1);
            return v.0 / v.1;
        }
        Exp::Mod(x, y) => {
            let v = (detect(x), detect(y));
            println!("% {} {}", v.0, v.1);
            return v.0 % v.1;
        }
        Exp::And(x, y) => {
            let v = (detect(x), detect(y));
            println!("& {} {}", v.0, v.1);
            return v.0 & v.1;
        }
        Exp::Ior(x, y) => {
            let v = (detect(x), detect(y));
            println!("| {} {}", v.0, v.1);
            return v.0 | v.1;
        }
        Exp::Xor(x, y) => {
            let v = (detect(x), detect(y));
            println!("^ {} {}", v.0, v.1);
            return v.0 ^ v.1;
        }
        Exp::Eqa(x, y) => {
            let v = (detect(x), detect(y));
            println!("= {} {}", v.0, v.1);
            if v.0 == v.1 {
                return ITRUE;
            } else {
                return IFALSE;
            }
        }
        Exp::Neq(x, y) => {
            let v = (detect(x), detect(y));
            println!("! {} {}", v.0, v.1);
            if v.0 != v.1 {
                return ITRUE;
            } else {
                return IFALSE;
            }
        }
        Exp::Min(x, y) => {
            let v = (detect(x), detect(y));
            println!("< {} {}", v.0, v.1);
            return cmp::min(v.0, v.1);
        }
        Exp::Max(x, y) => {
            let v = (detect(x), detect(y));
            println!("> {} {}", v.0, v.1);
            return cmp::max(v.0, v.1);
        }
    }
}

// Type detection
fn detect(p: Program) -> isize {
    match p {
        Val::Val(v) => v,
        Val::Exp(e) => eval(*e),
    }
}

// Program loader
fn load(path: &String) -> Program {
    let buf: String = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => panic!("Could not open file"),
    };
    match serde_lexpr::from_str::<Program>(&buf) {
        Ok(p) => return p,
        Err(_) => panic!("Could not parse file to valid program"),
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("usage: sdac [FILE]");

    println!(": {}", detect(load(path)));
}
