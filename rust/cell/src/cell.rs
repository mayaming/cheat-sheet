use std::cell::Cell;

pub fn cell() {
    let data: Cell<i32> = Cell::new(100);
    let p = &data;
    data.set(10);
    println!("{}", p.get());
    p.set(20);
    println!("{:? }", data);
}