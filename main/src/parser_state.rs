// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Copied from pest/pest/src/parser_state.rs (commit ac0aed3eecf435fd93ba575a39704aaa88a375b7)
//! and modified.

use core::ops::Range;

pub(crate) fn constrain_idxs(start: i32, end: Option<i32>, len: usize) -> Option<Range<usize>> {
    let start_norm = normalize_index(start, len)?;
    let end_norm = end.map_or(Some(len), |e| normalize_index(e, len))?;
    Some(start_norm..end_norm)
}

/// Normalizes the index using its sequence’s length.
/// Returns `None` if the normalized index is OOB.
const fn normalize_index(i: i32, len: usize) -> Option<usize> {
    if i > len as i32 {
        None
    } else if i >= 0 {
        Some(i as usize)
    } else {
        let real_i = len as i32 + i;
        if real_i >= 0 {
            Some(real_i as usize)
        } else {
            None
        }
    }
}
