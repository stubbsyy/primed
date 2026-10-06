use crate::lexer::{lex_line, StrPart, Tok};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct FnSig {
    pub name: String,
    pub params: Vec<String>,
    pub ret: Option<char>,
}

#[derive(Debug)]
pub enum Node {
    FnDef { sig: FnSig, body: Vec<Node> },
    Var { name: String, mutable: bool, expr: String },
    Assign { name: String, expr: String },
    If { arms: Vec<(Option<String>, Vec<Node>)> },
    While { cond: String, body: Vec<Node> },
    Ret(Option<String>),
    Expr(String),
    Print(String),
}

pub struct Prog {
    pub nodes: Vec<Node>,
}

fn ty(t: &str) -> Result<&'static str, String> {
    match t {
        "i" => Ok("i64"),
        "f" => Ok("f64"),
        "s" => Ok("String"),
        "b" => Ok("bool"),
        other => Err(format!("unknown type {other}")),
    }
}

fn parse_expr(toks: &[Tok], i: &mut usize) -> Result<String, String> {
    parse_or(toks, i)
}

fn parse_or(t: &[Tok], i: &mut usize) -> Result<String, String> {
    let mut l = parse_and(t, i)?;
    loop {
        let hit = matches!(t.get(*i), Some(Tok::Kw("or")) | Some(Tok::Sym("||")));
        if !hit {
            break;
        }
        *i += 1;
        let r = parse_and(t, i)?;
        l = format!("({l} || {r})");
    }
    Ok(l)
}

fn parse_and(t: &[Tok], i: &mut usize) -> Result<String, String> {
    let mut l = parse_cmp(t, i)?;
    loop {
        let hit = matches!(t.get(*i), Some(Tok::Kw("and")) | Some(Tok::Sym("&&")));
        if !hit {
            break;
        }
        *i += 1;
        let r = parse_cmp(t, i)?;
        l = format!("({l} && {r})");
    }
    Ok(l)
}

fn parse_cmp(t: &[Tok], i: &mut usize) -> Result<String, String> {
    let mut l = parse_add(t, i)?;
    loop {
        let s = match t.get(*i) {
            Some(Tok::Sym(s)) if matches!(*s, "==" | "!=" | "<" | ">" | "<=" | ">=") => *s,
            _ => break,
        };
        *i += 1;
        let r = parse_add(t, i)?;
        l = match s {
            "==" => format!("pm_eq(&{l}, &{r})"),
            "!=" => format!("!pm_eq(&{l}, &{r})"),
            "<" => format!("({l} < {r})"),
            ">" => format!("({l} > {r})"),
            "<=" => format!("({l} <= {r})"),
            _ => format!("({l} >= {r})"),
        };
    }
    Ok(l)
}

fn parse_add(t: &[Tok], i: &mut usize) -> Result<String, String> {
    let mut l = parse_mul(t, i)?;
    loop {
        let s = match t.get(*i) {
            Some(Tok::Sym(s)) if *s == "+" || *s == "-" => *s,
            _ => break,
        };
        *i += 1;
        let r = parse_mul(t, i)?;
        l = if s == "+" {
            format!("pm_add(&{l}, &{r})")
        } else {
            format!("pm_sub(&{l}, &{r})")
        };
    }
    Ok(l)
}

fn parse_mul(t: &[Tok], i: &mut usize) -> Result<String, String> {
    let mut l = parse_unary(t, i)?;
    loop {
        let s = match t.get(*i) {
            Some(Tok::Sym(s)) if matches!(*s, "*" | "/" | "%") => *s,
            _ => break,
        };
        *i += 1;
        let r = parse_unary(t, i)?;
        l = match s {
            "*" => format!("pm_mul(&{l}, &{r})"),
            "/" => format!("pm_div(&{l}, &{r})"),
            _ => format!("pm_rem(&{l}, &{r})"),
        };
    }
    Ok(l)
}

