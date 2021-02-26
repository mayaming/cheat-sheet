mod nim_game;

fn main() {
    println!("{}", nim_game::Solution::can_win_nim(4));
}
