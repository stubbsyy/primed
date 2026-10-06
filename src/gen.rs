use crate::parser::{parse, Node};

const PRELUDE: &str = r#"#![allow(dead_code, unused_parens)]
use std::fs;
use std::io::Write as _;
use std::io::Read as _;
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
fn pm_home() -> String { std::env::var("HOME").unwrap_or_else(|_| ".".into()) }
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

fn scan_json_str(raw: &str, key: &str) -> String {
    let pat = format!("\"{key}\"");
    let start = match raw.find(&pat) { Some(k) => k + pat.len(), None => return String::new() };
    let colon = match raw[start..].find(':') { Some(k) => start + k + 1, None => return String::new() };
    let rest = raw[colon..].trim_start();
    if !rest.starts_with('"') { return String::new(); }
    let mut out = String::new();
    let mut chars = rest[1..].chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => break,
            '\\' => {
                if let Some(e) = chars.next() {
                    out.push(match e { 'n' => '\n', 't' => '\t', other => other });
                }
            }
            other => out.push(other),
        }
    }
    out
}

// ---- Phase 5 runtime: HTTP server + JSON ----
struct Request {
    method: String,
    path: String,
    body: String,
}

struct Response {
    status: u16,
    headers: Vec<(String, String)>,
    body: String,
}

impl Response {
    fn new() -> Response {
        Response { status: 200, headers: vec![], body: String::new() }
    }
    fn code(mut self, c: u16) -> Response { self.status = c; self }
    fn text(mut self, body: impl Into<String>) -> Response {
        self.headers.push(("Content-Type".into(), "text/plain; charset=utf-8".into()));
        self.body = body.into();
        self
    }
    fn json(mut self, body: impl Into<String>) -> Response {
        self.headers.push(("Content-Type".into(), "application/json".into()));
        self.body = body.into();
        self
    }
}

static mut HANDLERS: Option<Vec<(&'static str, fn(String) -> String)>> = None;

fn leak(s: &str) -> &'static str {
    let b: &'static mut String = Box::leak(Box::new(s.to_string()));
    b.as_str()
}

fn route(path: impl AsRef<str>, handler: fn(String) -> String) {
    let h: &'static mut Option<Vec<(&'static str, fn(String) -> String)>> =
        unsafe { &mut *std::ptr::addr_of_mut!(HANDLERS) };
    let p: &'static str = leak(path.as_ref());
    if h.is_none() { *h = Some(vec![]); }
    h.as_mut().unwrap().push((p, handler));
}

fn srv(port: i64) -> ! {
    let addr = format!("0.0.0.0:{port}");
    let listener = std::net::TcpListener::bind(&addr)
        .unwrap_or_else(|e| { eprintln!("bind {addr}: {e}"); process::exit(1) });
    eprintln!("primed: serving on http://{addr}");
    for stream in listener.incoming() {
        let mut stream = match stream { Ok(s) => s, Err(_) => continue };
        let mut buf = [0u8; 8192];
        let n = match stream.read(&mut buf) { Ok(n) => n, Err(_) => continue };
        let raw = String::from_utf8_lossy(&buf[..n]).to_string();
        let req_line = raw.lines().next().unwrap_or("").to_string();
        let mut parts = req_line.split_whitespace();
        let method = parts.next().unwrap_or("GET").to_string();
        let path_full = parts.next().unwrap_or("/").to_string();
        let path = path_full.split('?').next().unwrap_or("/").to_string();
        let body = raw.split("\r\n\r\n").nth(1).unwrap_or("").to_string();
        let req_json = jobj(&[
            ("method", &format!("\"{method}\"")),
            ("path", &format!("\"{path}\"")),
            ("body", &format!("\"{body}\"")),
        ]);
        let handlers = unsafe { &*std::ptr::addr_of!(HANDLERS) };
        let mut body_out = format!("not found: {path}");
        let mut code = 404u16;
        if let Some(hs) = handlers {
            for (hp, h) in hs.iter() {
                if *hp == path { body_out = h(req_json.clone()); code = 200; break; }
            }
        }
        let mut resp = Response::new().code(code).text(body_out);
        let status_text = match resp.status {
            200 => "OK", 201 => "Created", 400 => "Bad Request",
            404 => "Not Found", 500 => "Internal Server Error", _ => "OK",
        };
        let mut out = format!("HTTP/1.1 {} {}\r\n", resp.status, status_text);
        for (k, v) in &resp.headers { out.push_str(&format!("{k}: {v}\r\n")); }
        out.push_str(&format!("Content-Length: {}\r\n\r\n", resp.body.len()));
        out.push_str(&resp.body);
        let _ = stream.write_all(out.as_bytes());
    }
    unreachable!()
}

