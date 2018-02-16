#[macro_use]
extern crate structopt;

use std::result;
use std::error::Error;
use std::fs;
use std::process;
use std::path::PathBuf;
use structopt::StructOpt;

type Result<T> = result::Result<T, Box<Error>>;

#[derive(StructOpt, Debug)]
#[structopt(name = "mkflashdriverepo", about = "Create a new flashdrive repo")]
struct Opts {
    #[structopt(help = "Name to create")]
    name: String,
    #[structopt(short = "p", long = "path", default_value = "/Volumes/PORTABLE/gitrepos", parse(from_os_str))]
    path: PathBuf,
    #[structopt(short = "f", long = "force")]
    force: bool,
    #[structopt(long = "no-add-origin")]
    no_add_origin: bool,
}

fn main() {
    let opts = Opts::from_args();
    if let Err(e) = run(opts) {
        eprintln!("Error: {:?}", e);
        ::std::process::exit(1);
    }
}

fn run(opts: Opts) -> Result<()> {
    let path = opts.path.as_path();
    if !path.is_dir() {
        let s = path.to_str().unwrap();
        return Err(format!("Path {} does not exist", s).into());
    }

    let newpath = path.join(format!("{}.git", opts.name));
    if newpath.exists() {
        if !opts.force {
            return Err(format!("Repo {} exists. Use -f/--force to overwrite",
                               newpath.to_str().unwrap()).into());
        } else {
            fs::remove_dir_all(&newpath)?;
        }
    }

    fs::create_dir_all(&newpath)?;

    process::Command::new("git")
        .args(&["init", "--bare"])
        .current_dir(&newpath)
        .spawn()?;

    if !opts.no_add_origin {
        process::Command::new("git")
            .args(&["remote", "add", "origin", newpath.to_str().unwrap()])
            .spawn()?;
    } else {
        println!("{}", newpath.to_str().unwrap());
    }

    Ok(())
}
