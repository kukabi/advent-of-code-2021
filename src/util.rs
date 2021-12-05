use std::str::FromStr;
use std::vec::IntoIter;

pub(crate) fn parse_input<T>(path: &str) -> IntoIter<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|a| a.parse::<T>().unwrap())
        .collect::<Vec<T>>()
        .into_iter()
}
