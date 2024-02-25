#[test]
fn with_zipkin_tests() {
    trycmd::TestCases::new()
        // .case("README.md")
        .case("tests/with_zipkin/*.trycmd");
}
