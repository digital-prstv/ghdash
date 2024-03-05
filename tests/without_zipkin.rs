#[test]
fn without_zipkin_tests() {
    trycmd::TestCases::new()
        .case("tests/logging/*.trycmd")
        .insert_var("[MESSAGE]", "logging")
        .unwrap();
}
