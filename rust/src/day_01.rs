fn sum_list_helper(current: i32, remaining: &[i32], total: i32) -> i32 {
    match remaining {
        [] => total,
        [next, rest..] => {
            let new_total = if current == *next {
                total + current
            } else {
                total
            };
            sum_list_helper(*next, &rest, new_total)
        }
    }
}

fn sum_list(numbers: &[i32]) -> i32 {
    match numbers {
        [] => 0,
        [x, xs..] => {
            let array = vec![xs, &[*x]]
                .iter()
                .flat_map(|s| s.iter())
                .cloned()
                .collect::<Vec<i32>>();
            sum_list_helper(*x, &array, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_adds_following() {
        assert_eq!(sum_list(&[1, 1, 2, 2]), 3);
    }

    #[test]
    fn it_adds_first_to_last() {
        assert_eq!(sum_list(&[1, 1, 1, 1]), 4);
    }

    #[test]
    fn it_returns_zero_if_no_matches() {
        assert_eq!(sum_list(&[1, 2, 3, 4]), 0);
    }

    #[test]
    fn it_works_on_longer_lists() {
        assert_eq!(sum_list(&[9, 1, 2, 1, 2, 1, 2, 9]), 9);
    }

    #[test]
    fn it_works_on_single_element_lists() {
        assert_eq!(sum_list(&[3]), 3);
    }
}
