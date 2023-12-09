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

        assert_eq!(part1(prompt.to_string()), 2);
    }

    #[test]
    fn part1_test2() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("inputs/input2.txt");

        let prompt = fs::read_to_string(path).unwrap();

        assert_eq!(part1(prompt.to_string()), 19637);
    }

    #[test]
    fn part1_test3() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("inputs/input3.txt");

        let prompt = fs::read_to_string(path).unwrap();

        assert_eq!(part1(prompt.to_string()), 6);
    }

    #[test]
    fn part2_test1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("inputs/input4.txt");

        let prompt = fs::read_to_string(path).unwrap();

        assert_eq!(part2(prompt.to_string()), 6);
    }

    #[test]
    fn part2_test2() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("inputs/input2.txt");

        let prompt = fs::read_to_string(path).unwrap();

        assert_eq!(part2(prompt.to_string()), 8811050362409);
    }
}
