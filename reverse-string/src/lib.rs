
use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    format!("{}", input.to_string()
                    .graphemes(true)
                    .rev()
                    .collect::<String>()
                )
}
