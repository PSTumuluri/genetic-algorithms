use super::*;

pub fn range_saturating_add(lhs: i64, rhs: i64, range: &Range<i64>) -> i64 {
    let sum = lhs.saturating_add(rhs);
    if sum >= range.end {
        range.end
    } else if sum < range.start {
        range.start
    } else {
        sum
    }
}
