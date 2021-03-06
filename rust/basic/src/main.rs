mod option;

fn main() {
    println!("Hello, world!");
    
    let cities = ["beijing", "shanghai", "guangzhou"];
    println!("{} {} {}", cities[0], cities[1], cities[2]);  // beijing shanghai guangzhou
    println!("{:?}", cities);  // ["beijing", "shanghai", "guangzhou"]

    /*
    beijing
    shanghai
    guangzhou
    */
    for i in 0..cities.len() {
        println!("{}", cities[i]);
    }

    option::option_demo();
}
