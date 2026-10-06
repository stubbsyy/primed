use crate::parser::{parse, Node};

const PRELUDE: &str = r#"#![allow(dead_code, unused_parens)]
use std::fs;
use std::io::Write as _;
use std::process;

trait PmAdd<R> { type Out; fn add(self, r: R) -> Self::Out; }
impl PmAdd<&i64> for &i64 { type Out = i64; fn add(self, r: &i64) -> i64 { *self + *r } }
impl PmAdd<&f64> for &i64 { type Out = f64; fn add(self, r: &f64) -> f64 { *self as f64 + *r } }
impl PmAdd<&i64> for &f64 { type Out = f64; fn add(self, r: &i64) -> f64 { *self + *r as f64 } }
impl PmAdd<&f64> for &f64 { type Out = f64; fn add(self, r: &f64) -> f64 { *self + *r } }
impl PmAdd<&str> for &String { type Out = String; fn add(self, r: &str) -> String { format!("{self}{r}") } }
impl PmAdd<&String> for &String { type Out = String; fn add(self, r: &String) -> String { format!("{self}{r}") } }
impl PmAdd<&String> for &str { type Out = String; fn add(self, r: &String) -> String { format!("{self}{r}") } }
impl PmAdd<&str> for &str { type Out = String; fn add(self, r: &str) -> String { format!("{self}{r}") } }
impl PmAdd<&i64> for &String { type Out = String; fn add(self, r: &i64) -> String { format!("{self}{r}") } }
impl PmAdd<&String> for &i64 { type Out = String; fn add(self, r: &String) -> String { format!("{self}{r}") } }
impl PmAdd<&i64> for &str { type Out = String; fn add(self, r: &i64) -> String { format!("{self}{r}") } }
impl PmAdd<&str> for &i64 { type Out = String; fn add(self, r: &str) -> String { format!("{self}{r}") } }
fn pm_add<T: PmAdd<R>, R>(a: T, b: R) -> <T as PmAdd<R>>::Out { a.add(b) }
fn pm_sub(a: &i64, b: &i64) -> i64 { *a - *b }
fn pm_mul(a: &i64, b: &i64) -> i64 { *a * *b }
fn pm_div(a: &i64, b: &i64) -> i64 { *a / *b }
fn pm_rem(a: &i64, b: &i64) -> i64 { *a % *b }
fn pm_neg(a: i64) -> i64 { -a }
fn pm_eq<A: PartialEq<B> + ?Sized, B: ?Sized>(a: &A, b: &B) -> bool { a == b }
fn pm_index(s: &impl AsRef<str>, i: i64) -> String {
    s.as_ref().chars().nth(i as usize).map(|c| c.to_string()).unwrap_or_default()
}
fn pm_slice(s: &str, a: i64, b: i64) -> String {
    s.chars().skip(a as usize).take((b - a) as usize).collect()
}
fn pm_slice_from(s: &str, a: i64) -> String { s.chars().skip(a as usize).collect() }
fn pm_dflt(o: Option<String>, d: impl Into<String>) -> String { o.unwrap_or_else(|| d.into()) }
fn pm_len(s: &impl AsRef<str>) -> i64 { s.as_ref().chars().count() as i64 }
fn read(p: &impl AsRef<str>) -> String {
    let p = p.as_ref();
    fs::read_to_string(p).unwrap_or_else(|e| { eprintln!("read {p}: {e}"); process::exit(1) })
}
fn write(p: &impl AsRef<str>, s: impl AsRef<str>) {
    let p = p.as_ref();
    fs::write(p, s.as_ref()).unwrap_or_else(|e| { eprintln!("write {p}: {e}"); process::exit(1) });
}
fn append(p: &impl AsRef<str>, s: impl AsRef<str>) {
    let p = p.as_ref();
    let mut f = fs::OpenOptions::new().create(true).append(true).open(p)
        .unwrap_or_else(|e| { eprintln!("append {p}: {e}"); process::exit(1) });
    f.write_all(s.as_ref().as_bytes()).unwrap_or_else(|e| { eprintln!("append {p}: {e}"); process::exit(1) });
}
fn home() -> String { std::env::var("HOME").unwrap_or_else(|_| ".".into()) }
fn arg(i: i64) -> Option<String> { std::env::args().nth(i as usize) }
fn argn() -> i64 { std::env::args().count() as i64 }
fn now() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64).unwrap_or(0)
}
fn int(s: &impl AsRef<str>) -> i64 { s.as_ref().trim().parse().unwrap_or(0) }
fn strt(x: impl std::fmt::Display) -> String { x.to_string() }
fn quit(c: i64) -> ! { process::exit(c as i32) }
"#;