fn json_skip_ws(b: &[u8], i: &mut usize) {
    while *i < b.len() && (b[*i] == b' ' || b[*i] == b'\n' || b[*i] == b'\t' || b[*i] == b'\r') { *i += 1; }
}

fn json_find(b: &[u8], i: &mut usize, key: &str) -> Option<String> {
    json_skip_ws(b, i);
    if *i >= b.len() || b[*i] != b'{' { return None; }
    *i += 1;
    loop {
        json_skip_ws(b, i);
        if *i >= b.len() || b[*i] == b'}' { return None; }
        if b[*i] != b'"' { return None; }
        *i += 1;
        let ks = *i;
        while *i < b.len() && b[*i] != b'"' { *i += 1; }
        let k = String::from_utf8_lossy(&b[ks..*i]).to_string();
        *i += 1;
        json_skip_ws(b, i);
        if *i < b.len() && b[*i] == b':' { *i += 1; }
        json_skip_ws(b, i);
        let vs = *i;
        if *i < b.len() && b[*i] == b'"' {
            *i += 1;
            while *i < b.len() && b[*i] != b'"' { if b[*i] == b'\\' { *i += 1; } *i += 1; }
            if *i < b.len() { *i += 1; }
        } else if *i < b.len() && (b[*i] == b'{' || b[*i] == b'[') {
            let open = b[*i];
            let close = if open == b'{' { b'}' } else { b']' };
            let mut depth = 0;
            while *i < b.len() {
                if b[*i] == open { depth += 1; }
                if b[*i] == close { depth -= 1; if depth == 0 { *i += 1; break; } }
                *i += 1;
            }
        } else {
            while *i < b.len() && b[*i] != b',' && b[*i] != b'}' { *i += 1; }
        }
        let val = String::from_utf8_lossy(&b[vs..(*i).min(b.len())]).trim().to_string();
        if k == key { return Some(val); }
        json_skip_ws(b, i);
        if *i < b.len() && b[*i] == b',' { *i += 1; }
    }
}

