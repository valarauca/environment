use super::regex::Regex;

lazy_static! {
    static ref DEC: Regex = Regex::new(r#"^([-|+]?[0-9]{1,19})$"#).unwrap();
    static ref HEX: Regex = Regex::new(r#"^0x([A-Fa-f0-9]{1,16})$"#).unwrap();
    static ref OCTAL: Regex = Regex::new(r#"^0o([0-7]{1,32})$"#).unwrap();
    static ref BOOL: Regex = Regex::new(r#"^0b([01]{1,64})$"#).unwrap();
}

pub fn parse_int<S: AsRef<str>>(arg: &S) -> Option<i64> {
    Option::None
        .into_iter()
        .chain(parse_base_10(arg))
        .chain(parse_base_16(arg))
        .chain(parse_base_8(arg))
        .chain(parse_base_2(arg))
        .next()
}

fn parse_base_10<S: AsRef<str>>(arg: &S) -> Option<i64> {
    boilerplate(&DEC, arg.as_ref(), 10)
}

fn parse_base_16<S: AsRef<str>>(arg: &S) -> Option<i64> {
    boilerplate(&HEX, arg.as_ref(), 16)
}

fn parse_base_8<S: AsRef<str>>(arg: &S) -> Option<i64> {
    boilerplate(&OCTAL, arg.as_ref(), 8)
}

fn parse_base_2<S: AsRef<str>>(arg: &S) -> Option<i64> {
    boilerplate(&BOOL, arg.as_ref(), 2)
}
fn boilerplate(regex: &Regex, data: &str, base: u32) -> Option<i64> {
    regex
        .captures(data)
        .into_iter()
        .flat_map(|captures| captures.get(1))
        .flat_map(|group_one| i64::from_str_radix(group_one.as_str(), base).ok())
        .next()
}

#[test]
fn test_parse_base_10() {
    let dut0 = "10";
    assert_eq!(parse_base_10(&dut0), Some(10i64));
    let dut1 = "+10";
    assert_eq!(parse_base_10(&dut1), Some(10i64));
    let dut2 = "-10";
    assert_eq!(parse_base_10(&dut2), Some(-10i64));
}

#[test]
fn test_parse_base_16() {
    let dut0 = "0xA";
    assert_eq!(parse_base_16(&dut0), Some(10i64));
}