// (primed name, rust prefix, borrow_first_arg)
const BUILTINS: &[(&str, &str, bool)] = &[
    ("len", "pm_len", true),
    ("read", "read", true),
    ("write", "write", true),
    ("append", "append", true),
    ("home", "home", false),
    ("arg", "arg", false),
    ("argn", "argn", false),
    ("now", "now", false),
    ("int", "int", true),
    ("strt", "strt", false),
    ("quit", "quit", false),
    ("char", "pm_index", true),
];

fn map_builtins(e: &str) -> String {
    let b: Vec<char> = e.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < b.len() {
        if b[i].is_alphabetic() || b[i] == '_' {
            let start = i;
            while i < b.len() && (b[i].is_alphanumeric() || b[i] == '_') {
                i += 1;
            }
            let w: String = b[start..i].iter().collect();
            let is_call = i < b.len() && b[i] == '(';
            if is_call {
                if let Some((_, rust, brw)) =
                    BUILTINS.iter().find(|(p, _, _)| *p == w)
                {
                    out.push_str(rust);
                    out.push('(');
                    if *brw {
                        out.push('&');
                    }
                    i += 1;
                    continue;
                }
            }
            out.push_str(&w);
            continue;
        }
        out.push(b[i]);
        i += 1;
    }
    out
}

fn gen_node(n: &Node, ind: usize, out: &mut String) {
    let pad = "  ".repeat(ind);
    match n {
        Node::FnDef { sig, body } => {
            if sig.name == "main" {
                for b in body {
                    gen_node(b, 1, out);
                }
                return;
            }
            let ret = match sig.ret {
                Some('i') => " -> i64",
                Some('f') => " -> f64",
                Some('s') => " -> String",
                Some('b') => " -> bool",
                _ => "",
            };
            let params = sig
                .params
                .iter()
                .map(|p| {
                    let mut it = p.splitn(2, ": ");
                    let n = it.next().unwrap_or("");
                    let t = match it.next().unwrap_or("").trim() {
                        "i64" => "i64",
                        "f64" => "f64",
                        "String" => "String",
                        "bool" => "bool",
                        o => o,
                    };
                    format!("{n}: {t}")
                })
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!("{pad}fn {}({}){} {{\n", sig.name, params, ret));
            for b in body {
                gen_node(b, ind + 1, out);
            }
            out.push_str(&format!("{pad}}}\n"));
        }
        Node::Var { name, mutable, expr } => {
            let kw = if *mutable { "let mut " } else { "let " };
            out.push_str(&format!("{pad}{kw}{name} = {};\n", map_builtins(expr)));
        }
        Node::Assign { name, expr } => {
            out.push_str(&format!("{pad}{name} = {};\n", map_builtins(expr)));
        }
        Node::If { arms } => {
            for (k, (cond, body)) in arms.iter().enumerate() {
                match (k, cond) {
                    (0, Some(c)) => {
                        out.push_str(&format!("{pad}if {} {{\n", map_builtins(c)))
                    }
                    (_, Some(c)) => {
                        out.push_str(&format!("{pad}}} else if {} {{\n", map_builtins(c)))
                    }
                    (_, None) => out.push_str(&format!("{pad}}} else {{\n")),
                }
                for b in body {
                    gen_node(b, ind + 1, out);
                }
            }
            out.push_str(&format!("{pad}}}\n"));
        }
        Node::While { cond, body } => {
            out.push_str(&format!("{pad}while {} {{\n", map_builtins(cond)));
            for b in body {
                gen_node(b, ind + 1, out);
            }
            out.push_str(&format!("{pad}}}\n"));
        }
        Node::Ret(e) => match e {
            Some(e) => out.push_str(&format!("{pad}return {};\n", map_builtins(e))),
            None => out.push_str(&format!("{pad}return;\n")),
        },
        Node::Expr(e) => out.push_str(&format!("{pad}{};\n", map_builtins(e))),
        Node::Print(e) => {
            out.push_str(&format!("{pad}println!(\"{{}}\", &{});\n", map_builtins(e)))
        }
    }
}

pub fn gen(src: &str, path: &str) -> Result<String, String> {
    let prog = parse(src, path)?;
    let mut main_body = String::new();
    let mut fns = String::new();
    for n in &prog.nodes {
        match n {
            Node::FnDef { sig, .. } if sig.name == "main" => {
                gen_node(n, 0, &mut main_body);
            }
            _ => {
                gen_node(n, 0, &mut fns);
            }
        }
    }
    Ok(format!("{PRELUDE}\n{fns}\nfn main() {{\n{main_body}}}\n"))
}
