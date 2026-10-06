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
    fs::read_to_string(p).unwrap_or_default()
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
fn str2(x: impl std::fmt::Display) -> String { x.to_string() }

// ---- Phase 3 runtime: time, stdin, screen ----
fn clock() -> String {
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs()).unwrap_or(0);
    let day = secs / 86400;
    let rem = secs % 86400;
    let (h, m) = (rem / 3600, (rem % 3600) / 60);
    let z = day as i64 + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe/1460 + doe/36524 - doe/146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365*yoe + yoe/4 - yoe/100);
    let mp = (5*doy + 2)/153;
    let d = doy - (153*mp+2)/5 + 1;
    let mth = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if mth <= 2 { y + 1 } else { y };
    format!("{y:04}-{mth:02}-{d:02} {h:02}:{m:02}")
}
fn today() -> String {
    let c = clock();
    c.split(' ').next().unwrap_or("").to_string()
}
fn sleep(ms: i64) {
    std::thread::sleep(std::time::Duration::from_millis(ms as u64));
}
fn ask(prompt: impl AsRef<str>) -> String {
    use std::io::BufRead;
    print!("{}", prompt.as_ref());
    use std::io::Write;
    let _ = std::io::stdout().flush();
    let mut line = String::new();
    let _ = std::io::stdin().lock().read_line(&mut line);
    line.trim().to_string()
}
fn cls() {
    print!("\u{1b}[2J\u{1b}[H");
    let _ = std::io::stdout().flush();
}
fn quit(c: i64) -> ! { process::exit(c as i32) }

// ---- Phase 2 runtime: lists & strings ----
fn ls(items: Vec<String>) -> Vec<String> { items }
fn li(items: Vec<i64>) -> Vec<i64> { items }
fn push(v: &mut Vec<String>, s: impl AsRef<str>) { v.push(s.as_ref().to_string()) }
fn pushi(v: &mut Vec<i64>, n: i64) { v.push(n) }
fn pop(v: &mut Vec<String>) -> Option<String> { v.pop() }
fn popi(v: &mut Vec<i64>) -> Option<i64> { v.pop() }
fn count(v: &Vec<String>) -> i64 { v.len() as i64 }
fn counti(v: &Vec<i64>) -> i64 { v.len() as i64 }
fn get(v: &Vec<String>, i: i64) -> String { v.get(i as usize).cloned().unwrap_or_default() }
fn geti(v: &Vec<i64>, i: i64) -> i64 { v.get(i as usize).copied().unwrap_or(0) }
fn seti(v: &mut Vec<String>, i: i64, s: impl Into<String>) {
    let k = i as usize;
    while v.len() <= k { v.push(String::new()); }
    v[k] = s.into();
}
fn setii(v: &mut Vec<i64>, i: i64, n: i64) {
    let k = i as usize;
    while v.len() <= k { v.push(0); }
    v[k] = n;
}
fn join(v: &Vec<String>, sep: impl AsRef<str>) -> String { v.join(sep.as_ref()) }
fn split(s: impl AsRef<str>, sep: impl AsRef<str>) -> Vec<String> {
    s.as_ref().split(sep.as_ref()).map(|x| x.to_string()).collect()
}
fn lines_of(s: impl AsRef<str>) -> Vec<String> {
    s.as_ref().lines().map(|x| x.to_string()).collect()
}
fn trim(s: &impl AsRef<str>) -> String { s.as_ref().trim().to_string() }
fn upper(s: &impl AsRef<str>) -> String { s.as_ref().to_uppercase() }
fn lower(s: &impl AsRef<str>) -> String { s.as_ref().to_lowercase() }
fn rep(s: &impl AsRef<str>, n: i64) -> String { s.as_ref().repeat(n as usize) }
fn has(s: &impl AsRef<str>, sub: impl AsRef<str>) -> bool { s.as_ref().contains(sub.as_ref()) }
fn starts(s: &impl AsRef<str>, sub: impl AsRef<str>) -> bool { s.as_ref().starts_with(sub.as_ref()) }
fn ends(s: &impl AsRef<str>, sub: impl AsRef<str>) -> bool { s.as_ref().ends_with(sub.as_ref()) }
fn idx(s: &impl AsRef<str>, sub: impl AsRef<str>) -> i64 {
    match s.as_ref().find(sub.as_ref()) { Some(k) => k as i64, None => -1 }
}
fn cut(s: &impl AsRef<str>, a: i64, b: i64) -> String { pm_slice(s.as_ref(), a, b) }
fn rev(s: &impl AsRef<str>) -> String { s.as_ref().chars().rev().collect() }
fn sort(v: &mut Vec<String>) { v.sort(); }
fn sorti(v: &mut Vec<i64>) { v.sort(); }
fn revv(v: &mut Vec<String>) { v.reverse(); }
fn sum(v: &Vec<i64>) -> i64 { v.iter().sum() }
"#;

// (primed name, rust prefix, borrow_first_arg)
const BUILTINS: &[(&str, &str, u8)] = &[
    ("len", "pm_len", 1),
    ("read", "read", 1),
    ("write", "write", 1),
    ("append", "append", 1),
    ("home", "home", 0),
    ("arg", "arg", 0),
    ("argn", "argn", 0),
    ("now", "now", 0),
    ("int", "int", 1),
    ("strt", "strt", 0),
    ("quit", "quit", 0),
    ("char", "pm_index", 1),
    ("push", "push", 2),
    ("pushi", "pushi", 2),
    ("pop", "pop", 2),
    ("popi", "popi", 2),
    ("count", "count", 1),
    ("get", "get", 1),
    ("geti", "geti", 1),
    ("join", "join", 1),
    ("split", "split", 1),
    ("lines", "lines_of", 1),
    ("trim", "trim", 1),
    ("upper", "upper", 1),
    ("lower", "lower", 1),
    ("rep", "rep", 1),
    ("has", "has", 1),
    ("starts", "starts", 1),
    ("ends", "ends", 1),
    ("idx", "idx", 1),
    ("cut", "cut", 1),
    ("rev", "rev", 1),
    ("sort", "sort", 2),
    ("sorti", "sorti", 2),
    ("seti", "seti", 2),
    ("setii", "setii", 2),
    ("sum", "sum", 1),
    ("clock", "clock", 0),
    ("today", "today", 0),
    ("sleep", "sleep", 0),
    ("ask", "ask", 0),
    ("cls", "cls", 0),
];

pub fn camel(name: &str) -> String {
    let mut out = String::new();
    for (k, part) in name.split('_').enumerate() {
        if k > 0 {
            out.push('_');
        }
        let mut c = part.chars();
        if let Some(f) = c.next() {
            out.extend(f.to_uppercase());
            out.push_str(c.as_str());
        }
    }
    out
}

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
                    match brw {
                        1 => out.push('&'),
                        2 => out.push_str("&mut "),
                        _ => {}
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
        Node::Struct { name, fields } => {
            let rn = camel(name);
            out.push_str(&format!(
                "{pad}#[derive(Debug, Clone)]\nstruct {rn} {{\n"
            ));
            for (f, t) in fields {
                out.push_str(&format!("{pad}  {f}: {t},\n"));
            }
            out.push_str(&format!("{pad}}}\n"));
        }
        Node::Each { var, over, body } => {
            let o = map_builtins(over);
            out.push_str(&format!("{pad}for {var} in {o}.iter() {{\n"));
            for b in body {
                gen_node(b, ind + 1, out);
            }
            out.push_str(&format!("{pad}}}\n"));
        }
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
