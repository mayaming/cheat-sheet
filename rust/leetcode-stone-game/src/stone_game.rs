pub struct Solution{} 

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let len = piles.len();
        let mut scores = vec![vec![(0, 0); len]; len];
        for step in 0..len {
            for i in 0..(len-step) {
                let j = i + step;
                if step == 0 {
                    scores[i][j] = (piles[i], 0)
                }
                else {
                    let choose_left = (piles[i] + scores[i+1][j].1, scores[i+1][j].0);
                    let choose_right = (piles[j] + scores[i][j-1].1, scores[i][j-1].0);
                    scores[i][j] = if choose_left.0 > choose_right.0 { choose_left } else { choose_right }
                }
            }
        }
        return scores[0][len-1].0 > scores[0][len-1].1
    }
}