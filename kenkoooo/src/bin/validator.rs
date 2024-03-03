use std::{fs::File, io::Read};

const U: usize = 0;
const R: usize = 1;
const D: usize = 2;
const L: usize = 3;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut input = String::new();
    File::open(&args[1])
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let mut writer = vec![];
    let mut sc = IO::new(input.as_bytes(), &mut writer);

    let _: i32 = sc.read();
    let n: usize = sc.read();

    let mut paths = vec![vec![vec![false; 4]; n]; n];

    for i in 0..n {
        let s = sc.chars();
        for j in 1..n {
            if s[j - 1] == '0' {
                paths[i][j - 1][R] = true;
                paths[i][j][L] = true;
            }
        }
    }

    for i in 1..n {
        let s = sc.chars();
        for j in 0..n {
            if s[j] == '0' {
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

    let mut solution = String::new();
    File::open(&args[2])
        .unwrap()
        .read_to_string(&mut solution)
        .unwrap();
    let mut writer = vec![];
    let mut sc = IO::new(solution.as_bytes(), &mut writer);

    let mut i1: usize = sc.read();
    let mut j1: usize = sc.read();
    let mut i2: usize = sc.read();
    let mut j2: usize = sc.read();
    for turn in 0..(4 * n * n) {
        let swap: usize = sc.read();
        let move1: String = sc.read();
        let move2: String = sc.read();

        if swap == 1 {
            let a = board[i1 * n + j1];
            let b = board[i2 * n + j2];
            board[i1 * n + j1] = b;
            board[i2 * n + j2] = a;
        }
        match move1.as_str() {
            "U" => {
                assert!(paths[i1][j1][U], "turn={turn} i1={i1} j1={j1}");
                i1 -= 1;
            }
            "R" => {
                assert!(paths[i1][j1][R], "turn={turn} i1={i1} j1={j1}");
                j1 += 1;
            }
            "D" => {
                assert!(paths[i1][j1][D], "turn={turn} i1={i1} j1={j1}");
                i1 += 1;
            }
            "L" => {
                assert!(paths[i1][j1][L], "turn={turn} i1={i1} j1={j1}");
                j1 -= 1;
            }
            "." => {}
            _ => panic!("invalid move1 in {turn}"),
        }
        match move2.as_str() {
            "U" => {
                assert!(paths[i2][j2][U], "turn={turn} i2={i2} j2={j2}");
                i2 -= 1;
            }
            "R" => {
                assert!(paths[i2][j2][R], "turn={turn} i2={i2} j2={j2}");
                j2 += 1;
            }
            "D" => {
                assert!(paths[i2][j2][D], "turn={turn} i2={i2} j2={j2}");
                i2 += 1;
            }
            "L" => {
                assert!(paths[i2][j2][L], "turn={turn} i2={i2} j2={j2}");
                j2 -= 1;
            }
            "." => {}
            _ => panic!("invalid move2 in {turn}"),
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
    eprintln!("score: {}", score);
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