fn jget(json: impl AsRef<str>, key: impl AsRef<str>) -> String {
    let (j, k) = (json.as_ref(), key.as_ref());
    let b = j.as_bytes();
    let mut i = 0;
    json_find(b, &mut i, k).unwrap_or_default()
}
fn jstr(raw: impl AsRef<str>) -> String {
    let t = raw.as_ref().trim();
    if t.starts_with('"') && t.len() >= 2 {
        t[1..t.len()-1].replace("\\n", "\n").replace("\\t", "\t").replace("\\\"", "\"")
    } else { String::new() }
}
fn jnum(raw: impl AsRef<str>) -> i64 { raw.as_ref().trim().parse().unwrap_or(0) }
fn jbool(raw: impl AsRef<str>) -> bool { raw.as_ref().trim() == "true" }
fn jq(k: impl AsRef<str>, v: impl AsRef<str>) -> String {
    let (k, v) = (k.as_ref(), v.as_ref());
    let vs = if v.starts_with('{') || v.starts_with('[') || v.starts_with('"')
        || v == "true" || v == "false" || v.parse::<i64>().is_ok() || v.parse::<f64>().is_ok()
    { v.to_string() } else { format!("\"{v}\"") };
    format!("\"{k}\":{vs}")
}
fn jo(k: impl AsRef<str>, v: impl AsRef<str>) -> String { format!("{{{}}}", jq(k, v)) }
fn jobj(pairs: &[(&str, &str)]) -> String {
    let inner: Vec<String> = pairs.iter().map(|(k, v)| jq(k, v)).collect();
    format!("{{{}}}", inner.join(","))
}


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
    ("home", "pm_home", 0),
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
    ("jget", "jget", 1),
    ("jstr", "jstr", 1),
    ("jnum", "jnum", 1),
    ("jbool", "jbool", 1),
    ("jobj", "jobj", 0),
    ("jq", "jq", 1),
    ("jo", "jo", 1),
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
        Node::ToolDef { sig, body, .. } => {
            let ret = match sig.ret {
                Some('i') => " -> i64",
                Some('f') => " -> f64",
                Some('s') => " -> String",
                Some('b') => " -> bool",
                _ => "",
            };
            out.push_str(&format!(
                "{pad}fn {}({}){} {{\n",
                sig.name,
                sig.params.join(", "),
                ret
            ));
            for b in body {
                gen_node(b, ind + 1, out);
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


pub const MCP_MAIN: &str = r#"static mut TOOLS: Option<Vec<(&'static str, &'static str, fn(String) -> String)>> = None;
fn main() {
    let mut tools: Vec<(&'static str, &'static str, fn(String) -> String)> = vec![];
__TOOL_REG__
    unsafe { *std::ptr::addr_of_mut!(TOOLS) = Some(tools); }
    use std::io::BufRead;
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = match line { Ok(l) => l, Err(_) => break };
        if line.trim().is_empty() { continue; }
        let m = jstr(jget(&line, "method"));
        let id = jget(&line, "id");
        if m == "initialize" {
            println!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"result\":{{\"protocolVersion\":\"2024-11-05\",\"capabilities\":{{\"tools\":{{}}}},\"serverInfo\":{{\"name\":\"primed\",\"version\":\"0.1\"}}}}}}", id);
        } else if m == "tools/list" {
            let tl = unsafe { (&*std::ptr::addr_of!(TOOLS)).as_ref() };
            let mut arr = String::new();
            if let Some(tl) = tl {
                let items: Vec<String> = tl.iter().map(|(n, d, _)| format!("{{\"name\":\"{}\",\"description\":\"{}\"}}", n, d)).collect();
                arr = items.join(",");
            }
            println!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"result\":{{\"tools\":[{}]}}}}", id, arr);
        } else if m == "tools/call" {
            // request shape: {"method":"tools/call","params":{"name":"x","arguments":{...}}}
            // jget only handles flat objects; extract name/arguments from the raw line
            let tname = scan_json_str(&line, "name");
            let arg = scan_json_str(&line, "text");
            let tl = unsafe { (&*std::ptr::addr_of!(TOOLS)).as_ref() };
            let mut result = format!("unknown tool: {}", tname);
            if let Some(tl) = tl {
                for (n, _, h) in tl.iter() {
                    if *n == tname { result = h(arg); break; }
                }
            }
            let esc = result.replace('\\', "\\\\").replace('"', "\\\"");
            println!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"result\":{{\"content\":[{{\"type\":\"text\",\"text\":\"{}\"}}]}}}}", id, esc);
        }
    }
}
"#;

pub fn gen(src: &str, path: &str) -> Result<String, String> {
    gen_mode(src, path, false)
}

