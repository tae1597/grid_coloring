use crate::Grid;

/// Algorithm A: Fast Greedy Linear Scan
///
/// This algorithm performs a single pass over the grid in row-major order.
/// For each cell, it computes a bitmask of forbidden colors based on:
/// 1. The original character of the cell (which it must change).
/// 2. The chosen character of the cell directly above (if inbounds).
/// 3. The chosen character of the cell directly to the left (if inbounds).
///
/// It then selects the first character in {'A', 'B', 'C', 'D'} that is not forbidden.
/// Since there are at most 3 forbidden characters and 4 choices, a valid coloring is
/// guaranteed to exist and is found in O(N * M) time and O(1) auxiliary space.
///
/// Safety: This function uses `get_unchecked` to bypass bounds checks for hot loops.
/// All indices are mathematically verified to be in bounds.
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
            
            // Retrieve original character safely/unchecked
            let original_char = unsafe { *grid_buffer.get_unchecked(idx) };
            
            // Initialize mask with original char
            let mut forbidden_mask = 1 << (original_char - b'A');

            // Add top neighbor if valid
            if r > 0 {
                let top_char = unsafe { *out_buffer.get_unchecked(idx - m) };
                forbidden_mask |= 1 << (top_char - b'A');
            }
            
            // Add left neighbor if valid
            if c > 0 {
                let left_char = unsafe { *out_buffer.get_unchecked(idx - 1) };
                forbidden_mask |= 1 << (left_char - b'A');
            }

            // Find first available color
            let mut chosen = 0;
            for i in 0..4 {
                if (forbidden_mask & CHAR_BITS[i]) == 0 {
                    chosen = CHARS[i];
                    break;
                }
            }

            if chosen == 0 {
                return None; // Theoretically unreachable for 4-coloring
            }
            
            // Write output safely/unchecked
            unsafe {
                *out_buffer.get_unchecked_mut(idx) = chosen;
            }
        }
    }

    Some(Grid { n, m, raw_buffer: out_buffer })
}
