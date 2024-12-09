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

pub mod space {
    use std::ops::{Neg, Sub};

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Coord {
        inner: usize,
    }

    impl Coord {
        pub fn new(inner: usize) -> Self {
            Self { inner }
        }

        pub fn into_inner(self) -> usize {
            let Self { inner } = self;
            inner
        }

        #[track_caller]
        pub fn rel_offset(self, offset: RelativeOffset) -> Option<Self> {
            let Self { inner } = self;
            let RelativeOffset { value, sign } = offset;
            let Offset { inner: value } = value;
            (match sign {
                PosNeg::Positive => Some(inner.checked_add(value).unwrap()),
                PosNeg::Negative => inner.checked_sub(value),
            })
            .map(|inner| Self { inner })
        }
    }

    impl Sub for Coord {
        type Output = RelativeOffset;

        fn sub(self, rhs: Self) -> Self::Output {
            let Self { inner: lhs } = self;
            let Self { inner: rhs } = rhs;
            RelativeOffset {
                value: Offset::new(lhs.abs_diff(rhs)),
                sign: lhs
                    .checked_sub(rhs)
                    .map(|_| PosNeg::Negative)
                    .unwrap_or(PosNeg::Positive),
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct Offset {
        inner: usize,
    }

    impl Offset {
        pub fn new(inner: usize) -> Self {
            Self { inner }
        }

        pub fn checked_mul(self, rhs: usize) -> Option<Self> {
            let Self { inner } = self;
            let inner = inner.checked_mul(rhs)?;
            Some(Self { inner })
        }
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub enum PosNeg {
        Positive,
        Negative,
    }

    impl PosNeg {
        pub fn invert(self) -> Self {
            match self {
                PosNeg::Positive => PosNeg::Negative,
                PosNeg::Negative => PosNeg::Positive,
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct RelativeOffset {
        pub value: Offset,
        pub sign: PosNeg,
    }

    impl Neg for RelativeOffset {
        type Output = Self;

        fn neg(self) -> Self::Output {
            let Self { value, sign } = self;

            Self {
                value,
                sign: sign.invert(),
            }
        }
    }

    impl RelativeOffset {
        pub fn checked_mul(self, rhs: usize) -> Option<Self> {
            let Self { value, sign } = self;
            let value = value.checked_mul(rhs)?;
            Some(Self { value, sign })
        }
    }

    pub mod d2 {
        use std::ops::{Neg, Sub};

        use super::Coord;

        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub struct Size {
            pub row: Coord,
            pub col: Coord,
        }

        impl Size {
            pub fn from_row_major(coords: (usize, usize)) -> Self {
                let (row, col) = coords;
                let [row, col] = [row, col].map(Coord::new);
                Self { row, col }
            }
        }

        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub struct RelativeOffset {
            pub row: super::RelativeOffset,
            pub col: super::RelativeOffset,
        }

        impl RelativeOffset {
            pub fn checked_mul(self, rhs: usize) -> Option<Self> {
                let Self { row, col } = self;
                let row = row.checked_mul(rhs)?;
                let col = col.checked_mul(rhs)?;
                Some(Self { row, col })
            }
        }

        impl Neg for RelativeOffset {
            type Output = Self;

            fn neg(self) -> Self::Output {
                let Self { row, col } = self;

                Self {
                    row: -row,
                    col: -col,
                }
            }
        }

        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
        pub struct Coords {
            pub row: Coord,
            pub col: Coord,
        }

        impl Coords {
            pub fn from_row_major(coords: (usize, usize)) -> Self {
                let (row, col) = coords;
                let [row, col] = [row, col].map(Coord::new);
                Self { row, col }
            }
        }

        impl Sub for Coords {
            type Output = RelativeOffset;

            fn sub(self, rhs: Self) -> Self::Output {
                let Self {
                    row: lhs_row,
                    col: lhs_col,
                } = self;
                let Self {
                    row: rhs_row,
                    col: rhs_col,
                } = rhs;

                let [row, col] =
                    [(lhs_row, rhs_row), (lhs_col, rhs_col)].map(|(lhs, rhs)| lhs - rhs);

                RelativeOffset { row, col }
            }
        }

        pub fn apply_rel_offset(
            bounds: Size,
            origin: Coords,
            offset: RelativeOffset,
        ) -> Option<Coords> {
            let RelativeOffset { row, col } = offset;
            let Coords {
                row: origin_row,
                col: origin_col,
            } = origin;
            let Size {
                row: bounds_row,
                col: bounds_col,
            } = bounds;

            let row = origin_row.rel_offset(row).filter(|o| *o < bounds_row)?;
            let col = origin_col.rel_offset(col).filter(|o| *o < bounds_col)?;
            Some(Coords { row, col })
        }
    }
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

            let convert = |value, sign| {
                let (value, sign) = match sign {
                    Sign::Positive => (value, crate::space::PosNeg::Positive),
                    Sign::Neutral => (0, crate::space::PosNeg::Positive),
                    Sign::Negative => (value, crate::space::PosNeg::Negative),
                };
                let value = crate::space::Offset::new(value);
                crate::space::RelativeOffset { value, sign }
            };

            let bounds = crate::space::d2::Size::from_row_major(bounds);
            let origin = crate::space::d2::Coords::from_row_major(origin);
            let offset = crate::space::d2::RelativeOffset {
                row: convert(offset, vertical),
                col: convert(offset, horizontal),
            };

            let coords = crate::space::d2::apply_rel_offset(bounds, origin, offset)?;
            let crate::space::d2::Coords { row, col } = coords;
            let [row, col] = [row, col].map(|coord| coord.into_inner());
            Some((row, col))
        }
    }
}