fn parse_unary(t: &[Tok], i: &mut usize) -> Result<String, String> {
    match t.get(*i) {
        Some(Tok::Kw("not")) => {
            *i += 1;
            let e = parse_unary(t, i)?;
            Ok(format!("(!{e})"))
        }
        Some(Tok::Sym("-")) => {
            *i += 1;
            let e = parse_unary(t, i)?;
            Ok(format!("(pm_neg({e}))"))
        }
        _ => parse_postfix(t, i),
    }
}

fn parse_postfix(t: &[Tok], i: &mut usize) -> Result<String, String> {
    let mut e = parse_atom(t, i)?;
    loop {
        match t.get(*i) {
            Some(Tok::Sym("[")) => {
                *i += 1;
                let a = parse_or(t, i)?;
                if matches!(t.get(*i), Some(Tok::Sym(":"))) {
                    *i += 1;
                    let b = if matches!(t.get(*i), Some(Tok::Sym("]"))) {
                        String::new()
                    } else {
                        parse_or(t, i)?
                    };
                    expect(t, i, "]")?;
                    e = if b.is_empty() {
                        format!("pm_slice_from(&{e}, {a})")
                    } else {
                        format!("pm_slice(&{e}, {a}, {b})")
                    };
                } else {
                    expect(t, i, "]")?;
                    e = format!("pm_index(&{e}, {a})");
                }
            }
            Some(Tok::Sym("?")) => {
                *i += 1;
                let d = parse_unary(t, i)?;
                e = format!("pm_dflt({e}, {d})");
            }
            _ => break,
        }
    }
    Ok(e)
}

fn parse_atom(t: &[Tok], i: &mut usize) -> Result<String, String> {
    if *i >= t.len() {
        return Err("unexpected end of expression".into());
    }
    match &t[*i] {
        Tok::Num(v) => {
            let v = *v;
            *i += 1;
            if v.fract() == 0.0 && v.abs() < 9.3e18 {
                Ok(format!("{}_i64", v as i64))
            } else {
                Ok(format!("{v}f64"))
            }
        }
        Tok::Kw("true") => {
            *i += 1;
            Ok("true".into())
        }
        Tok::Kw("false") => {
            *i += 1;
            Ok("false".into())
        }
        Tok::Str(lit, parts) => {
            let (lit, parts) = (lit.clone(), parts.clone());
            *i += 1;
            if parts.is_empty() {
                return Ok(format!("String::from({lit:?})"));
            }
            let mut fmtstr = String::new();
            let mut args = Vec::new();
            for p in &parts {
                match p {
                    StrPart::Lit(l) => {
                        fmtstr.push_str(&l.replace('{', "{{").replace('}', "}}"))
                    }
                    StrPart::Expr(e) => {
                        let sub = lex_line(&format!("x {e}"))?;
                        let mut si = 1;
                        let code = parse_or(&sub, &mut si)?;
                        if si != sub.len() {
                            return Err(format!("bad interpolation: {e}"));
                        }
                        fmtstr.push_str(&format!("{{{}}}", args.len()));
                        args.push(code);
                    }
                }
            }
            Ok(format!("format!({fmtstr:?}, {})", args.join(", ")))
        }
        Tok::Ident(name) => {
            let name = name.clone();
            *i += 1;
            if matches!(t.get(*i), Some(Tok::Sym("("))) {
                *i += 1;
                let mut a = Vec::new();
                if !matches!(t.get(*i), Some(Tok::Sym(")"))) {
                    loop {
                        a.push(parse_or(t, i)?);
                        match t.get(*i) {
                            Some(Tok::Sym(",")) => *i += 1,
                            _ => break,
                        }
                    }
                }
                expect(t, i, ")")?;
                Ok(format!("{name}({})", a.join(", ")))
            } else {
                Ok(name)
            }
        }
        Tok::Sym("(") => {
            *i += 1;
            let e = parse_or(t, i)?;
            expect(t, i, ")")?;
            Ok(e)
        }
        other => Err(format!("unexpected token {other:?}")),
    }
}

fn expect(t: &[Tok], i: &mut usize, s: &str) -> Result<(), String> {
    if matches!(t.get(*i), Some(Tok::Sym(x)) if *x == s) {
        *i += 1;
        Ok(())
    } else {
        Err(format!("expected {s}"))
    }
}

