use std::collections::BinaryHeap;

use rand::{rngs::StdRng, SeedableRng};

const U: usize = 0;
const R: usize = 1;
const D: usize = 2;
const L: usize = 3;

const X: [usize; 4] = [0, 1, 0, !0];
const Y: [usize; 4] = [!0, 0, 1, 0];

fn main() {
    let mut rng = StdRng::seed_from_u64(71);
    let mut hash = vec![0; 10000];
    for i in 0..10000 {}

    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let generation: i32 = sc.read();
    let n: usize = sc.read();

    let mut paths = vec![vec![vec![false; 4]; n]; n];

    for i in 0..n {
        for j in 1..n {
            let v: usize = sc.read();
            if v == 0 {
                paths[i][j - 1][R] = true;
                paths[i][j][L] = true;
            }
        }
    }

    for i in 1..n {
        for j in 0..n {
            let h: usize = sc.read();
            if h == 0 {
                paths[i - 1][j][D] = true;
                paths[i][j][U] = true;
            }
        }
    }

    let mut board = vec![0; n * n];
    for i in 0..n {
        for j in 0..n {
            let a: i64 = sc.read();
            board[i * n + j] = a;
        }
    }

    let mut score = 0;
    for i in 0..n {
        for j in 0..n {
            if i + 1 < n {
                let a1 = board[i * n + j];
                let a2 = board[(i + 1) * n + j];
                let da = a1 - a2;
                score += da * da;
            }

            if j + 1 < n {
                let a1 = board[i * n + j];
                let a2 = board[i * n + j + 1];
                let da = a1 - a2;
                score += da * da;
            }
        }
    }

    let mut heap = BinaryHeap::new();
}

#[derive(Clone)]
struct State {
    board: Vec<Vec<i64>>,
    score: i64,
    pos1: (usize, usize),
    pos2: (usize, usize),
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> Self {
        Self(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: ToString>(&mut self, s: S) {
        use std::io::Write;
        self.1.write_all(s.to_string().as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn usize0(&mut self) -> usize {
        self.read::<usize>() - 1
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}
