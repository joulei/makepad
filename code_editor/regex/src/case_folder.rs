use crate::{CharClass, Range};

#[derive(Clone, Debug, Default)]
pub(crate) struct CaseFolder {
    stack: Vec<Range<u32>>,
}

impl CaseFolder {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn fold(&mut self, char_range: Range<char>, output: &mut CharClass) {
        use crate::unicode_tables;

        self.stack
            .push(Range::new(char_range.start as u32, char_range.end as u32));
        while let Some(mut range) = self.stack.pop() {
            if !output.insert(Range::new(
                char::from_u32(range.start).unwrap(),
                char::from_u32(range.end).unwrap(),
            )) {
                continue;
            }
            while range.start <= range.end {
                match unicode_tables::SIMPLE_CASE_FOLDING.binary_search_by(|(other_range, _)| {
                    use std::cmp::Ordering;

                    if (other_range.end as u32) < range.start {
                        return Ordering::Less;
                    }
                    if (other_range.start as u32) > range.start {
                        return Ordering::Greater;
                    }
                    Ordering::Equal
                }) {
                    Ok(index) => {
                        let (other_range, delta) = unicode_tables::SIMPLE_CASE_FOLDING[index];
                        self.stack.push(apply_delta(
                            Range::new(range.start, range.end.min(other_range.end as u32)),
                            delta,
                        ));
                        range.start = other_range.end as u32 + 1;
                    }
                    Err(index) => {
                        if index < unicode_tables::SIMPLE_CASE_FOLDING.len() {
                            let (other_range, _) = unicode_tables::SIMPLE_CASE_FOLDING[index];
                            range.start = other_range.start as u32;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn apply_delta(mut range: Range<u32>, delta: i32) -> Range<u32> {
    if delta == 1 {
        if range.start % 2 == 1 {
            range.start -= 1;
        }
        if range.end % 2 == 0 {
            range.end += 1;
        }
    } else if delta == -1 {
        if range.start % 2 == 0 {
            range.start -= 1;
        }
        if range.end % 2 == 1 {
            range.start += 1;
        }
    } else {
        range.start = (range.start as i32 + delta) as u32;
        range.end = (range.end as i32 + delta) as u32;
    }
    range
}
