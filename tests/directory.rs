use ansi_term::Color;
use dirs::home_dir;
use git2::Repository;
use std::fs;
use std::io;
use std::path::Path;
use tempfile::TempDir;

mod common;

#[test]
fn home_directory() -> io::Result<()> {
    let output = common::render_module("dir").current_dir("~").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("~"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn directory_in_home() -> io::Result<()> {
    let dir = home_dir().unwrap().join("starship/engine");
    fs::create_dir_all(&dir)?;

    let output = common::render_module("dir")
        .current_dir("~/starship/engine")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("~/starship/engine"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn truncated_directory_in_home() -> io::Result<()> {
    let dir = home_dir().unwrap().join("starship/engine/schematics");
    fs::create_dir_all(&dir)?;

    let output = common::render_module("dir")
        .current_dir("~/starship/engine/schematics")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan.bold().paint("starship/engine/schematics")
    );
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn root_directory() -> io::Result<()> {
    let output = common::render_module("dir").current_dir("/").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("/"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn directory_in_root() -> io::Result<()> {
    let output = common::render_module("dir").current_dir("/tmp").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("/tmp"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn truncated_directory_in_root() -> io::Result<()> {
    let dir = Path::new("/tmp/starship/thrusters/rocket");
    fs::create_dir_all(&dir)?;

    let output = common::render_module("dir")
        .current_dir("/tmp/starship/thrusters/rocket")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan.bold().paint("starship/thrusters/rocket")
    );
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn git_repo_root() -> io::Result<()> {
    // TODO: Investigate why git repo related tests fail when the tempdir is within /tmp/...
    // Temporarily making the tempdir within $HOME
    // #[ignore] can be removed after this TODO is addressed
    let tmp_dir = TempDir::new_in(dirs::home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    fs::create_dir(&repo_dir)?;
    Repository::init(&repo_dir).unwrap();

    let output = common::render_module("dir")
        .current_dir(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("rocket-controls"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn directory_in_git_repo() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(dirs::home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let dir = repo_dir.join("src");
    fs::create_dir_all(&dir)?;
    Repository::init(&repo_dir).unwrap();

    let output = common::render_module("dir").current_dir(dir).output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("rocket-controls/src"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn truncated_directory_in_git_repo() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(dirs::home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let dir = repo_dir.join("src/meters/fuel-gauge");
    fs::create_dir_all(&dir)?;
    Repository::init(&repo_dir).unwrap();

    let output = common::render_module("dir").current_dir(dir).output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("in {} ", Color::Cyan.bold().paint("src/meters/fuel-gauge"));
    assert_eq!(expected, actual);
    Ok(())
}
