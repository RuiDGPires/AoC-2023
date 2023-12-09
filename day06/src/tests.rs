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

        assert_eq!(part1(prompt.to_string()), 288);
    }

    #[test]
    fn part1_test2() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("inputs/input2.txt");

        let prompt = fs::read_to_string(path).unwrap();

        assert_eq!(part1(prompt.to_string()), 1108800);
    }

    #[test]
    fn part2_test1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("inputs/input1.txt");

        let prompt = fs::read_to_string(path).unwrap();

        assert_eq!(part2(prompt.to_string()), 71503);
    }

    #[test]
    fn part2_test2() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("inputs/input2.txt");

        let prompt = fs::read_to_string(path).unwrap();

        assert_eq!(part2(prompt.to_string()), 36919753);
    }

}
