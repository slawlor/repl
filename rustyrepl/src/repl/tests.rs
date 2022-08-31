// Copyright (c) Sean Lawlor
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use super::*;
use anyhow::Result;
use clap::Parser;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
struct TestCli {}

type TestRepl = Repl::<TestCli>;

#[test]
fn test_history_path_parsing() -> Result<()> {
    // ========= None ========= //
    let no_path: Option<PathBuf> = TestRepl::get_history_file_path(None);
    assert_eq!(None, no_path);

    // ========= Just a filename ========= //
    let just_a_filename = TestRepl::get_history_file_path(Some("a_test_file.txt".to_string()));
    let mut home_dir = dirs::home_dir().unwrap();
    home_dir.push("a_test_file.txt");
    assert_eq!(home_dir, just_a_filename.unwrap());

    // ========= A real file ========= //
    let mut tempfile = tempfile::NamedTempFile::new()?;
    // extract the tempfile name
    let real_file: String = tempfile.path().to_path_buf().to_str().unwrap().to_string();
    // write some dummy data to the file + close it
    tempfile.write_all("some_test_data".as_bytes())?;
    info!("The tempfile is {}", real_file);

    let relative_path_to_real_file = TestRepl::get_history_file_path(Some(real_file.clone()));
    tempfile.close()?;
    assert_eq!(Path::new(&real_file).to_path_buf(), relative_path_to_real_file.unwrap());

    // ========= A directory ========= //
    let mut tempdir = tempfile::tempdir()?.into_path();
    let directory_plus_default_filename = TestRepl::get_history_file_path(Some(tempdir.to_str().unwrap().to_string()));
    tempdir.push(super::DEFAULT_HISTORY_FILE_NAME);
    assert_eq!(tempdir, directory_plus_default_filename.unwrap());

    // ========= Bad paths ========= //
    let bad_path = "/some/fake/path.txt".to_string();
    let no_file = TestRepl::get_history_file_path(Some(bad_path));
    assert_eq!(None, no_file);

    Ok(())
}
