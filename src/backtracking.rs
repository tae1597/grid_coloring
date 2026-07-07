use crate::Grid;

/// Algorithm B: Depth-First Search with Backtracking
///
/// This algorithm implements a recursive constraint satisfaction solver.
/// It assigns colors in row-major order, checking local constraints
/// (avoiding the original cell color, the top neighbor's color, and the left neighbor's color).
/// If a collision occurs, it backtracks.
///
/// Even though a solution is mathematically guaranteed to be found without backtracking
/// (since max forbidden colors is 3 and available colors is 4), this recursive structure
/// illustrates a generic DFS solver which contrasts with the single-pass greedy scan.
pub fn solve_backtracking(grid: &Grid) -> Option<Grid> {
    let n = grid.n;
    let m = grid.m;
    let mut out_buffer = vec![0u8; n * m];

    if backtrack(0, n, m, &grid.raw_buffer, &mut out_buffer) {
        Some(Grid { n, m, raw_buffer: out_buffer })
    } else {
        None
    }
}

fn backtrack(
    idx: usize,
    n: usize,
    m: usize,
    grid_buffer: &[u8],
    out_buffer: &mut [u8],
) -> bool {
    // Base Case: all cells colored
    if idx == n * m {
        return true;
    }

    let r = idx / m;
    let c = idx % m;

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

    const CHARS: [u8; 4] = [b'A', b'B', b'C', b'D'];
    const CHAR_BITS: [u32; 4] = [1, 2, 4, 8];

    // Try each of the 4 colors
    for i in 0..4 {
        if (forbidden_mask & CHAR_BITS[i]) == 0 {
            unsafe {
                *out_buffer.get_unchecked_mut(idx) = CHARS[i];
            }
            if backtrack(idx + 1, n, m, grid_buffer, out_buffer) {
                return true;
            }
        }
    }

    false
}
