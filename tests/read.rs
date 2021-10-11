use json_parser::File;
use std::{
    io::{BufRead, BufReader},
    path::Path,
};

#[test]
fn read_all() {
    let file_op = File::new(Path::new("tests/sample.txt"));

    if let Some(file_op) = file_op {
        let f = file_op.read_all().unwrap();
        assert_eq!(f, "hoge\nfuga\nfoo\n")
    }
}

#[test]
fn read_line() {
    let file_op = File::new(Path::new("tests/sample.txt"));

    if let Some(file_op) = file_op {
        let mut lines_iter = BufReader::new(file_op.read_line().unwrap())
            .lines()
            .map(|l| l.unwrap());

        assert_eq!(lines_iter.next(), Some(String::from("hoge")));
        assert_eq!(lines_iter.next(), Some(String::from("fuga")));
        assert_eq!(lines_iter.next(), Some(String::from("foo")));
    }
}
