//extern crate caseless;
extern crate unicode_general_category;
extern crate unicode_normalization;

use self::unicode_general_category::{get_general_category, GeneralCategory, GeneralCategory::*};
use self::unicode_normalization::UnicodeNormalization;

#[derive(Debug)]
#[allow(dead_code)]
struct Fid {
    canonical: String
}

pub enum FidCharType {
    Content,
    Delim,
    Ignorable
}

impl FidCharType {
    pub fn new(gc: GeneralCategory) -> FidCharType {
        use parser::fid::FidCharType::*;
        match gc {
            LowercaseLetter => Content,
            UppercaseLetter => Content,
            DecimalNumber => Content,
            SpaceSeparator => Delim,
            CurrencySymbol => Content,
            EnclosingMark => Content,
            LetterNumber => Content,
            MathSymbol => Content,
            OtherLetter => Content,
            OtherNumber => Content,
            OtherSymbol => Content,
            PrivateUse => Content,
            Surrogate => Content,
            TitlecaseLetter => Content,
            NonspacingMark => Ignorable,
            ModifierLetter => Ignorable,
            ModifierSymbol => Ignorable,
            Unassigned => Ignorable,
            _ => Delim
        }
    }
}

impl Fid {
    #[allow(dead_code)]
    pub fn new(friendly: &str) -> Fid {
        let mut s = String::with_capacity(friendly.len());
        let mut last_was_hyphen = false;
        for c in friendly.nfkd() {
            let fid_type = FidCharType::new(get_general_category(c));
            match fid_type {
                FidCharType::Content => {
                    last_was_hyphen = false;
                    s.push(c)
                },
                FidCharType::Delim => {
                    if !last_was_hyphen {
                        s.push('-');
                        last_was_hyphen = true
                    }
                },
                _ => {}
            }
        }
        Fid {
            canonical: caseless::default_case_fold_str(&s)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Fid;

    #[test]
    fn create_fid() {
        let f = Fid::new("Abc Def");
        assert_eq!(
            f.canonical, "abc-def"
        );
    }
}
