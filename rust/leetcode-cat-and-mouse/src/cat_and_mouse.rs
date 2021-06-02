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
        let mut result_next = -1;  // 额外增加一个状态-1，表示未确定
        result_map.insert(self.clone(), result_next);  // 曾经访问过当前节点，用于发现平局的情况 
    
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
                for cat_next in cat_nexts {
                    let cat_next = *cat_next;
                    if cat_next != 0 {
                        let state_next = State{mouse_pos: mouse_next, cat_pos: cat_next};

                        if !result_map.contains_key(&state_next) {
                            self.calc_result(result_map, graph);
                        }

                        let n = *result_map.get(&state_next).unwrap();
                        if n == -1 || n == 0 {
                            if result_next == -1 || result_next == 2 {
                                result_next = 0;
                            }
                        }
                        else if n == 1 {
                            result_next = 1;
                        }
                        else if n == 2 {
                            result_next = 2;
                        }
                    }
                }
            }
        }

        result_map.insert(self.clone(), result_next);
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