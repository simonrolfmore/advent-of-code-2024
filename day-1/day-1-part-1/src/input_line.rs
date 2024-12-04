    use std::str::FromStr;

    pub struct InputLine {
        pub left_val: i32,
        pub right_val: i32,
    }
    #[derive(Debug)]
    pub struct InputLineParseError;

    impl FromStr for InputLine {
        type Err = InputLineParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {

            let left_val: i32 = s[..5].parse().map_err(|_| InputLineParseError)?;
            let right_val: i32 = s[8..].parse().map_err(|_| InputLineParseError)?;
            Ok(InputLine {
                left_val,
                right_val,
            })
        }
    }
