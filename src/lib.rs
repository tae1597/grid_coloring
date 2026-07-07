pub mod greedy;
pub mod backtracking;

#[derive(Clone)]
pub struct Grid {
    pub n: usize,
    pub m: usize,
    pub raw_buffer: Vec<u8>,
}

impl Grid {
    #[inline(always)]
    pub fn get(&self, r: usize, c: usize) -> u8 {
        self.raw_buffer[r * self.m + c]
    }

    #[inline(always)]
    pub fn set(&mut self, r: usize, c: usize, val: u8) {
        self.raw_buffer[r * self.m + c] = val;
    }
}

#[inline(always)]
pub fn parse_int(bytes: &[u8], cursor: &mut usize) -> usize {
    while *cursor < bytes.len() && bytes[*cursor] <= b' ' {
        *cursor += 1;
    }
    let mut val = 0;
    while *cursor < bytes.len() && bytes[*cursor] >= b'0' && bytes[*cursor] <= b'9' {
        val = val * 10 + (bytes[*cursor] - b'0') as usize;
        *cursor += 1;
    }
    val
}

pub fn parse_input(bytes: &[u8]) -> Grid {
    let mut cursor = 0;
    let n = parse_int(bytes, &mut cursor);
    let m = parse_int(bytes, &mut cursor);
    
    let mut raw_buffer = Vec::with_capacity(n * m);
    while cursor < bytes.len() && raw_buffer.len() < n * m {
        let b = bytes[cursor];
        cursor += 1;
        if b >= b'A' && b <= b'D' {
            raw_buffer.push(b);
        }
    }
    
    Grid { n, m, raw_buffer }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::greedy::solve_greedy;
    use crate::backtracking::solve_backtracking;

    fn check_valid(grid: &Grid, out: &Grid) {
        assert_eq!(grid.n, out.n);
        assert_eq!(grid.m, out.m);
        for r in 0..grid.n {
            for c in 0..grid.m {
                let orig = grid.get(r, c);
                let new_val = out.get(r, c);
                assert_ne!(orig, new_val, "Cell ({}, {}) shares same color as original: {}", r, c, new_val as char);
                if r > 0 {
                    assert_ne!(new_val, out.get(r - 1, c), "Cell ({}, {}) conflicts with top neighbor: {}", r, c, new_val as char);
                }
                if c > 0 {
                    assert_ne!(new_val, out.get(r, c - 1), "Cell ({}, {}) conflicts with left neighbor: {}", r, c, new_val as char);
                }
            }
        }
    }

    #[test]
    fn test_parse() {
        let input = b"3 3\nABC\nBCD\nCDA";
        let grid = parse_input(input);
        assert_eq!(grid.n, 3);
        assert_eq!(grid.m, 3);
        assert_eq!(grid.raw_buffer, vec![b'A', b'B', b'C', b'B', b'C', b'D', b'C', b'D', b'A']);
    }

    #[test]
    fn test_algorithms_basic() {
        let input = b"3 3\nAAA\nAAA\nAAA";
        let grid = parse_input(input);
        
        let out_greedy = solve_greedy(&grid).expect("Greedy solved basic grid");
        check_valid(&grid, &out_greedy);

        let out_backtrack = solve_backtracking(&grid).expect("Backtracking solved basic grid");
        check_valid(&grid, &out_backtrack);
    }

    #[test]
    fn test_algorithms_random() {
        // Linear congruential generator for reproducible tests
        let mut seed = 0u64;
        let mut rand = || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            seed
        };

        let chars = [b'A', b'B', b'C', b'D'];

        for _ in 0..100 {
            let n = (rand() % 15 + 1) as usize;
            let m = (rand() % 15 + 1) as usize;
            let mut raw_buffer = Vec::with_capacity(n * m);
            for _ in 0..(n * m) {
                raw_buffer.push(chars[(rand() % 4) as usize]);
            }
            let grid = Grid { n, m, raw_buffer };

            let out_greedy = solve_greedy(&grid).expect("Greedy solved random grid");
            check_valid(&grid, &out_greedy);

            let out_backtrack = solve_backtracking(&grid).expect("Backtracking solved random grid");
            check_valid(&grid, &out_backtrack);
        }
    }
}
