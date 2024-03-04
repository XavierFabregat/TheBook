fn main() {
    _also_make_it_panic();
}
fn _make_it_panic() {
    panic!("crash and burn");
}

fn _also_make_it_panic() {
    let v = vec![1, 2, 3];
    v[99];
}
