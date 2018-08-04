use std;

fn redist(xs: &[u8]) -> Vec<u8> {
    match xs {
        [] => xs.to_vec(),
        _ => {
            let max = xs.iter().max().unwrap();
            let idx = xs.iter().position(|x| x == max).unwrap();
            let mut clone = xs.to_vec();
            clone[idx] = 0;
            (1..=*max as usize).for_each(|i| clone[(i + idx) % xs.len()] += 1);
            clone
        }
    }
}

fn realloc(xs: &[u8]) -> u32 {
    fn realloc_helper(mut set: std::collections::HashSet<Vec<u8>>, x: &[u8], steps: u32) -> u32 {
        if set.insert(x.to_owned()) {
            realloc_helper(set, &redist(&x), steps + 1)
        } else {
            steps
        }
    }
    let set = std::collections::HashSet::new();
    realloc_helper(set, &xs, 0)
}

fn count_loop(xs: &[u8]) -> u32 {
    fn count_loop_helper(
        mut map: std::collections::HashMap<Vec<u8>, u32>,
        x: &[u8],
        steps: u32,
    ) -> u32 {
        match map.insert(x.to_owned(), steps) {
            None => count_loop_helper(map, &redist(&x), steps + 1),
            Some(prev_step) => steps - prev_step,
        }
    }
    let map = std::collections::HashMap::new();
    count_loop_helper(map, &xs, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    const PUZZLE_INPUT: &str = "0	5	10	0	11	14	13	4	11	8	8	7	1	4	12	11";
    fn puzzle_input_as_vec() -> Vec<u8> {
        PUZZLE_INPUT
            .split_whitespace()
            .filter_map(|x| x.parse::<u8>().ok())
            .collect()
    }

    // part one tests
    #[test]
    fn it_steps_correctly() {
        let steps = [
            vec![0, 2, 7, 0],
            vec![2, 4, 1, 2],
            vec![3, 1, 2, 3],
            vec![0, 2, 3, 4],
            vec![1, 3, 4, 1],
            vec![2, 4, 1, 2],
        ];
        for i in 0..steps.len() - 1 {
            assert_eq!(steps[i + 1], redist(&steps[i]));
        }
    }

    #[test]
    fn it_takes_the_expected_number_of_redists() {
        assert_eq!(5, realloc(&[0, 2, 7, 0]))
    }

    #[test]
    fn part_one() {
        assert_eq!(7864, realloc(&puzzle_input_as_vec()))
    }

    // part two
    #[test]
    fn it_calcs_the_loop_size() {
        assert_eq!(4, count_loop(&[0, 2, 7, 0]))
    }

    #[test]
    fn part_two() {
        assert_eq!(1695, count_loop(&puzzle_input_as_vec()))
    }
}
