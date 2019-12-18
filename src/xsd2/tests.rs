use super::*;

fn check_valid(s: &str) {
    match duration::Duration::from_lexical_representation(s) {
        Ok(_) => assert!(true),
        Err(err) => assert!(false, "{} should be valid, but an error occurred: {}", s, err),
    }
}

fn check_invalid(s: &str) {
    match duration::Duration::from_lexical_representation(s) {
        Ok(dur) => assert!(false, "{} should be invalid", s),
        Err(_) => assert!(true),
    }
}

#[test]
fn duration_parsing_test() {
    check_valid("P2Y6M5DT12H35M30S");
    check_valid("P1DT2H");
    check_valid("P20M");
    check_valid("PT20M");
    check_valid("P0Y20M0D");
    check_valid("P0Y");
    check_valid("-P60D");
    check_valid("PT1M30.5S");

    check_invalid("P-20M");
    check_invalid("P20MT");
    check_invalid("P1YM5D");
    check_invalid("P15.5Y");
    check_invalid("P1D2H");
    check_invalid("1Y2M");
    check_invalid("P2M1Y");
    check_invalid("P");
    check_invalid("PT15.S");
}
