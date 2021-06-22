use std::collections::HashMap;
use std::collections::HashSet;

// https://leetcode.com/problems/cat-and-mouse/

pub struct Solution {} 

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct State {
    mouse_pos: i32,
    cat_pos: i32
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.mouse_pos, self.cat_pos)
    }
}

struct Path {
    visited: HashSet<State>,
    seq: Vec<State>
}

impl Path {
    fn new(state: &State) -> Path {
        let mut path = Path{visited: HashSet::new(), seq: vec![]};
        path.add(state);
        path
    }

    fn add(&mut self, state: &State) -> bool {
        match self.visited.get(state) {
            Some(_) => { false }
            None => {
                self.visited.insert(state.clone());
                self.seq.push(state.clone());
                true
            }
        }
    }

    fn leave(&mut self) -> () {
        let state = self.seq.last().unwrap();
        self.visited.remove(state);
        self.seq.truncate(self.seq.len()-1);
    }

    fn can_go_back(&self, mouse_next_pos: i32) -> bool {
        self.seq.len() > 1 && self.seq[self.seq.len()-2].mouse_pos == mouse_next_pos
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();
        for state in &self.seq {
            s.push_str((format!("{} -> ", state)).as_str());
        }
        write!(f, "{}", s)
    }
}

impl State {
    fn calc_result(&self, result_map: &mut HashMap<State, i32>, graph: &Vec<Vec<i32>>, path: &mut Path, prepare: bool, depth: usize, debug: bool) -> i32 {
        if debug && !prepare {
            println!("{}check for {}, path = {}", "  ".repeat(depth), self, path);
        }
        
        let ret = match result_map.get(self) {
            Some(n) => { *n }
            None => {
                if self.mouse_pos == 0 {
                    result_map.insert(self.clone(), 1);
                }
                else if self.cat_pos == self.mouse_pos {
                    result_map.insert(self.clone(), 2);
                }
                else {
                    let mouse_nexts = &graph[self.mouse_pos as usize];
                    let cat_nexts = &graph[self.cat_pos as usize];
                    let mut result_next = -1;
                    
                    for mouse_next in mouse_nexts {
                        let mouse_next = *mouse_next;
                        let mut mouse_result_next = if path.can_go_back(mouse_next) { 0 } else { -1 };
                        if mouse_next == self.cat_pos {
                            mouse_result_next = 2;
                        }
                        else if mouse_next == 0 {
                            mouse_result_next = 1;
                        }
                        else {
                            for cat_next in cat_nexts {
                                let cat_next = *cat_next;
                                if cat_next != 0 {
                                    let state_next = State{mouse_pos: mouse_next, cat_pos: cat_next};
                                    if path.add(&state_next) {
                                        let n = result_map.get(&state_next).map(|r| *r).unwrap_or_else(|| state_next.calc_result(result_map, graph, path, prepare, depth+1, debug));
                                        path.leave();
                                        if debug && !prepare {
                                            println!("{}mouse goes {} cat goes {}, got {}, mouse_result_next = {}", "  ".repeat(depth), state_next.mouse_pos, state_next.cat_pos, n, mouse_result_next);
                                        }
                                        match n {
                                            2 => { mouse_result_next = 2; break; }
                                            0 => { mouse_result_next = 0; }
                                            1 if mouse_result_next == -1 || mouse_result_next == 1 => { mouse_result_next = 1; }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }

                        match mouse_result_next {
                            1 => { result_next = 1; break; }
                            0 => { result_next = 0; }
                            2 if result_next == -1 || result_next == 2 => { result_next = 2; }
                            _ => { result_next = 0; }
                        }

                    }

                    if debug && !prepare {
                        println!("{}result for {}: {}", "  ".repeat(depth), self, result_next);
                    }


                    if !prepare || result_next == 1 || result_next == 2 {
                        result_map.insert(self.clone(), result_next);
                    }
                }

                *result_map.get(self).unwrap_or(&0)
            }
        };

        ret
    }
}

impl Solution {
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let debug = false;
        let initial_stat = State{mouse_pos: 1, cat_pos: 2};
        let mut result_map = HashMap::new();
        let mut path = Path::new(&initial_stat);
        initial_stat.calc_result(&mut result_map, &graph, &mut path, true, 0, debug);
        if debug {
            println!("result map after preparation: {:?}", result_map);
        }
        initial_stat.calc_result(&mut result_map, &graph, &mut path, false, 0, debug)
    }
}