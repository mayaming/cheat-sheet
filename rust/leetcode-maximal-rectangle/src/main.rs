mod maximal_rectangle;

fn main() {
    let max_size = maximal_rectangle::Solution::maximal_rectangle(vec![
        vec!['1', '0', '1'], 
        vec!['0', '0', '0']
    ]
    );
    println!("{}", max_size);
    let max_size = maximal_rectangle::Solution::maximal_rectangle(vec![
        vec!['1', '0', '1', '0', '0'], 
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '1', '1', '1', '1'], 
        vec!['1', '0', '0', '1', '0']
    ]
    );
    println!("{}", max_size);

    let max_size = maximal_rectangle::Solution::maximal_rectangle(vec![
        vec!['1']
    ]
    );
    println!("{}", max_size);

    let max_size = maximal_rectangle::Solution::maximal_rectangle(vec![
        vec!['0']
    ]
    );
    println!("{}", max_size);

    let mut v: Vec<Vec<char>> = Vec::new();
    for _ in 0..150 {
        v.push(vec!['1', '1', '1', '1', '1']);
    }
    let max_size = maximal_rectangle::Solution::maximal_rectangle(v);
    println!("{}", max_size);
}

