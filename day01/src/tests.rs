#[cfg(test)]
pub mod tests {
    use super::super::*;
    use std::path::PathBuf;
    use std::fs;

    #[test]
    fn part1_test1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("inputs/input1.txt");

        let prompt = fs::read_to_string(path).unwrap();
        assert_eq!(part1(prompt), 142);
    }

    #[test]
    fn part1_test2() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("inputs/input2.txt");

        let prompt = fs::read_to_string(path).unwrap();
        assert_eq!(part1(prompt), 55130);
    }

    #[test]
    fn part2_test1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("inputs/input3.txt");

        let prompt = fs::read_to_string(path).unwrap();
        assert_eq!(part2(prompt), 281);
    }

    #[test]
    fn part2_test2() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("inputs/input2.txt");

        let prompt = fs::read_to_string(path).unwrap();
        assert_eq!(part2(prompt.to_string()), 54985);
    }
}
