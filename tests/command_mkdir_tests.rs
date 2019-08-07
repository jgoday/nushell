mod helpers;

use h::{in_directory as cwd, Playground};
use helpers as h;
use std::path::PathBuf;

#[test]
fn creates_directory() {
    let sandbox = Playground::setup_for("mkdir_test").test_dir_name();

    let full_path = format!("{}/{}", Playground::root(), sandbox);

    nu!(_output, cwd(&full_path), "mkdir my_new_directory");

    let mut expected = PathBuf::from(full_path);
    expected.push("my_new_directory");

    assert!(h::dir_exists_at(expected));
}

#[test]
fn error_if_intermediary_directory_doesnt_exist() {
    let sandbox = Playground::setup_for("mkdir_test_2").test_dir_name();

    let full_path = format!("{}/{}", Playground::root(), sandbox);

    nu_error!(
        output,
        cwd(&full_path),
        "mkdir some_folder/another/deeper_one"
    );

    assert!(output.contains("some_folder/another/deeper_one"));
    assert!(output.contains("No such file or directory"));
}

#[test]
fn creates_intermediary_directories_with_p_flag() {
    let sandbox = Playground::setup_for("mkdir_test_3").test_dir_name();

    let full_path = format!("{}/{}", Playground::root(), sandbox);

    nu!(
        _output,
        cwd(&full_path),
        "mkdir some_folder/another/deeper_one --create-all"
    );

    let mut expected = PathBuf::from(full_path);
    expected.push("some_folder/another/deeper_one");

    assert!(h::dir_exists_at(expected));
}
