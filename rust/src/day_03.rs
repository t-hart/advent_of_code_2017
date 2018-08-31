use num;
use std;

fn part_one(target: std::num::NonZeroU32) -> u32 {
    match target.get() {
        1 => 0,
        target => {
            let (sqrt_upper, max_dist) = {
                let intermediate = (f64::from(target)).sqrt().ceil() as u32;
                let rem = intermediate % 2;
                (intermediate + 1 - rem, intermediate - rem)
            };
            max_dist - (sqrt_upper * sqrt_upper - target) % max_dist
        }
    }
}

fn part_one_generic<T>(target: T) -> Option<T>
where
    T: num::Unsigned + num::Integer + num::NumCast + Copy,
{
    let zero = num::Zero::zero();
    let one = num::One::one();
    match target {
        _ if target == zero => None,
        _ if target == one => Some(zero),
        _ => {
            let (sqrt_upper, max_dist) = {
                num::NumCast::from(target)
                    .and_then(|x: f64| {
                        num::NumCast::from(x.sqrt().ceil()).and_then(|intermediate: T| {
                            let rem = intermediate % (one + one);
                            Some((intermediate + one - rem, intermediate - rem))
                        })
                    }).unwrap_or((zero, zero))
            };
            Some(max_dist - (sqrt_upper * sqrt_upper - target) % max_dist)
        }
    }
}

fn part_two(target: u32) -> u32 {
    type Point = (i32, i32);
    fn scan_layer(target: u32, layer: i32, map: &mut std::collections::HashMap<Point, u32>) -> u32 {
        fn scan_layer_helper(
            map: &mut std::collections::HashMap<Point, u32>,
            target: u32,
            directions: &[Point],
            current_pos: Point,
        ) -> Option<u32> {
            const OPTIONS: [i32; 3] = [-1, 0, 1];
            match directions {
                [x, xs..] => {
                    let new_pos = (current_pos.0 + x.0, current_pos.1 + x.1);
                    let v = OPTIONS
                        .iter()
                        .flat_map(|x| OPTIONS.iter().map(move |y| (new_pos.0 + x, new_pos.1 + y)))
                        .filter_map(|key| map.get(&key))
                        .sum();
                    if v > target {
                        Some(v)
                    } else {
                        map.insert(new_pos, v);
                        scan_layer_helper(map, target, xs, new_pos)
                    }
                }
                [] => None,
            }
        }

        let directions: Vec<Point> = [(0, -1), (-1, 0), (0, 1), (1, 0)]
            .iter()
            .flat_map(|dir| (1..=layer * 2).map(move |_| (dir.0, dir.1)))
            .collect();

        scan_layer_helper(map, target, &directions, (layer, layer))
            .unwrap_or_else(|| scan_layer(target, layer + 1, map))
    }

    let mut map = std::collections::HashMap::new();
    map.insert((0, 0), 1);
    scan_layer(target, 1, &mut map)
}

#[cfg(test)]
mod tests {
    use super::*;
    const PUZZLE_INPUT: u32 = 277678;

    fn non_zero(x: u32) -> std::num::NonZeroU32 {
        std::num::NonZeroU32::new(x).unwrap()
    }

    #[test]
    fn part_one_tests() {
        assert_eq!(0, part_one(non_zero(1)));
        assert_eq!(3, part_one(non_zero(12)));
        assert_eq!(2, part_one(non_zero(23)));
        assert_eq!(31, part_one(non_zero(1024)));
    }

    #[test]
    fn part_one_generic_tests() {
        assert_eq!(None, part_one_generic(0_u32));
        assert_eq!(Some(0), part_one_generic(1_u32));
        assert_eq!(Some(3), part_one_generic(12_u32));
        assert_eq!(Some(2), part_one_generic(23_u32));
        assert_eq!(Some(31), part_one_generic(1024_u32));
    }

    #[test]
    fn it_solves_part_one() {
        assert_eq!(475, part_one(non_zero(PUZZLE_INPUT)));
    }

    #[test]
    fn it_solves_part_one_generic() {
        assert_eq!(Some(475), part_one_generic(PUZZLE_INPUT));
    }

    #[test]
    fn part_two_tests() {
        assert_eq!(351, part_two(330));
        assert_eq!(330, part_two(329));
        assert_eq!(806, part_two(747));
        assert_eq!(747, part_two(746));
        assert_eq!(2, part_two(1));
    }

    #[test]
    fn it_solves_part_two() {
        assert_eq!(279138, part_two(PUZZLE_INPUT));
    }

}