pub fn gen_mode(src: &str, path: &str, mcp: bool) -> Result<String, String> {
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
    if !mcp {
        return Ok(format!("{PRELUDE}\n{fns}\nfn main() {{\n{main_body}}}\n"));
    }
    let mut tool_reg = String::new();
    for (name, desc) in &prog.tools {
        tool_reg.push_str(&format!(
            "    tools.push((\"{name}\", \"{desc}\", {name} as fn(String) -> String));\n"
        ));
    }
    let mcp_main = MCP_MAIN.replace("__TOOL_REG__", &tool_reg);
    Ok(format!("{PRELUDE}\n{fns}\n{mcp_main}"))
}

pub const CHEATSHEET: &str = r#"# primed language (.pm) — LLM cheat sheet

Compiles to native Rust. Write minimal, dense code. 2-space indent blocks, no semicolons, # comments.

## types
i=i64 f=f64 s=String b=bool ls=Vec<String> li=Vec<i64>

## decls
f name(a i, b s) s    # fn, typed params, optional return type
v x = expr            # immutable let
m x = expr            # mutable let
x = expr              # assignment (m-declared only)

## control
if cond / elif cond / e        # else
w cond                          # while
each x list                     # for-in
ret expr                        # return

## exprs
"a {x} b"              # interpolation, any expr in {}
+ - * / %              # + also concatenates strings
== != < <= > >= and or not
x[i] x[a:b] x[a:]      # char / slice
arg(1) ? "default"     # Option default

## structs
t name { id i, text s }
{note id: 1, text: "x"}    # literal (camel-cased internally)
n.id                       # field

## lists
[1, 2] ["a", "b"]
push(l, x) pushi count(l) get(l, i) geti seti(l, i, x)
pop(l) popi sort(l) sorti sum(l) join(l, ",") split(s, ",") lines(s)

## strings
trim upper lower rep(s, n) rev has(s, sub) starts ends idx cut(s, a, b)
len(s) int(s) str(x)

## io/system
p x                    # println
read(path) write(path, s) append(path, s)   # read of missing file = ""
ask("prompt")          # stdin line
arg(i) argn() home() now() clock() today() sleep(ms) cls() quit(code)

## web (phase 5)
f h(req s) s ...        # handler: req is JSON string, ret body
route("/path", h)       # register
srv(8080)               # serve http
jget(req, "body") jstr jnum jbool   # parse request json
jq(k, v) jo(k, v)       # build json

## mcp (phase 6)
tool name(text s) s "desc"   # MCP tool; body like f; ret = result
primed mcp file.pm           # serve MCP over stdio

## cli
primed run file.pm [args]    # transpile+compile+run
primed build file.pm -o out  # native binary
primed install file.pm       # -> ~/.local/bin/<name>
primed transpile file.pm     # print generated rust
primed tokens file.pm        # token count report
primed doc file.pm [--mcp]   # JSON metadata + transpiled rust for agents
primed cheat                 # this sheet

## example
f main()
  v cmd = arg(1) ? "list"
  if cmd == "add"
    append(home() + "/notes.txt", arg(2) + "\n")
"#;

pub fn list_fns(src: &str, path: &str) -> Result<Vec<String>, String> {
    let prog = parse(src, path)?;
    Ok(prog
        .nodes
        .iter()
        .filter_map(|n| match n {
            Node::FnDef { sig, .. } => Some(format!(
                "{}({}){}",
                sig.name,
                sig.params.join(","),
                sig.ret.map(|r| format!(" {r}")).unwrap_or_default()
            )),
            _ => None,
        })
        .collect())
}

pub fn list_tools(src: &str, path: &str) -> Result<Vec<(String, String)>, String> {
    let prog = parse(src, path)?;
    Ok(prog.tools.clone())
}

pub fn list_structs(src: &str, path: &str) -> Result<Vec<String>, String> {
    let prog = parse(src, path)?;
    Ok(prog
        .nodes
        .iter()
        .filter_map(|n| match n {
            Node::Struct { name, .. } => Some(name.clone()),
            _ => None,
        })
        .collect())
}
