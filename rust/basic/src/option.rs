fn fake() -> i32 {
    -1
}

pub fn option_demo() {
    println!("{:?}", Some(fake()));
}