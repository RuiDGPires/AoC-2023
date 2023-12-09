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

        assert_eq!(part1(prompt.to_string()), 35);
    }

    #[test]
    fn part1_test2() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("inputs/input2.txt");

        let prompt = fs::read_to_string(path).unwrap();

        assert_eq!(part1(prompt.to_string()), 389056265);
    }

    #[test]
    fn part2_test1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("inputs/input1.txt");

        let prompt = fs::read_to_string(path).unwrap();

        assert_eq!(part2(prompt.to_string()), 46);
    }

    #[test]
    fn part2_test2() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("inputs/input2.txt");

        let prompt = fs::read_to_string(path).unwrap();

        assert_eq!(part2(prompt.to_string()), 137516820);
    }

}
