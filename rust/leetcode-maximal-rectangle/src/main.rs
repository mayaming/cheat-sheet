mod maximal_rectangle;

fn main() {
    let max_size = maximal_rectangle::Solution::maximal_rectangle(vec![
        vec!['1', '0', '1'], 
        vec!['0', '0', '0']
    ]
    );
    println!("{}", max_size);
}

