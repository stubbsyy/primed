use std::process::exit;

mod gen;
mod lexer;
mod parser;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() < 3 {
        eprintln!("usage: primed run|build|transpile file.pm [args... | -o out]");
        exit(2);
    }
    let cmd = argv[1].as_str();
    let file = &argv[2];
    let src = match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("primed: cannot read {file}: {e}");
            exit(2);
        }
    };
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
