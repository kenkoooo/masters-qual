#![allow(unused_imports)]
use std::{io, mem};
use std::cmp;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::f64::consts::PI;
use std::str::FromStr;
use std::cmp::Reverse;
use std::time::{Duration, Instant};
use rand::Rng;

#[allow(dead_code)]
type Error = Box<dyn std::error::Error>;

fn read_line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();
    buf
}

struct Scanner {
    buff: Vec<String>,
    pos: usize,
}

struct Solution {
    p: (usize, usize),
    q: (usize, usize),
    steps: Vec<(u8, char, char)>,
}

#[allow(dead_code)]
impl Scanner {
    pub fn new() -> Self {
        Scanner {
            buff: Vec::new(),
            pos: 0,
        }
    }

    pub fn next<T: FromStr>(&mut self) -> T {
        while self.buff.len() <= self.pos {
            self.pos = 0;
            self.buff = read_line().split_whitespace().map(|s| s.to_owned()).collect();
        }

        let result = self.buff[self.pos].parse::<T>();
        self.pos += 1;
        result.ok().unwrap()
    }

    pub fn next_line(&mut self) -> String {
        let result  = read_line().trim().to_owned();
        self.buff.clear();
        self.pos = 0;

        result
    }

    pub fn next_vec<T: FromStr>(&mut self) -> Vec<T> {
        self.buff.clear();
        self.pos = 0;
        read_line().trim().split_whitespace().map(|s| s.parse().ok().unwrap()).collect()
    }

    pub fn next_line_chars(&mut self) -> Vec<char> {
        let result  = read_line().trim().chars().collect();
        self.buff.clear();
        self.pos = 0;

        result
    }
}

fn calc_score(a: &Vec<Vec<i64>>) -> i64 {
    let n = a.len();
    let mut score = 0;
    for i in 0..n {
        for j in 0..n-1 {
            let d = a[i][j] - a[i][j+1];
            score += d*d;
        }
    }
    for i in 0..n-1 {
        for j in 0..n {
            let d = (a[i][j] - a[i+1][j]).abs();
            score += d*d;
        }
    }
    score
}

fn find_solution(n: usize, a: &Vec<Vec<i64>>, graph: &Vec<Vec<usize>>, now: &Instant) -> Solution {
    let n = a.len();
    let mut rnd = rand::thread_rng();
    let mut best_score = calc_score(a);
    let mut best_solution = Solution {
        p: (0, 0),
        q: (0, 0),
        steps: Vec::new(),
    };
    while now.elapsed() < Duration::from_millis(800) {
        let px = rnd.gen_range(0..n);
        let py = rnd.gen_range(0..n);
        let qx = rnd.gen_range(0..n);
        let qy = rnd.gen_range(0..n);
        let mut a = a.clone();
        let mut solution = Solution {
            p: (px, py),
            q: (qx, qy),
            steps: Vec::new(),
        };
        let mut ppos = (px, py);
        let mut qpos = (qx, qy);
        for i in 0..4*n*n {
            let should_swap = rnd.gen_bool(0.5);
            let pi = ppos.0*n+ppos.1;
            let qi = qpos.0*n+qpos.1;
            let pmove = rnd.gen_range(0..=graph[pi].len());
            let qmove = rnd.gen_range(0..=graph[qi].len());
            let pmovec = if pmove == graph[pi].len() {
                '.'
            } else {
                let npi = graph[pi][pmove];
                let np = (npi/n, npi%n);
                if np.0 < ppos.0 {
                    'U'
                } else if np.0 > ppos.0 {
                    'D'
                } else if np.1 < ppos.1 {
                    'L'
                } else {
                    'R'
                }
            };
            let qmovec = if qmove == graph[qi].len() {
                '.'
            } else {
                let nqi = graph[qi][qmove];
                let nq = (nqi/n, nqi%n);
                if nq.0 < qpos.0 {
                    'U'
                } else if nq.0 > qpos.0 {
                    'D'
                } else if nq.1 < qpos.1 {
                    'L'
                } else {
                    'R'
                }
            };
            if should_swap {
                let tmp = a[ppos.0][ppos.1];
                a[ppos.0][ppos.1] = a[qpos.0][qpos.1];
                a[qpos.0][qpos.1] = tmp;
            }
            if pmovec !='.' {
                let npi = graph[pi][pmove];
                ppos = (npi/n, npi%n);

            }
            if qmovec !='.' {
                let nqi = graph[qi][qmove];
                qpos = (nqi/n, nqi%n);
            }
            solution.steps.push((should_swap as u8, pmovec, qmovec));
        }
        let score = calc_score(&a);
        if score < best_score {
            best_score = score;
            best_solution = solution;
        }
    }
    best_solution
}

fn main() {
    let now = Instant::now();
    let mut sc = Scanner::new();
    let t: usize = sc.next();
    let n: usize = sc.next();
    let mut v: Vec<Vec<bool>> = vec![vec![true; n+2]; n+2];
    let mut h: Vec<Vec<bool>> = vec![vec![true; n+2]; n+2];
    for i in 0..n {
        let vs = sc.next_line_chars();
        for j in 0..n-1 {
            if vs[j] == '0' {
                v[i+1][j+1] = false;
            }
        }
    }
    for i in 0..n-1 {
        let hs = sc.next_line_chars();
        for j in 0..n {
            if hs[j] == '0' {
                h[i+1][j+1] = false;
            }
        }
    }
    let mut a: Vec<Vec<i64>> = Vec::new();
    for i in 0..n {
        let row: Vec<i64> = sc.next_vec();
        a.push(row);
    }
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n*n];
    for i in 0..n {
        for j in 0..n {
            if !v[i+1][j] {
                graph[i*n+j].push(i*n+j-1);
            }
            if !v[i+1][j+1] {
                graph[i*n+j].push(i*n+j+1);
            }
            if !h[i][j+1] {
                graph[i*n+j].push((i-1)*n+j);
            }
            if !h[i+1][j+1] {
                graph[i*n+j].push((i+1)*n+j);
            }
        }
    }

    let solution = find_solution(n, &a, &graph, &now);

    // 出力
    println!("{} {} {} {}", solution.p.0, solution.p.1, solution.q.0, solution.q.1);
    for (i, p, q) in solution.steps {
        println!("{} {} {}", i, p, q);
    }
}
