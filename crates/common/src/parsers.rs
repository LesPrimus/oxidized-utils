use anyhow::anyhow;
use std::ops::Range;

pub type PositionList = Vec<Range<usize>>;

fn parse_part(s: &str, part: &str) -> anyhow::Result<usize> {
    let illegal = || anyhow!(r#"illegal list value: "{part}""#);
    if s.starts_with('+') {
        return Err(illegal());
    }
    s.parse::<usize>()
        .ok()
        .filter(|&n| n > 0)
        .ok_or_else(illegal)
}

pub fn parse_range(value: &str) -> anyhow::Result<PositionList> {
    value
        .split(',')
        .map(|part| match part.split_once('-') {
            Some((start, end)) => {
                let (start, end) = (parse_part(start, part)?, parse_part(end, part)?);
                anyhow::ensure!(
                    start < end,
                    "First number in range ({start}) must be lower than second number ({end})"
                );
                Ok(start - 1..end)
            }
            None => parse_part(part, part).map(|n| n - 1..n),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::parse_range;

    #[test]
    fn valid() {
        let cases = [
            ("1", vec![0..1]),
            ("01", vec![0..1]),
            ("1,3", vec![0..1, 2..3]),
            ("1-3", vec![0..3]),
            ("0001-03", vec![0..3]),
            ("1,7,3-5", vec![0..1, 6..7, 2..5]),
            ("15,19-20", vec![14..15, 18..20]),
        ];
        for (input, expected) in cases {
            assert_eq!(parse_range(input).unwrap(), expected, "input: {input:?}");
        }
    }

    #[test]
    fn illegal() {
        for input in [
            "", "0", "0-1", "+1", "+1-2", "1-+2", "a", "1-a", "a-1", " 1", "1-", "-1",
        ] {
            let err = parse_range(input).unwrap_err().to_string();
            assert_eq!(err, format!(r#"illegal list value: "{input}""#));
        }
        assert_eq!(
            parse_range("1,a").unwrap_err().to_string(),
            r#"illegal list value: "a""#
        );
    }

    #[test]
    fn decreasing_range() {
        for (input, (a, b)) in [("1-1", (1, 1)), ("2-1", (2, 1))] {
            assert_eq!(
                parse_range(input).unwrap_err().to_string(),
                format!("First number in range ({a}) must be lower than second number ({b})")
            );
        }
    }
}
