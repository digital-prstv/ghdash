#[test]
fn without_zipkin_tests() {
    trycmd::TestCases::new()
        // .case("README.md")
        .case("tests/without_zipkin/*.trycmd");
}
