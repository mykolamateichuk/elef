use std::fmt::Display;
use std::env;
use std::fs;
use std::error::Error;

pub struct Config {
    pub file_path: String,
    pub show_in_bytes: bool,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("Wrong number of arguments passed")
        }

        let show_in_bytes = env::var("SHOW_IN_BYTES").is_ok();

        Ok(Config {
            file_path: args[1].clone(),
            show_in_bytes: show_in_bytes
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file_path)?;
    
    let size = count_size(file_contents);

    let fmt_size = if !config.show_in_bytes {
        SizeFmt::create(size)
    }
    else {
        SizeFmt::create_in_bytes(size)
    };

    println!("{}", fmt_size);

    Ok(())
}

fn count_size(contents: String) -> usize {
    let mut counter: usize = 0;

    for _ in contents.chars() {
        counter += 1;
    }

    counter
}

struct SizeFmt {
    size: f32,
    postfix: String
}

impl Display for SizeFmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.size == self.size.trunc() {
            write!(f, "{}{}", self.size, self.postfix)
        } else {
            write!(f, "{:.2}{}", self.size, self.postfix)
        }
    }
}

impl SizeFmt {
    fn create(size_in_bytes: usize) -> Self {
        let mut degree: u8 = 0;

        let mut byte_size = size_in_bytes.clone() as f32;

        while byte_size >= 1024 as f32 {
            byte_size = byte_size / 1024 as f32;
            degree += 1;
        }

        let postfix = match degree {
            0 => String::from("b"),
            1 => String::from("Kb"),
            2 => String::from("Mb"),
            3 => String::from("Gb"),
            _ => String::from("*b")
        };

        let size = byte_size;

        SizeFmt {
            size,
            postfix
        }
    }

    fn create_in_bytes(size_in_bytes: usize) -> Self {
        SizeFmt {
            size: size_in_bytes as f32,
            postfix: String::from("b")
        }
    }
}