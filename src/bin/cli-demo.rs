use argparse::{ArgumentParser, Store};
use chrono::prelude::*;
use cli_demo::options;
use cli_demo::options::Options;
use std::env;
use std::path::PathBuf;
use std::process;
use std::process::Command;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[allow(dead_code)]
const BITCOIN_GENESIS: &str = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f";

fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

#[allow(dead_code)]
fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

//repo status CLEAN not enough
fn ensure_no_unstaged_changes(repo: &mut git2::Repository) -> Result<(), &'static str> {
    let mut opts = git2::StatusOptions::new();
    let mut m = git2::Status::empty();
    let statuses = repo.statuses(Some(&mut opts)).unwrap();

    m.insert(git2::Status::WT_NEW);
    m.insert(git2::Status::WT_MODIFIED);
    m.insert(git2::Status::WT_DELETED);
    m.insert(git2::Status::WT_RENAMED);
    m.insert(git2::Status::WT_TYPECHANGE);

    for i in 0..statuses.len() {
        let status_entry = statuses.get(i).unwrap();
        if status_entry.status().intersects(m) {
            println!("Please stash all unstaged changes before running.");
            //return Err("Please stash all unstaged changes before running.");
            process::exit(1)
        }
    }

    Ok(())
}

fn git_init(opts: &mut cli_demo::options::Options) -> core::result::Result<(), git2::Error> {
    println!("opts.message={}", opts.message);
    let now = SystemTime::now();
    println!("now={:?}", now);

    #[allow(clippy::if_same_then_else)]
    let mut command = Command::new("printenv");

    let git_author_date = "GIT_AUTHOR_DATE";
    //println!("{}", git_author_date);
    let git_author_date_value = "Thu, 01 Jan 1970 00:00:00 +0000";
    //println!("{}", git_author_date_value);
    let git_committer_date = "GIT_COMMITTER_DATE";
    //println!("{}", git_committer_date);
    let git_committer_date_value = "Thu, 01 Jan 1970 00:00:00 +0000";
    //println!("{}", git_committer_date_value);
    let git_command = "GIT_COMMAND";
    //println!("{}", git_command);
    let git_command_string = "commit --allow-empty -m 'Initial commit'";
    //println!("{}", git_command_string);

    command.env(git_author_date, git_author_date_value);
    command.env(git_committer_date, git_committer_date_value);
    command.env(git_command, git_command_string);

    let printenv = command.output().expect("failed to execute process");
    let mut _result = String::from_utf8(printenv.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    println!("{}", _result);

    let mut git_init = Command::new("git");
    git_init
        .args(["init"])
        .output()
        .expect("failed to execute process");

    let git_init = command.output().expect("failed to execute process");
    let mut _result = String::from_utf8(git_init.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    println!("git_init:{}", _result);

    let start = time::get_time();
    let epoch = get_epoch_ms();
    println!("{}", epoch);
    let system_time = SystemTime::now();

    let datetime: DateTime<Utc> = system_time.into();
    println!("{}", datetime.format("%d/%m/%Y %T/%s"));
    println!("{}", datetime.format("%d/%m/%Y %T"));

    let mut git_empty_commit = Command::new("git");
    git_empty_commit
        .args([
            "commit",
            "--allow-empty",
            "-m",
            &epoch.to_string(),
            "-m",
            &opts.message,
            "--no-gpg-sign",
        ])
        .output()
        .expect("failed to execute process");

    let git_empty_commit = command.output().expect("failed to execute process");
    let mut _result = String::from_utf8(git_empty_commit.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    println!("git_init:{}", _result);

    Ok(())
}

fn parse_args_or_exit(opts: &mut cli_demo::options::Options) {
    let mut ap = ArgumentParser::new();
    ap.set_description("a cli-demo");
    //ap.stop_on_first_argument(false);

    //ap.refer(&mut opts.repo)
    //    //.add_argument("repository-path", Store, "Path to your git repository (required)");
    //    .add_argument("repository-path", Store, "Path to your git repository");
    //    //.required();
    //ap.refer(&mut opts.repo)
    //  .add_argument("repository-path", Store, "Path to your git repository");

    ap.refer(&mut opts.repo)
        .add_option(&["-r", "--repo"], Store, "Path to git repository");
    //.required();

    ap.refer(&mut opts.message)
        .add_option(
            &["-m", "--message"],
            Store,
            "Commit message to use (required)",
        )
        .required();

    ap.parse_args_or_exit();
}

fn main() -> core::result::Result<(), git2::Error> {
    let get_pwd = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo %cd%"])
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "macos") {
        Command::new("sh")
            .arg("-c")
            .arg("echo ${PWD##*/}")
            .output()
            .expect("failed to execute process")
    } else if cfg!(target_os = "linux") {
        Command::new("sh")
            .arg("-c")
            .arg("echo ${PWD##*/}")
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo ${PWD##*/}")
            .output()
            .expect("failed to execute process")
    };

    let pwd = String::from_utf8(get_pwd.stdout)
        .map_err(|non_utf8| String::from_utf8_lossy(non_utf8.as_bytes()).into_owned())
        .unwrap();
    //println!("pwd={}", pwd);

    let mut opts = options::Options {
        message: pwd,
        repo: ".".to_string(),
    };

    parse_args_or_exit(&mut opts);

    println!("opts.message={}", opts.message);
    println!("opts.repo={}", opts.repo);

    let _init_result = git_init(&mut opts);
    Ok(())
}
