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
