mod cat_and_mouse;

fn main() {
    // println!("{}", cat_and_mouse::Solution::cat_mouse_game(vec![vec![1,3], vec![0], vec![3], vec![0,2]]));
    // println!("{}", cat_and_mouse::Solution::cat_mouse_game(vec![vec![2,5], vec![3], vec![0,4,5], vec![1,4,5], vec![2,3], vec![0,2,3]]));
    // [[2,3],[3,4],[0,4],[0,1],[1,2]]
    // println!("{}", cat_and_mouse::Solution::cat_mouse_game(vec![vec![2,3], vec![3,4], vec![0,4], vec![0,1], vec![1,2]]));
    // [[2,3],[2],[0,1],[0,4],[3]]
    // println!("{}", cat_and_mouse::Solution::cat_mouse_game(vec![vec![2,3], vec![2], vec![0,1], vec![0,4], vec![3]]));
    // [[6],[4,11],[9,12],[5],[1,5,11],[3,4,6],[0,5,10],[8,9,10],[7],[2,7,12],[6,7],[1,4],[2,9]]
    println!("{}", cat_and_mouse::Solution::cat_mouse_game(vec![vec![6], vec![4,11], vec![9,12], vec![5], vec![1,5,11], vec![3,4,6], vec![0,5,10], vec![8,9,10], vec![7], vec![2,7,12], vec![6,7], vec![1,4], vec![2,9]]));
}
