pub fn uniform_width_ascii_lines<'a>(
    input: impl Iterator<Item = &'a str> + Clone + 'a,
) -> impl Iterator<Item = &'a str> + Clone + 'a {
    let mut first_line_width = None;
    input
        .inspect(move |line| match (first_line_width, line.len()) {
            (None, len) => first_line_width = Some(len),
            (Some(expected), actual) => {
                assert_eq!(expected, actual, "line does not match first line's length")
            }
        })
        .inspect(|line| assert!(line.is_ascii()))
}

pub mod search_direction {
    #[derive(Clone, Copy, Debug, Eq, PartialEq, strum::EnumIter)]
    pub enum Sign {
        Neutral,
        Negative,
        Positive,
    }

    #[derive(Clone)]
    pub struct SearchDirection {
        pub horizontal: Sign,
        pub vertical: Sign,
    }

    impl SearchDirection {
        pub fn to_2d_offsets(
            &self,
            origin: (usize, usize),
            bounds: (usize, usize),
            offset: usize,
        ) -> Option<(usize, usize)> {
            let &Self {
                horizontal,
                vertical,
            } = self;

            let (origin_row, origin_col) = origin;
            let (bounds_row, bounds_col) = bounds;

            let row_idx = match vertical {
                Sign::Neutral => Some(origin_row),
                Sign::Negative => origin_row.checked_sub(offset),
                Sign::Positive => origin_row.checked_add(offset).filter(|o| *o < bounds_row),
            };

            let col_idx = match horizontal {
                Sign::Neutral => Some(origin_col),
                Sign::Negative => origin_col.checked_sub(offset),
                Sign::Positive => origin_col.checked_add(offset).filter(|o| *o < bounds_col),
            };

            row_idx.zip(col_idx)
        }
    }
}
