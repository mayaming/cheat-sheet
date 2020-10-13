use std::cell::{Ref, RefCell};

fn vsum(v: Ref<Vec<i32>>) -> i32 {
    let mut ret = 0;
    for num in v.iter() {
        ret += num;
    }
    ret
}

fn main() {
    let v: RefCell<Vec<i32>> = RefCell::new(vec![]);
    for i in 0..10 {
        v.borrow_mut().push(i);
        println!("{}", vsum(v.borrow()))
    }
}
