use parser::Block;
use parser::Block::Hr;
use regex::Regex;

pub fn parse_hr(lines: &[&str]) -> Option<(Block, usize)> {
    lazy_static! {
        static ref HORIZONTAL_RULE: Regex = Regex::new(r"^___+$").unwrap();
    }

    if HORIZONTAL_RULE.is_match(lines[0]) {
        return Some((Hr, 1));
    }
    None
}

#[cfg(test)]
mod test {
    use super::parse_hr;
    use parser::Block::Hr;

    #[test]
    fn finds_hr() {
        assert_eq!(parse_hr(&vec!["_______"]).unwrap(), (Hr, 1));
        assert_eq!(parse_hr(&vec!["___"]).unwrap(), (Hr, 1));
        assert_eq!(
            parse_hr(&vec!["____________________"]).unwrap(),
            (Hr, 1)
        );
        assert_eq!(parse_hr(&vec!["_______", "abc"]).unwrap(), (Hr, 1));
    }

    #[test]
    fn no_false_positives() {
        // Don't support markdown's <hr> made out of ===
        assert_eq!(parse_hr(&vec!["======="]), None);
        assert_eq!(parse_hr(&vec!["==="]), None);
        assert_eq!(parse_hr(&vec!["============================"]), None);
        assert_eq!(parse_hr(&vec!["=======", "abc"]), None);
        // Or made out of ***
        assert_eq!(parse_hr(&vec!["***"]), None);
        // Or made out of --- (this turns into a YAML symbol instead)
        assert_eq!(parse_hr(&vec!["---"]), None);


        assert_eq!(parse_hr(&vec!["a-------"]), None);
        assert_eq!(parse_hr(&vec!["--- a"]), None);
        assert_eq!(parse_hr(&vec!["--a-"]), None);
        assert_eq!(parse_hr(&vec!["-------====--------------"]), None);

        assert_eq!(parse_hr(&vec!["a======"]), None);
        assert_eq!(parse_hr(&vec!["=== a"]), None);
        assert_eq!(parse_hr(&vec!["==a="]), None);
        assert_eq!(parse_hr(&vec!["=======---================="]), None);
    }
}
