static UPPERS: &str = "ABCDEFHIJKLMNOPQRSTUVWXYZ";

pub fn diamond(letter: char) -> String {
    format!("{0}{1}", upper_diamond(letter), lower_diamond(letter))
}

fn upper_diamond(letter: char) -> String {
    let mut result = String::new();
    let letters = letters_upto(letter);
    let half_width = letters.len();
    for (index, c) in letters.iter().enumerate() {
        result.push_str(&pad_right(
            &pad_left(&c.to_string(), half_width - index),
            half_width,
        ));
        if index != 0 {
            result.push_str(&pad_left(&c.to_string(), index));
        }
        result.push('\n');
    }
    result
}

fn lower_diamond(letter: char) -> String {
    let mut result = String::new();
    let letters = letters_upto(letter);
    let half_width = letters.len();
    for (index, c) in letters.iter().rev().skip(1).enumerate() {
        result.push_str(&pad_right(&pad_left(&c.to_string(), index + 2), half_width));
        if index != half_width - 2 {
            result.push_str(&pad_left(&c.to_string(), half_width - index - 2));
        }
        result.push('\n');
    }
    result
}

fn letters_upto(end: char) -> Vec<char> {
    let upped = end.to_ascii_uppercase();
    let mut upto = UPPERS
        .chars()
        .take_while(|c| c != &upped)
        .collect::<Vec<char>>();
    upto.push(upped);
    upto
}

fn pad_left(to_pad: &str, padded_size: usize) -> String {
    format!("{0:>width$}", to_pad, width = padded_size).to_string()
}

fn pad_right(to_pad: &str, padded_size: usize) -> String {
    format!("{0:<width$}", to_pad, width = padded_size).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod letters_upto {
        use super::*;

        macro_rules! test_letters_upto {
            ($name:ident, $input:expr, $expected:expr) => {
                #[test]
                fn $name() {
                    assert_eq!(letters_upto($input), $expected);
                }
            };
        }

        test_letters_upto!(a, 'A', vec!['A']);
        test_letters_upto!(c, 'C', vec!['A', 'B', 'C']);
        test_letters_upto!(z, 'Z', UPPERS.chars().collect::<Vec<char>>());
    }

    mod pad {
        use super::*;
        use proptest::prelude::*;

        proptest! {

            #[test]
            fn pad_left_does_not_pad(to_pad in "[A-Z]{0,100}") {
                let original_len = to_pad.len();
                prop_assert_eq!(&to_pad, &pad_left(&to_pad, original_len));
            }

            #[test]
            fn pad_left_pads_to_given_length(to_pad in "[A-Z]{0,100}",
                                        padded_size in 1usize..1000usize) {
                prop_assume!(padded_size > to_pad.len());
                prop_assert_eq!(padded_size, pad_left(&to_pad, padded_size).len());
            }

            #[test]
            fn pad_right_does_not_pad(to_pad in "[A-Z]{0,100}") {
                let original_len = to_pad.len();
                prop_assert_eq!(&to_pad, &pad_right(&to_pad, original_len));
            }

            #[test]
            fn pad_right_pads_to_given_length(to_pad in "[A-Z]{0,100}",
                                        padded_size in 1usize..1000usize) {
                prop_assume!(padded_size > to_pad.len());
                prop_assert_eq!(padded_size, pad_right(&to_pad, padded_size).len());
            }
        }

        macro_rules! test_pad {
            ($name:ident, $fname:ident, $input:expr, $padded_size:expr, $expected:expr) => {
                #[test]
                fn $name() {
                    let actual = $fname(&$input.to_string(), $padded_size);
                    assert_eq!($expected, actual);
                }
            };
        }

        test_pad!(right_1, pad_right, " a", 3usize, " a ");
        test_pad!(right_2, pad_right, "a ", 4usize, "a   ");
        test_pad!(right_3, pad_right, "a", 5usize, "a    ");
        test_pad!(right_4, pad_right, " a", 8usize, " a      ");
        test_pad!(left_1, pad_left, "a", 3usize, "  a");
        test_pad!(left_2, pad_left, "  a", 4usize, "   a");
        test_pad!(left_3, pad_left, "a", 2usize, " a");
        test_pad!(left_4, pad_left, "a", 10usize, "         a");
    }

    mod diamond {
        use super::*;

        macro_rules! test_diamond {
            ($name:ident, $fname:ident, $input:expr, $expected:expr) => {
                #[test]
                fn $name() {
                    let result = $fname($input);
                    let actual = result.trim();
                    assert_eq!($expected.trim(), actual);
                }
            };
        }

        mod upper {
            use super::*;

            test_diamond!(
                b,
                upper_diamond,
                'B',
                "
 A
B B
"
            );

            test_diamond!(
                d,
                upper_diamond,
                'D',
                "
   A
  B B
 C   C
D     D
"
            );
        }

        mod lower {
            use super::*;

            test_diamond!(
                d,
                lower_diamond,
                'D',
                "
 C   C
  B B
   A
"
            );
        }

        mod full {
            use super::*;

            test_diamond!(a, diamond, 'A', "A");

            test_diamond!(
                d,
                diamond,
                'D',
                "
   A
  B B
 C   C
D     D
 C   C
  B B
   A
"
            );

            test_diamond!(
                e,
                diamond,
                'E',
                "
    A
   B B
  C   C
 D     D
E       E
 D     D
  C   C
   B B
    A
"
            );
        }
    }
}
