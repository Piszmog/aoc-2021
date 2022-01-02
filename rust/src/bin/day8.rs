fn main() {}

fn part_1(input: &Vec<Entry>) -> i32 {
    0
}

#[derive(Debug)]
struct Entry {
    signals: Vec<Vec<char>>,
    outputs: Vec<Vec<char>>,
}

#[cfg(test)]
mod day8_tests {
    use crate::{Entry, part_1};

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&get_input()), 0);
    }

    fn get_input() -> Vec<Entry> {
        vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
        ]
            .iter()
            .map(|s| {
                let entry_sides = s.split(" | ").collect::<Vec<&str>>();
                let signals = entry_sides[0].split(" ").map(|sig| sig.chars().collect()).collect::<Vec<Vec<char>>>();
                let outputs = entry_sides[1].split(" ").map(|sig| sig.chars().collect()).collect::<Vec<Vec<char>>>();
                Entry { signals, outputs }
            })
            .collect()
    }
}