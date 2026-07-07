use std::io::{self, Read, Write};

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

pub fn solve_greedy(grid: &Grid) -> Option<Grid> {
    let n = grid.n;
    let m = grid.m;
    let mut out_buffer = vec![0u8; n * m];
    let grid_buffer = &grid.raw_buffer;

    const CHARS: [u8; 4] = [b'A', b'B', b'C', b'D'];
    const CHAR_BITS: [u32; 4] = [1, 2, 4, 8];

    for r in 0..n {
        let row_offset = r * m;
        for c in 0..m {
            let idx = row_offset + c;

            // Unsafe lookup for hot loop speed
            let original_char = unsafe { *grid_buffer.get_unchecked(idx) };
            let mut forbidden_mask = 1 << (original_char - b'A');

            if r > 0 {
                let top_char = unsafe { *out_buffer.get_unchecked(idx - m) };
                forbidden_mask |= 1 << (top_char - b'A');
            }
            if c > 0 {
                let left_char = unsafe { *out_buffer.get_unchecked(idx - 1) };
                forbidden_mask |= 1 << (left_char - b'A');
            }

            let mut chosen = 0;
            for i in 0..4 {
                if (forbidden_mask & CHAR_BITS[i]) == 0 {
                    chosen = CHARS[i];
                    break;
                }
            }

            if chosen == 0 {
                return None;
            }
            unsafe {
                *out_buffer.get_unchecked_mut(idx) = chosen;
            }
        }
    }

    Some(Grid {
        n,
        m,
        raw_buffer: out_buffer,
    })
}

fn main() {
    let mut raw_buffer = Vec::new();
    io::stdin().read_to_end(&mut raw_buffer).unwrap();

    let grid = parse_input(&raw_buffer);

    if let Some(out_grid) = solve_greedy(&grid) {
        let mut out_bytes = Vec::with_capacity(out_grid.n * (out_grid.m + 1));

        for r in 0..out_grid.n {
            let start = r * out_grid.m;
            let end = start + out_grid.m;
            out_bytes.extend_from_slice(&out_grid.raw_buffer[start..end]);
            out_bytes.push(b'\n');
        }

        let stdout = io::stdout();
        let mut handle = io::BufWriter::new(stdout.lock());
        handle.write_all(&out_bytes).unwrap();
    } else {
        println!("IMPOSSIBLE");
    }
}
