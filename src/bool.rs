use super::regex::Regex;

lazy_static! {
    static ref BOOL: Regex = Regex::new(r#"^([tT]|[fF]|true|false|TRUE|FALSE)$"#).unwrap();
}

pub fn parse_bool<S: AsRef<str>>(arg: &S) -> Option<bool> {
    BOOL.captures(arg.as_ref())
        .into_iter()
        .flat_map(|captures| captures.get(1))
        .flat_map(|capture_group| match capture_group.as_str() {
            "t" | "T" | "true" | "TRUE" => Some(true),
            "f" | "F" | "false" | "FALSE" => Some(false),
            _ => None,
        })
        .next()
}

#[test]
fn test_parse_bool() {
    let dut0 = "t";
    assert_eq!(parse_bool(&dut0), Some(true));
    let dut1 = "f";
    assert_eq!(parse_bool(&dut1), Some(false));
    let dut2 = "true";
    assert_eq!(parse_bool(&dut2), Some(true));
    let dut3 = "false";
    assert_eq!(parse_bool(&dut3), Some(false));
    let dut4 = "T";
    assert_eq!(parse_bool(&dut4), Some(true));
    let dut5 = "F";
    assert_eq!(parse_bool(&dut5), Some(false));
    let dut6 = "TRUE";
    assert_eq!(parse_bool(&dut6), Some(true));
    let dut7 = "FALSE";
    assert_eq!(parse_bool(&dut7), Some(false));
}
