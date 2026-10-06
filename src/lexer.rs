#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
    Ident(String),
    Num(f64),
    Str(String, Vec<StrPart>),
    Kw(&'static str),
    Sym(&'static str),
}

#[derive(Debug, Clone, PartialEq)]
pub enum StrPart {
    Lit(String),
    Expr(String),
}

pub fn lex_line(line: &str) -> Result<Vec<Tok>, String> {
    let b: Vec<char> = line.chars().collect();
    let mut toks = Vec::new();
    let mut i = 0;
    let n = b.len();
    while i < n {
        let c = b[i];
        if c == ' ' || c == '\t' {
            i += 1;
            continue;
        }
        if c == '#' {
            break;
        }
        if c == '"' {
            i += 1;
            let mut parts = Vec::new();
            let mut lit = String::new();
            while i < n && b[i] != '"' {
                if b[i] == '{' {
                    // find matching close brace, tracking nesting
                    if !lit.is_empty() {
                        parts.push(StrPart::Lit(std::mem::take(&mut lit)));
                    }
                    let mut depth = 0;
                    let mut ex = String::new();
                    while i < n {
                        if b[i] == '{' {
                            depth += 1;
                            if depth == 1 {
                                i += 1;
                                continue;
                            }
                        } else if b[i] == '}' {
                            depth -= 1;
                            if depth == 0 {
                                i += 1;
                                break;
                            }
                        }
                        ex.push(b[i]);
                        i += 1;
                    }
                    if depth != 0 {
                        return Err("unterminated interpolation".into());
                    }
                    parts.push(StrPart::Expr(ex.trim().to_string()));
                    continue;
                }
                if b[i] == '\\' && i + 1 < n {
                    i += 1;
                    let e = b[i];
                    lit.push(match e {
                        'n' => '\n',
                        't' => '\t',
                        '"' => '"',
                        '{' => '{',
                        '\\' => '\\',
                        other => other,
                    });
                    i += 1;
                    continue;
                }
                lit.push(b[i]);
                i += 1;
            }
            if i >= n {
                return Err("unterminated string".into());
            }
            i += 1;
            if parts.is_empty() {
                toks.push(Tok::Str(lit, Vec::new()));
            } else {
                if !lit.is_empty() {
                    parts.push(StrPart::Lit(lit));
                }
                toks.push(Tok::Str(String::new(), parts));
            }
            continue;
        }
        if c.is_ascii_digit()
            || (c == '-' && i + 1 < n && b[i + 1].is_ascii_digit() && toks.is_empty())
        {
            let start = i;
            if c == '-' {
                i += 1;
            }
            let mut isf = false;
            while i < n && (b[i].is_ascii_digit() || b[i] == '.') {
                if b[i] == '.' {
                    if isf {
                        break;
                    }
                    isf = true;
                }
                i += 1;
            }
            let s: String = b[start..i].iter().collect();
            let v: f64 = s.parse().map_err(|_| "bad number")?;
            toks.push(Tok::Num(v));
            continue;
        }
        if c.is_alphabetic() || c == '_' {
            let start = i;
            while i < n && (b[i].is_alphanumeric() || b[i] == '_') {
                i += 1;
            }
            let w: String = b[start..i].iter().collect();
            let kw: Option<&'static str> = match w.as_str() {
                "f" => Some("f"),
                "v" => Some("v"),
                "m" => Some("m"),
                "if" => Some("if"),
                "elif" => Some("elif"),
                "e" => Some("e"),
                "w" => Some("w"),
                "ret" => Some("ret"),
                "and" => Some("and"),
                "or" => Some("or"),
                "not" => Some("not"),
                "true" => Some("true"),
                "false" => Some("false"),
                "t" => Some("t"),
                "each" => Some("each"),
                _ => None,
            };
            match kw {
                Some(k) => toks.push(Tok::Kw(k)),
                None => toks.push(Tok::Ident(w)),
            }
            continue;
        }
        // multi-char symbols
        let two: String = b[i..(i + 2).min(n)].iter().collect();
        let sym2: Option<&'static str> = match two.as_str() {
            "==" => Some("=="),
            "!=" => Some("!="),
            "<=" => Some("<="),
            ">=" => Some(">="),
            "&&" => Some("&&"),
            "||" => Some("||"),
            _ => None,
        };
        if let Some(s) = sym2 {
            toks.push(Tok::Sym(s));
            i += 2;
            continue;
        }
        let sym1: Option<&'static str> = match c {
            '+' => Some("+"),
            '-' => Some("-"),
            '*' => Some("*"),
            '/' => Some("/"),
            '%' => Some("%"),
            '(' => Some("("),
            ')' => Some(")"),
            ',' => Some(","),
            '=' => Some("="),
            '<' => Some("<"),
            '>' => Some(">"),
            '[' => Some("["),
            ']' => Some("]"),
            ':' => Some(":"),
            '?' => Some("?"),
            '{' => Some("{"),
            '}' => Some("}"),
            '.' => Some("."),
            _ => None,
        };
        match sym1 {
            Some(s) => toks.push(Tok::Sym(s)),
            None => return Err(format!("unexpected character {c}")),
        }
        i += 1;
    }
    Ok(toks)
}
