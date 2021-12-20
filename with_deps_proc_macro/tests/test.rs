use trybuild;

#[test]
pub fn test() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01.rs");
}
