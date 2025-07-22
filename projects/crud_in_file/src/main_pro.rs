use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write, Seek, SeekFrom};
use std::path::PathBuf;
use std::fmt;

#[derive(Debug)]
enum CrudError {
    Io(io::Error),
    InvalidLineNumber,
    FileNotFound,
}

impl fmt::Display for CrudError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CrudError::Io(e) => write!(f, "I/O error: {}", e),
            CrudError::InvalidLineNumber => write!(f, "Invalid line number"),
            CrudError::FileNotFound => write!(f, "File not found"),
        }
    }
}

impl From<io::Error> for CrudError {
    fn from(error: io::Error) -> Self {
        CrudError::Io(error)
    }
}

struct TextFileDb {
    path: PathBuf,
}

impl TextFileDb {
    fn new(path: &str) -> Self {
        TextFileDb {
            path: PathBuf::from(path),
        }
    }

    fn create(&self, content: &str) -> Result<(), CrudError> {
        fs::write(&self.path, content)?;
        Ok(())
    }

    fn read(&self) -> Result<Vec<String>, CrudError> {
        if !self.path.exists() {
            return Err(CrudError::FileNotFound);
        }

        let file = File::open(&self.path)?;
        let lines = BufReader::new(file)
            .lines()
            .collect::<Result<Vec<_>, _>>()?;
        Ok(lines)
    }

    fn update_line(&mut self, line_no: usize, new_text: &str) -> Result<(), CrudError> {
        let mut lines = self.read()?;
        
        if line_no == 0 || line_no > lines.len() {
            return Err(CrudError::InvalidLineNumber);
        }

        lines[line_no - 1] = new_text.to_string();
        self.write_all(&lines)?;
        Ok(())
    }

    fn delete_line(&mut self, line_no: usize) -> Result<(), CrudError> {
        let mut lines = self.read()?;
        
        if line_no == 0 || line_no > lines.len() {
            return Err(CrudError::InvalidLineNumber);
        }

        lines.remove(line_no - 1);
        self.write_all(&lines)?;
        Ok(())
    }

    fn write_all(&self, lines: &[String]) -> Result<(), CrudError> {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.path)?;
        
        file.seek(SeekFrom::Start(0))?;
        for line in lines {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), CrudError> {
    let mut db = TextFileDb::new("notes.txt");

    loop {
        println!("\n📝 Text File CRUD Menu:");
        println!("1. Create (overwrite)");
        println!("2. Read");
        println!("3. Update line");
        println!("4. Delete line");
        println!("5. Exit");

        let choice = input("Choose an option: ")?;

        match choice.as_str() {
            "1" => {
                let content = input("Enter new content: ")?;
                db.create(&content)?;
                println!("✅ File overwritten.");
            }
            "2" => match db.read() {
                Ok(lines) => {
                    println!("📄 File Content:");
                    for (i, line) in lines.iter().enumerate() {
                        println!("{}: {}", i + 1, line);
                    }
                }
                Err(CrudError::FileNotFound) => println!("📭 File not found."),
                Err(e) => return Err(e),
            },
            "3" => {
                let line_no = input("Line to update: ")?.parse().unwrap_or(0);
                let new_text = input("New content: ")?;
                match db.update_line(line_no, &new_text) {
                    Ok(_) => println!("✅ Line {} updated.", line_no),
                    Err(CrudError::InvalidLineNumber) => println!("❌ Invalid line number."),
                    Err(e) => return Err(e),
                }
            }
            "4" => {
                let line_no = input("Line to delete: ")?.parse().unwrap_or(0);
                match db.delete_line(line_no) {
                    Ok(_) => println!("✅ Line {} deleted.", line_no),
                    Err(CrudError::InvalidLineNumber) => println!("❌ Invalid line number."),
                    Err(e) => return Err(e),
                }
            }
            "5" => break,
            _ => println!("❌ Invalid choice."),
        }
    }

    Ok(())
}

fn input(prompt: &str) -> Result<String, CrudError> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}