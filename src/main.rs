use std::fmt::Display;
use std::fmt::Write;
use std::fs;
use std::env;
use std::io;
use std::io::Read;
use std::io::Write as IOWrite;

type IOResult<T> = std::io::Result<T>;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(e) = run(){
        let _ = write!(io::stderr(), "Error: {}", e);
    }
}

fn run() -> Result<()> {
    let file = env::args().nth(1).unwrap_or("-".to_string());

    let chars = match file.as_str() {
        "-" => string_from_stdin(),
        _ => fs::read_to_string(file),
    }?;
    let mut chars_iter = chars.chars();

    let headers = read_headers(&mut chars_iter)?;
    read_items(chars_iter, headers)?;

    Ok(())
}

fn read_items(chars_iter: std::str::Chars, headers: Vec<String>) -> Result<()> {
    let mut col = String::new();
    let mut row: Vec<Value> = Vec::new();

    let mut row_num = 0;

    let mut stdout = io::stdout();
    write!(stdout, "[")?;

    for c in chars_iter {
        match c {
            ',' | '\n' => {
                row.push(col.into());
                col = String::new();
                if c == '\n' {

                    if row_num > 0 {
                        write!(stdout, ",\n  {{")?;
                    } else {
                        write!(stdout, "\n  {{")?;
                    }

                    for (col, (key, val)) in headers.iter().zip(row).enumerate() {
                        if col > 0 {
                            write!(stdout, ", ")?;
                        }
                        write!(stdout, "\"{}\": {}", key, val)?;
                    }

                    write!(stdout, "}}")?;

                    row_num += 1;
                    row = Vec::new();
                }
            },
            '\r' => (),
            _ => col.write_char(c)?
        }
    }
    if row_num > 0 {
        writeln!(stdout, "\n]")?;
    } else {
        writeln!(stdout, "]")?;
    }

    Ok(())
}

fn read_headers(chars_iter: &mut std::str::Chars) -> Result<Vec<String>> {
    let mut headers: Vec<String> = Vec::new();
    let mut col = String::new();
    for c in chars_iter.by_ref() {
        match c {
            ',' | '\n' => {
                headers.push(col);
                if c == '\n' {
                    break;
                }
                col = String::new();
            },
            '\r' => (),
            _ => col.write_char(c)?,
        }
    }
    return Ok(headers);
}

fn string_from_stdin() -> IOResult<String> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).and(Ok(buf))
}

enum Value {
    Bool(bool),
    String(String)
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => {
                f.write_char('"')?;
                f.write_str(&s)?;
                f.write_char('"')?;
                Ok(())
            }
            Self::Bool(true) => f.write_str("true"),
            Self::Bool(false) => f.write_str("false"),
        }
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        match v.as_str() {
            "true" => Value::Bool(true),
            "false" => Value::Bool(false),
            _ => Value::String(v),
        }
    }
}