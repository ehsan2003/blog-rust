use with_deps_proc_macro::WithDeps;

#[derive(WithDeps)]
pub struct Test {
    pub a: u32,
    pub b: u32,
}

fn main() {
    let mut t = Test::new(1, 2);
    t.set_b(3);
    assert_eq!(t.a, 1);
    assert_eq!(t.b, 3);
}
