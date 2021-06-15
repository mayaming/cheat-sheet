use std::collections::HashMap;

// https://leetcode.com/problems/cat-and-mouse/

pub struct Solution {} 

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    mouse_pos: i32,
    cat_pos: i32
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "mouse={},cat={}", self.mouse_pos, self.cat_pos)
    }
}

impl State {
    fn calc_result(&self, result_map: &mut HashMap<State, i32>, graph: &Vec<Vec<i32>>) -> i32 {
        println!("check for {} {}", self.mouse_pos, self.cat_pos);
        let mut result_next = -1;  // 额外增加一个状态-1，表示未确定
        
        match result_map.get(self) {
            Some(-1) => { result_map.insert(self.clone(), 0); return 0; }
            Some(n) => { return *n; }
            None => { result_map.insert(self.clone(), -1); }  // 曾经访问过当前节点，用于发现平局的情况 
        }

        if self.mouse_pos == 0 {
            result_next = 1;
        }
        else if self.cat_pos == self.mouse_pos {
            result_next = 2;
        }
        else {
            let mouse_nexts = &graph[self.mouse_pos as usize];
            let cat_nexts = &graph[self.cat_pos as usize];
            
            for mouse_next in mouse_nexts {
                let mouse_next = *mouse_next;
                let mut mouse_result_next = -1;
                if mouse_next == self.cat_pos {
                    mouse_result_next = 2;
                }
                else {
                    for cat_next in cat_nexts {
                        let cat_next = *cat_next;
                        if cat_next != 0 {
                            let state_next = State{mouse_pos: mouse_next, cat_pos: cat_next};
                            let n = result_map.get(&state_next).map(|r| *r).unwrap_or_else(|| state_next.calc_result(result_map, graph));
                            match n {
                                2 => { mouse_result_next = 2; break; }
                                0 => { mouse_result_next = 0; }
                                1 if mouse_result_next == -1 || mouse_result_next == 1 => { mouse_result_next = 1; }
                                _ => { mouse_result_next = 0 }
                            }

                            println!("    stat next = {} {}, mouse result next = {}", state_next.mouse_pos, state_next.cat_pos, mouse_result_next);
                        }
                    }
                }
                match mouse_result_next {
                    1 => { result_next = 1; break; }
                    0 => { result_next = 0; }
                    2 if result_next == -1 => { result_next = 2; }
                    _ => {}
                }
                // println!("  stat next = {} {}, result next = {}", self.mouse_pos, self.cat_pos, result_next);
            }
        }

        result_map.insert(self.clone(), result_next);
        println!("state: {}, result: {}", self, result_next);
        result_next
    }
}

impl Solution {
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let initial_stat = State{mouse_pos: 1, cat_pos: 2};
        let mut result_map = HashMap::new();
        initial_stat.calc_result(&mut result_map, &graph)
    }
}