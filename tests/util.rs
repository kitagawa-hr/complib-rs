use std::fs;
use std::path::PathBuf;

use itertools::izip;

pub fn test_solution<F>(dir_path: &str, solution: F)
    where
        F: Fn(&str) -> String,
{
    let inp_dir = format!("{}/{}", dir_path, "in");
    let ans_dir = format!("{}/{}", dir_path, "out");
    let inputs = read_dir_to_strings(&inp_dir);
    let answers = read_dir_to_strings(&ans_dir);
    for (input, answer) in izip!(inputs, answers) {
        assert_eq!(answer.trim_end(), solution(&input))
    }
}

fn read_dir_to_strings(dir_path: &str) -> Vec<String> {
    let mut paths: Vec<PathBuf> = fs::read_dir(dir_path)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    paths.sort();
    paths.into_iter()
        .map(|path| fs::read_to_string(path.as_path()).unwrap())
        .collect()
}