use rand::{rngs::StdRng, SeedableRng};

const U: usize = 0;
const R: usize = 1;
const D: usize = 2;
const L: usize = 3;
const STAY: usize = 4;

const I: [usize; 5] = [!0, 0, 1, 0, 0];
const J: [usize; 5] = [0, 1, 0, !0, 0];

const BEAM: usize = 1;

fn main() {
    let _ = StdRng::seed_from_u64(71);

    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut sc = IO::new(r.lock(), w.lock());

    let _: i32 = sc.read();
    let n: usize = sc.read();

    let mut paths = vec![vec![vec![false; 4]; n]; n];

    for i in 0..n {
        let v = sc.chars();
        assert_eq!(v.len(), n - 1);
        for j in 1..n {
            if v[j - 1] == '0' {
                paths[i][j - 1][R] = true;
                paths[i][j][L] = true;
            }
        }
    }

    for i in 1..n {
        let h = sc.chars();
        assert_eq!(h.len(), n);
        for j in 0..n {
            if h[j] == '0' {
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

    let mut states = vec![];
    states.push(State {
        board,
        score,
        pos1: (0, 0),
        pos2: (n - 1, n - 1),
        n,
        log: vec![],
    });

    let mut next = vec![];

    let start = std::time::Instant::now();
    for _turn in 0..(4 * n * n) {
        if start.elapsed().as_millis() > 1800 {
            break;
        }
        for state in states {
            let (i1, j1) = state.pos1;
            let (i2, j2) = state.pos2;

            for move1 in 0..5 {
                if move1 != STAY && !paths[i1][j1][move1] {
                    continue;
                }

                let ni1 = i1.wrapping_add(I[move1]);
                let nj1 = j1.wrapping_add(J[move1]);
                if ni1 >= n || nj1 >= n {
                    continue;
                }

                for move2 in 0..5 {
                    if move2 != STAY && !paths[i2][j2][move2] {
                        continue;
                    }

                    let ni2 = i2.wrapping_add(I[move2]);
                    let nj2 = j2.wrapping_add(J[move2]);
                    if ni2 >= n || nj2 >= n {
                        continue;
                    }

                    if move1 == STAY && move2 == STAY {
                        continue;
                    }

                    for swap in 0..2 {
                        let mut state = state.clone();
                        if swap == 1 {
                            state.swap();
                        }
                        state.log.push((swap, move1, move2));
                        state.pos1 = (ni1, nj1);
                        state.pos2 = (ni2, nj2);

                        next.push(state);
                    }
                }
            }
        }

        next.sort_unstable_by_key(|state| state.score);
        next.truncate(BEAM);
        states = vec![];
        (states, next) = (next, states);

        assert!(states.len() > 0);
    }

    let state = states.into_iter().min_by_key(|s| s.score).unwrap();
    eprintln!("score={} hands={}", state.score, state.log.len());
    sc.write(format!("0 0 {} {}\n", n - 1, n - 1));

    for i in 0..(4 * n * n) {
        if i >= state.log.len() {
            sc.write("0 . .\n");
            continue;
        }

        let (swap, move1, move2) = state.log[i];
        let move1 = match move1 {
            U => 'U',
            R => 'R',
            D => 'D',
            L => 'L',
            STAY => '.',
            _ => unreachable!(),
        };
        let move2 = match move2 {
            U => 'U',
            R => 'R',
            D => 'D',
            L => 'L',
            STAY => '.',
            _ => unreachable!(),
        };

        sc.write(swap);
        sc.write(' ');
        sc.write(move1);
        sc.write(' ');
        sc.write(move2);
        sc.write('\n');
    }
}

impl State {
    fn swap(&mut self) {
        let (i1, j1) = self.pos1;
        let (i2, j2) = self.pos2;
        let a1 = self.board[i1 * self.n + j1];
        let a2 = self.board[i2 * self.n + j2];

        self.pull(i1, j1);
        self.push(i1, j1, a2);

        self.pull(i2, j2);
        self.push(i2, j2, a1);
    }

    fn pull(&mut self, i: usize, j: usize) {
        let mut remove = 0;
        let a = self.board[i * self.n + j];
        for d in 0..4 {
            let ni = i.wrapping_add(I[d]);
            let nj = j.wrapping_add(J[d]);
            if ni >= self.n || nj >= self.n {
                continue;
            }

            let b = self.board[ni * self.n + nj];
            let da = a - b;
            remove += da * da;
        }

        self.score -= remove;
        self.board[i * self.n + j] = 0;
    }

    fn push(&mut self, i: usize, j: usize, a: i64) {
        let mut add = 0;
        self.board[i * self.n + j] = a;
        for d in 0..4 {
            let ni = i.wrapping_add(I[d]);
            let nj = j.wrapping_add(J[d]);
            if ni >= self.n || nj >= self.n {
                continue;
            }

            let b = self.board[ni * self.n + nj];
            let da = a - b;
            add += da * da;
        }

        self.score += add;
    }
}

#[derive(Clone, Debug)]
struct State {
    board: Vec<i64>,
    score: i64,
    pos1: (usize, usize),
    pos2: (usize, usize),
    n: usize,
    log: Vec<(usize, usize, usize)>,
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
