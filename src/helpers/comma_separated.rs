use std::fmt;

pub fn comma_separated(exps: &[impl fmt::Display]) -> String {
  exps
    .iter()
    .map(|a| a.to_string())
    .collect::<Vec<String>>()
    .join(", ")
}
