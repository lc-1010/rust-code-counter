#![feature(fs_try_exists)]

use std::env;
use std::fmt::{write, Display, Error};
use std::fs::File;
use std::io::{self, prelude::*};
use std::path::{Path, PathBuf};

const CHECKDIR: &str = "../";

enum OpenErr {
    PATHERROR(String),
    FILEERROR(String),
    OTHER(String),
}

#[derive(Debug)]
struct ProjectInfo {
    name: String,
    path: PathBuf,
    code_line: i32,
}

impl Display for OpenErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FILEERROR(s) => {
                write!(f, "{}", s)
            }
            Self::OTHER(s) => {
                write!(f, "{}", s)
            }
            Self::PATHERROR(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let user_path = if args.len() < 2 {
        CHECKDIR
    } else {
        args[1].as_str()
    };

    let path = Path::new(user_path);
    let display = path.display();
    println!("目录=>\n{}", display);
    if !path.is_dir() {
        let err = format!(
            "{}:{}",
            OpenErr::PATHERROR(String::from("can't open path")),
            display
        );
        panic!("{}", err);
    }

    //let mut counter:Vec<(String,i32)>=vec![];
    let mut all = 0;
    if let Ok(projects) = get_projects_list(path) {
        for item in projects.iter() {
            let lines = get_lines(&item.path);
            println!("---> {}-{}", item.name, lines);
            all = lines + all;
        }
    } else {
        let err = format!("{}", OpenErr::PATHERROR(String::from("can't open path")));
        panic!("{}", err)
    }
    println!("all  lines:{}", all);
    Ok(())
}

fn get_projects_list(parent_path: &Path) -> Result<Vec<ProjectInfo>, Error> {
    let dir_len = parent_path.display().to_string().len() + 1;
    let mut res: Vec<ProjectInfo> = vec![];
    for entry in parent_path.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                let project_name = path.to_str().unwrap().get(dir_len..);
                let name = match project_name {
                    Some(s) => s.to_string(),
                    None => "".to_string(),
                };
                if !name.is_empty() {
                    res.push(ProjectInfo {
                        name,
                        path,
                        code_line: 0,
                    });
                }
            }
        }
    }

    Ok(res)
}

fn get_lines(entry_name: &Path) -> i32 {
    let mut line = 0;
    if entry_name.is_dir() {
        let entry_string = entry_name.to_str().unwrap().to_string();
        let src = "/src/main.rs";
        let entry_src = format!("{}{}", entry_string, src);
        let src = Path::new(&entry_src);
        if src.is_file() {
            line = open_file(&entry_src);
        } else {
            println!("{} is not file ", entry_name.display());
        }
    } else {
        println!("{} is not dir ", entry_name.display());
    }
    line
}
fn open_file(path: &str) -> i32 {
    //println!("open file =>{}",path);
    // let  file = match File::open(&path){
    //     Err(why) =>{
    //         panic!("counldn't open{}:{}",display,why)
    //     },
    //     Ok(file) => file,
    // };
    let mut counter = 0;
    if let Ok(lines) = read_line(path) {
        for line in lines {
            if let Ok(code) = line {
                if !code.is_empty() {
                    counter = counter + 1;
                }
            }
        }
    }

    counter
}

fn read_line<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
