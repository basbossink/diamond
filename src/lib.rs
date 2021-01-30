static UPPERS: &str = "ABCDEFHIJKLMNOPQRSTUVWXYZ";

pub fn diamond(letter: char) -> String {
    let upper = upper_diamond(letter);
    let lower = upper
        .lines()
        .rev()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("\n");
    format!("{0}{1}", upper, lower)
}

fn upper_diamond(letter: char) -> String {
    let letters = letters_upto(letter);
    let half_width = letters.len();
    let mut result = String::new();
    for (index, letter) in letters.iter().enumerate() {
        let left = format!("{0:>width$}", letter, width = half_width - index);
        result.push_str(&left);
        if index != 0 {
            result.push_str(&format!("{0:>width$}\n", letter, width = 2 * index));
        } else {
            result.push('\n');
        }
    }
    result
}

fn letters_upto(end: char) -> Vec<char> {
    let upped = end.to_ascii_uppercase();
    let end_index = UPPERS.find(upped);
    match end_index {
        Some(index) => UPPERS.chars().take(index + 1).collect(),
        _ => UPPERS.chars().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letters_upto_singleton() {
        assert_eq!(letters_upto('A'), vec!['A']);
    }

    #[test]
    fn letters_upto_c() {
        assert_eq!(letters_upto('C'), vec!['A', 'B', 'C']);
    }

    #[test]
    fn letters_upto_z() {
        assert_eq!(letters_upto('Z'), UPPERS.chars().collect::<Vec<char>>());
    }

    #[test]
    fn diamond_upper_two_rows() {
        assert_eq!(
            upper_diamond('B').trim(),
            "
 A
B B
"
            .trim()
        );
    }

    #[test]
    fn diamond_upper_four_rows() {
        assert_eq!(
            upper_diamond('D').trim(),
            "
   A
  B B
 C   C
D     D
"
            .trim()
        );
    }

    #[test]
    fn diamond_single_row() {
        assert_eq!(diamond('A').trim(), "A");
    }

    #[test]
    fn diamond_seven_rows() {
        assert_eq!(
            diamond('D').trim(),
            "
   A
  B B
 C   C
D     D
 C   C
  B B
   A
"
            .trim()
        );
    }

    #[test]
    fn diamond_nine_rows() {
        assert_eq!(
            diamond('E').trim(),
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
            .trim()
        );
    }
}
