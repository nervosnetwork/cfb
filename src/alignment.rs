/// Finds next aligned position starting from `current_position` which is multiple of `alignment`.
///
/// This method assumes that `alignment` is a power of 2.
///
/// ## Examples
///
/// ```
/// use cfb::alignment::align;
/// # let current_position = 0;
/// # let alignment = 16;
/// let result = align(current_position, alignment);
/// assert_eq!(result % alignment, 0);
/// assert!(result >= current_position);
/// assert!(result - current_position < alignment);
/// ```
#[inline(always)]
pub fn align(current_position: usize, alignment: usize) -> usize {
    debug_assert_eq!(alignment.count_ones(), 1);
    // current_position + (((!current_position) + 1) & (alignment - 1))
    current_position + ((!current_position).wrapping_add(1) & (alignment.wrapping_sub(1)))
}

/// Finds next aligned position starting from `current_position` such that:
///
/// After appending the data of length `len`, the next position is aligned to multiple of
/// `alignment`.
///
/// If `len` is larger than `alignment`, `len` will be used as the alignment to find the position.
///
/// This method assumed that `len` and `alignment` are powers of 2.
///
/// ## Examples
///
/// ```
/// use cfb::alignment::align_after;
/// # let current_position = 0;
/// # let len = 8;
/// # let alignment = 16;
/// let result = align_after(current_position, len, alignment);
/// assert_eq!((result + len) % alignment, 0);
/// assert!(result >= current_position);
/// assert!(result - current_position < alignment);
/// ```
pub fn align_after(current_position: usize, len: usize, alignment: usize) -> usize {
    let real_alignment = alignment.max(len);
    let first_available_position = align(current_position, real_alignment);

    if first_available_position - current_position >= len {
        first_available_position - len
    } else {
        first_available_position + real_alignment - len
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_align() {
        assert_eq!(0, align(0, 16));
        assert_eq!(16, align(1, 16));
        assert_eq!(16, align(15, 16));
        assert_eq!(16, align(16, 16));
        assert_eq!(32, align(17, 16));
    }

    #[test]
    fn test_align_after() {
        assert_eq!(0, align_after(0, 0, 32));
        assert_eq!(24, align_after(0, 8, 32));
        assert_eq!(24, align_after(24, 8, 32));
        assert_eq!(56, align_after(25, 8, 32));
    }

    proptest! {
        #[test]
        fn align_proptest(
            current_position: usize,
            alignment in (0u32..16).prop_map(|d| 2usize.pow(d)),
        ) {
            prop_assume!(
                current_position.checked_add(alignment).is_some(),
                "avoid integer overflow"
            );
            let result = align(current_position, alignment);
            assert!(result >= current_position);
            assert!(result - current_position < alignment);
            assert_eq!(0, result % alignment);
        }

        #[test]
        fn align_after_proptest(
            current_position: usize,
            len in (0u32..16).prop_map(|d| 2usize.pow(d)),
            alignment in (0u32..16).prop_map(|d| 2usize.pow(d)),
        ) {
            prop_assume!(
                current_position
                    .checked_add(len)
                    .and_then(|a| a.checked_add(alignment))
                    .is_some(),
                "avoid integer overflow"
            );
            let result = align_after(current_position, len, alignment);
            assert!(result >= current_position);
            assert!(result - current_position < alignment.max(len));
            assert_eq!(0, (result + len) % alignment.max(len));
        }
    }
}