type Line = (usize, Vec<Tok>, usize);

fn parse_block(
    lines: &[Line],
    i: &mut usize,
    indent: usize,
    mutable: &mut HashSet<String>,
    top: bool,
) -> Result<Vec<Node>, String> {
    let mut out = Vec::new();
    while *i < lines.len() {
        let (ind, toks, ln) = &lines[*i];
        if *ind < indent {
            break;
        }
        if *ind > indent {
            return Err(format!("line {ln}: unexpected indent"));
        }
        let toks = toks.clone();
        let ln = *ln;
        *i += 1;

        match toks.first() {
            Some(Tok::Kw("f")) => {
                if !top {
                    return Err(format!(
                        "line {ln}: functions can only be declared at top level"
                    ));
                }
                if !matches!(toks.get(1), Some(Tok::Ident(_)))
                    || !matches!(toks.get(2), Some(Tok::Sym("(")))
                {
                    return Err(format!("line {ln}: f needs name(params)"));
                }
                let name = match &toks[1] {
                    Tok::Ident(n) => n.clone(),
                    _ => unreachable!(),
                };
                let mut j = 3;
                let mut params = Vec::new();
                while !matches!(toks.get(j), Some(Tok::Sym(")"))) {
                    let pn = match toks.get(j) {
                        Some(Tok::Ident(p)) => p.clone(),
                        _ => {
                            return Err(format!("line {ln}: bad param at pos {}", j - 2))
                        }
                    };
                    j += 1;
                    let pt = match toks.get(j) {
                        Some(Tok::Ident(t)) if t.len() == 1 => {
                            let r = ty(t)?;
                            j += 1;
                            r
                        }
                        _ => {
                            return Err(format!(
                                "line {ln}: param {pn} needs a type (i/f/s/b)"
                            ))
                        }
                    };
                    params.push(format!("{pn}: {pt}"));
                    match toks.get(j) {
                        Some(Tok::Sym(",")) => j += 1,
                        Some(Tok::Sym(")")) => {}
                        _ => return Err(format!("line {ln}: expected , or )")),
                    }
                }
                j += 1;
                let mut ret = None;
                if let Some(Tok::Ident(t)) = toks.get(j) {
                    if t.len() == 1 {
                        ty(t)?;
                        ret = Some(t.chars().next().unwrap());
                    } else {
                        return Err(format!("line {ln}: unknown return type {t}"));
                    }
                }
                if toks.len() != j + if ret.is_some() { 1 } else { 0 } {
                    return Err(format!("line {ln}: trailing tokens after f"));
                }
                let body = parse_block(lines, i, indent + 1, mutable, false)?;
                out.push(Node::FnDef {
                    sig: FnSig { name, params, ret },
                    body,
                });
            }
            Some(Tok::Kw("v")) | Some(Tok::Kw("m")) => {
                let ism = matches!(toks[0], Tok::Kw("m"));
                let name = match toks.get(1) {
                    Some(Tok::Ident(n)) => n.clone(),
                    _ => return Err(format!("line {ln}: v/m needs a name")),
                };
                if !matches!(toks.get(2), Some(Tok::Sym("="))) {
                    return Err(format!("line {ln}: v/m needs ="));
                }
                let mut k = 3;
                let expr = parse_expr(&toks, &mut k)
                    .map_err(|e| format!("line {ln}: {e}"))?;
                if k != toks.len() {
                    return Err(format!("line {ln}: trailing tokens"));
                }
                if ism {
                    mutable.insert(name.clone());
                }
                out.push(Node::Var { name, mutable: ism, expr });
            }
            Some(Tok::Ident(name)) if matches!(toks.get(1), Some(Tok::Sym("="))) => {
                let name = name.clone();
                if !mutable.contains(&name) {
                    return Err(format!(
                        "line {ln}: cannot assign to immutable {name}"
                    ));
                }
                let mut k = 2;
                let expr = parse_expr(&toks, &mut k)
                    .map_err(|e| format!("line {ln}: {e}"))?;
                if k != toks.len() {
                    return Err(format!("line {ln}: trailing tokens"));
                }
                out.push(Node::Assign { name, expr });
            }
            Some(Tok::Ident(n)) if n == "p" && toks.len() > 1 => {
                let mut k = 1;
                let e = parse_expr(&toks, &mut k)
                    .map_err(|e| format!("line {ln}: {e}"))?;
                if k != toks.len() {
                    return Err(format!("line {ln}: trailing tokens"));
                }
                out.push(Node::Print(e));
            }
            Some(Tok::Kw("if")) => {
                let mut k = 1;
                let cond = parse_expr(&toks, &mut k)
                    .map_err(|e| format!("line {ln}: {e}"))?;
                if k != toks.len() {
                    return Err(format!("line {ln}: trailing tokens"));
                }
                let mut arms: Vec<(Option<String>, Vec<Node>)> = Vec::new();
                let body = parse_block(lines, i, indent + 1, mutable, false)?;
                arms.push((Some(cond), body));
                loop {
                    if *i >= lines.len() {
                        break;
                    }
                    let (ind2, t2, ln2) = &lines[*i];
                    if *ind2 != indent {
                        break;
                    }
                    match t2.first() {
                        Some(Tok::Kw("elif")) => {
                            *i += 1;
                            let t2 = t2.clone();
                            let ln2 = *ln2;
                            let mut k2 = 1;
                            let c2 = parse_expr(&t2, &mut k2)
                                .map_err(|e| format!("line {ln2}: {e}"))?;
                            if k2 != t2.len() {
                                return Err(format!("line {ln2}: trailing tokens"));
                            }
                            let b2 =
                                parse_block(lines, i, indent + 1, mutable, false)?;
                            arms.push((Some(c2), b2));
                        }
                        Some(Tok::Kw("e")) => {
                            if t2.len() != 1 {
                                return Err(format!("line {ln2}: e takes nothing"));
                            }
                            *i += 1;
                            let b2 =
                                parse_block(lines, i, indent + 1, mutable, false)?;
                            arms.push((None, b2));
                            break;
                        }
                        _ => break,
                    }
                }
                out.push(Node::If { arms });
            }
            Some(Tok::Kw("w")) => {
                let mut k = 1;
                let cond = parse_expr(&toks, &mut k)
                    .map_err(|e| format!("line {ln}: {e}"))?;
                if k != toks.len() {
                    return Err(format!("line {ln}: trailing tokens"));
                }
                let body = parse_block(lines, i, indent + 1, mutable, false)?;
                out.push(Node::While { cond, body });
            }
            Some(Tok::Kw("ret")) => {
                let e = if toks.len() == 1 {
                    None
                } else {
                    let mut k = 1;
                    let e = parse_expr(&toks, &mut k)
                        .map_err(|e| format!("line {ln}: {e}"))?;
                    if k != toks.len() {
                        return Err(format!("line {ln}: trailing tokens"));
                    }
                    Some(e)
                };
                out.push(Node::Ret(e));
            }
            _ => {
                let mut k = 0;
                let e = parse_expr(&toks, &mut k)
                    .map_err(|e| format!("line {ln}: {e}"))?;
                if k != toks.len() {
                    return Err(format!("line {ln}: trailing tokens"));
                }
                out.push(Node::Expr(e));
            }
        }
    }
    Ok(out)
}

pub fn parse(src: &str, path: &str) -> Result<Prog, String> {
    let mut lines: Vec<Line> = Vec::new();
    for (idx, raw) in src.lines().enumerate() {
        let toks =
            lex_line(raw).map_err(|e| format!("{path}:{}: {}", idx + 1, e))?;
        if toks.is_empty() {
            continue;
        }
        let indent = raw.chars().take_while(|c| *c == ' ').count() / 2;
        lines.push((indent, toks, idx + 1));
    }
    let mut mutable = HashSet::new();
    let mut i = 0;
    let nodes = parse_block(&lines, &mut i, 0, &mut mutable, true)?;
    if i != lines.len() {
        return Err(format!("{path}: unexpected top-level indent"));
    }
    Ok(Prog { nodes })
}
