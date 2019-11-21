use super::regex::Regex;

const FLOAT_STRING: &str = "^([+-]?(inf|Nan|((([0-9]{1,17})|([0-9]{1,17}.[0-9]{1,17})|([0-9]{0,17}.[0-9]{1,17}))[eE]?[+-]?[0-9]{0,17})))$";

lazy_static! {
    static ref FLOAT_NUM_DEC: Regex = Regex::new(FLOAT_STRING).unwrap();
}

pub fn parse_float<S: AsRef<str>>(arg: &S) -> Option<f64> {
    FLOAT_NUM_DEC
        .captures(arg.as_ref())
        .into_iter()
        .flat_map(|captures| captures.get(1))
        .flat_map(|capture_group| {
            <f64 as ::std::str::FromStr>::from_str(capture_group.as_str()).ok()
        })
        .next()
}

#[test]
fn test_float_parse() {
    // sanity tests are taken from the f64::from_str documentation page

    let dut0 = "3.14";
    assert_eq!(parse_float(&dut0), Option::Some(3.14f64));

    let dut1 = "-3.14";
    assert_eq!(parse_float(&dut1), Option::Some(-3.14f64));

    let dut2 = "2.5E10";
    assert_eq!(parse_float(&dut2), Option::Some(2.5E10f64));

    let dut3 = "-2.5e-10";
    assert_eq!(parse_float(&dut3), Option::Some(-2.5e-10f64));

    let dut4 = ".5";
    assert_eq!(parse_float(&dut4), Option::Some(0.5f64));

    let dut5 = "0.5";
    assert_eq!(parse_float(&dut5), parse_float(&dut4));
}
