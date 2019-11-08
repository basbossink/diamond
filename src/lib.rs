static UPPERS: &str = "ABCDEFHIJKLMNOPQRSTUVWXYZ";

fn letters_upto(end: char) -> Vec<char> {
    let upped = end.to_ascii_uppercase();
    match UPPERS.find(upped) {
        Some(index) => UPPERS.chars().take(index + 1).collect(),
        None => UPPERS.chars().collect(),
    }
}

pub fn diamond(end: char) -> String {
    let upper = upper_diamond(end);
    upper
        .iter()
        .chain(upper.iter().rev().skip(1))
        .flat_map(|s| s.chars())
        .collect::<String>()
        .trim_end()
        .to_string()
}

fn upper_diamond(end: char) -> Vec<String> {
    let letters = letters_upto(end);
    let half_width = letters.len();
    letters
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let char_str = &item.to_string();
            let left = pad_right(&pad_left(char_str, half_width - i), half_width);
            let right = if i > 0 {
                pad_left(char_str, i)
            } else {
                String::new()
            };
            format!("{0}{1}\n", left, right)
        })
        .collect()
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
           ( $(($name:ident, $input:expr, $expected:expr)),* ) => {
               $(
                   #[test]
                   fn $name() {
                       assert_eq!(letters_upto($input), $expected);
                   }
               )*
           };
        }

        test_letters_upto!(
            (a, 'A', vec!['A']),
            (c, 'C', vec!['A', 'B', 'C']),
            (z, 'Z', UPPERS.chars().collect::<Vec<char>>())
        );
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
            ( $(($name:ident, $fname:ident, $input:expr, $padded_size:expr, $expected:expr)),* ) => {
                $(
                    #[test]
                    fn $name() {
                        let actual = $fname(&$input.to_string(), $padded_size);
                        assert_eq!($expected, actual);
                    }
                )*
            };
        }
        test_pad!(
            (right_1, pad_right, " a", 3usize, " a "),
            (right_2, pad_right, "a ", 4usize, "a   "),
            (right_3, pad_right, "a", 5usize, "a    "),
            (right_4, pad_right, " a", 8usize, " a      "),
            (left_1, pad_left, "a", 3usize, "  a"),
            (left_2, pad_left, "  a", 4usize, "   a"),
            (left_3, pad_left, "a", 2usize, " a"),
            (left_4, pad_left, "a", 10usize, "         a")
        );
    }
    mod diamond {
        use super::*;
        macro_rules! test_diamond {
            ($name:ident, $fname:ident, $input:expr, $expected:expr) => {
                #[test]
                fn $name() {
                    let result = $fname($input);
                    assert_eq!($expected, result);
                }
            };
        }
        mod upper {
            use super::*;
            test_diamond!(b, upper_diamond, 'B', vec![" A\n", "B B\n"]);
            test_diamond!(
                d,
                upper_diamond,
                'D',
                vec!["   A\n", "  B B\n", " C   C\n", "D     D\n"]
            );
        }
        mod full {
            use super::*;
            test_diamond!(a, diamond, 'A', "A");
            test_diamond!(
                d,
                diamond,
                'D',
                "   A
  B B
 C   C
D     D
 C   C
  B B
   A"
            );
            test_diamond!(
                e,
                diamond,
                'E',
                "    A
   B B
  C   C
 D     D
E       E
 D     D
  C   C
   B B
    A"
            );
        }
    }
}
