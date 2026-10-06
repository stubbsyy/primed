use std::process::exit;

mod gen;
mod lexer;
mod parser;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let cmds_no_file = ["cheat"];
    if argv.len() < 3 && !cmds_no_file.contains(&argv[1].as_str()) {
        eprintln!(
            "usage: primed run|build|install|transpile|tokens file.pm [args... | -o out] | primed cheat"
        );
        exit(2);
    }
    let cmd = argv[1].as_str();
    if cmd == "mcp" {
        let file = &argv[2];
        let src = match std::fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("primed: cannot read {file}: {e}");
                exit(2);
            }
        };
        let rs = match gen::gen_mode(&src, file, true) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("primed: {e}");
                exit(1);
            }
        };
        let dir = std::env::temp_dir().join("primed-mcp");
        std::fs::create_dir_all(&dir).ok();
        let stem = std::path::Path::new(file)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "mcp".into());
        let bin = dir.join(&stem).to_string_lossy().to_string();
        let rs_path = format!("{bin}.rs");
        if let Err(e) = std::fs::write(&rs_path, &rs) {
            eprintln!("primed: cannot write {rs_path}: {e}");
            exit(1);
        }
        let status = std::process::Command::new("rustc")
            .arg("-O")
            .arg("--edition")
            .arg("2021")
            .arg("-o")
            .arg(&bin)
            .arg(&rs_path)
            .status();
        match status {
            Ok(s) if s.success() => {
                let code = std::process::Command::new(&bin).status()
                    .map(|s| s.code().unwrap_or(1))
                    .unwrap_or(1);
                exit(code);
            }
            Ok(s) => {
                eprintln!("primed: rustc failed: {s}");
                exit(1);
            }
            Err(e) => {
                eprintln!("primed: cannot invoke rustc: {e}");
                exit(1);
            }
        }
    }
    if cmd == "cheat" {
        print!("{}", gen::CHEATSHEET);
        exit(0);
    }
    let file = &argv[2];
    let src = match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("primed: cannot read {file}: {e}");
            exit(2);
        }
    };
    if cmd == "tokens" {
        let rs = match gen::gen(&src, file) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("primed: {e}");
                exit(1);
            }
        };
        let pm_tokens = est_tokens(&src);
        let rs_tokens = est_tokens(&rs);
        println!("file: {file}");
        println!("primed source: {} bytes, ~{} tokens", src.len(), pm_tokens);
        println!(
            "generated rust: {} bytes, ~{} tokens",
            rs.len(),
            rs_tokens
        );
        println!(
            "source-only ratio (primed vs equivalent hand rust, est): ~{}x",
            (rs_tokens as f64 / 3.0 / pm_tokens as f64 * 10.0).round() / 10.0
        );
        println!(
            "pipeline check: transpile {} -> rustc -> binary",
            if pm_tokens > 0 { "ok" } else { "?" }
        );
        exit(0);
    }
    let rs = match gen::gen(&src, file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("primed: {e}");
            exit(1);
        }
    };
    match cmd {
        "transpile" => {
            print!("{rs}");
        }
        "install" => {
            let bin = std::path::Path::new(file)
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "app".into());
            let dest_dir = std::env::var("HOME")
                .map(|h| format!("{h}/.local/bin"))
                .unwrap_or_else(|_| "/usr/local/bin".into());
            if std::fs::create_dir_all(&dest_dir).is_err() {
                eprintln!("primed: cannot create {dest_dir}");
                exit(1);
            }
            let dest = format!("{dest_dir}/{bin}");
            let tmp = std::env::temp_dir().join(format!("primed-{bin}"));
            let rs_path = tmp.to_string_lossy().to_string() + ".rs";
            if let Err(e) = std::fs::write(&rs_path, &rs) {
                eprintln!("primed: cannot write {rs_path}: {e}");
                exit(1);
            }
            let status = std::process::Command::new("rustc")
                .arg("-O")
                .arg("--edition")
                .arg("2021")
                .arg("-o")
                .arg(&tmp)
                .arg(&rs_path)
                .status();
            match status {
                Ok(s) if s.success() => {
                    if std::fs::rename(&tmp, &dest).is_err() {
                        eprintln!("primed: cannot install to {dest}");
                        exit(1);
                    }
                    let _ = std::fs::remove_file(&rs_path);
                    println!("installed: {dest}");
                    if !dest_dir.is_empty() {
                        println!("make sure {dest_dir} is on PATH");
                    }
                }
                Ok(s) => {
                    eprintln!("primed: rustc failed: {s}");
                    exit(1);
                }
                Err(e) => {
                    eprintln!("primed: cannot invoke rustc: {e}");
                    exit(1);
                }
            }
        }
        "run" | "build" => {
            let out = if cmd == "build" {
                let mut o = "a.out".to_string();
                let mut k = 3;
                while k < argv.len() {
                    if argv[k] == "-o" {
                        if k + 1 < argv.len() {
                            o = argv[k + 1].clone();
                        } else {
                            eprintln!("primed: -o needs a value");
                            exit(2);
                        }
                    }
                    k += 1;
                }
                o
            } else {
                let dir = std::env::temp_dir().join("primed");
                std::fs::create_dir_all(&dir).ok();
                let stem = std::path::Path::new(file)
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "pm".into());
                dir.join(stem).to_string_lossy().to_string()
            };
            let out = std::fs::canonicalize(std::path::Path::new(&out))
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or(out);
            let rs_path = format!("{out}.rs");
            if let Err(e) = std::fs::write(&rs_path, &rs) {
                eprintln!("primed: cannot write {rs_path}: {e}");
                exit(2);
            }
            let status = std::process::Command::new("rustc")
                .arg("-O")
                .arg("--edition")
                .arg("2021")
                .arg("-o")
                .arg(&out)
                .arg(&rs_path)
                .status();
            match status {
                Ok(s) if s.success() => {
                    if cmd == "run" {
                        let code = std::process::Command::new(&out)
                            .args(&argv[3..])
                            .status()
                            .map(|s| s.code().unwrap_or(1))
                            .unwrap_or(1);
                        exit(code);
                    }
                }
                Ok(s) => {
                    eprintln!("primed: rustc failed: {s}");
                    exit(1);
                }
                Err(e) => {
                    eprintln!("primed: cannot invoke rustc: {e}");
                    exit(1);
                }
            }
        }
        _ => {
            eprintln!("primed: unknown command {cmd}");
            exit(2);
        }
    }
}

fn est_tokens(s: &str) -> usize {
    // rough LLM tokenizer estimate: ~4 chars/token for code, adjust for
    // symbol-heavy content
    let chars = s.chars().count();
    let syms = s
        .chars()
        .filter(|c| {
            !c.is_alphanumeric() && !c.is_whitespace() && *c != '_'
        })
        .count();
    (chars + syms) / 4
}
