#![allow(unused_imports)]
use std::io;
use std::cmp;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::f64::consts::PI;
use std::str::FromStr;
use std::cmp::Reverse;

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

fn main() {
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
    let mut a: Vec<Vec<usize>> = Vec::new();
    for i in 0..n {
        let row: Vec<usize> = sc.next_vec();
        a.push(row);
    }

    // 初期位置決め
    let p = (0, 0);
    let q = (0, 0);

    // 出力
    println!("{} {} {} {}", p.0, p.1, q.0, q.1);
}
