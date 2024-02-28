#[test]
fn with_zipkin_tests() {
    trycmd::TestCases::new()
        .case("tests/with_zipkin/*.trycmd")
        .insert_var("[MESSAGE]", "tracing and logging")
        .unwrap();
}
