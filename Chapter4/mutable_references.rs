fn main() {
    let mut s1 = String::from("Hello");

    change(&mut s1);

    println!("{}", s1);

    let x = 1;

    {
        let y = &mut s1;
        y.push_str(", world!");
    };

    let z = &mut s1;

    println!("{} {}", x, z);
}

fn change(s: &mut String) {
    s.push_str(", world!");
}
