use std;

fn part_one(target: std::num::NonZeroU32) -> u32 {
    let target = target.get() as i64;
    let sqrt_upper = match (target as f64).sqrt().ceil() as i64 {
        x if x % 2 == 0 => x + 1,
        x => x,
    };
    match sqrt_upper - 1 {
        0 => 0,
        x => (x - (sqrt_upper * sqrt_upper - target) % x) as u32,
    }
}

fn part_two(target: u32) -> u32 {
    type Point = (i32, i32);
    let mut map = std::collections::HashMap::new();
    map.insert((0, 0), 1);

    fn scan_layer(target: u32, layer: i32, map: &mut std::collections::HashMap<Point, u32>) -> u32 {
        fn scan_layer_helper(
            map: &mut std::collections::HashMap<Point, u32>,
            target: u32,
            directions: &[Point],
            current_pos: Point,
        ) -> Option<u32> {
            fn insert_new(index: Point, map: &mut std::collections::HashMap<Point, u32>) -> u32 {
                let options = [-1, 0, 1];
                let value = options
                    .iter()
                    .flat_map(|x| options.iter().map(move |y| (index.0 + x, index.1 + y)))
                    .filter_map(|key| map.get(&key))
                    .sum();
                map.insert(index, value);
                value
            };

            match directions {
                [x, xs..] => {
                    let new_pos = (current_pos.0 + x.0, current_pos.1 + x.1);
                    let v = insert_new(new_pos, map);
                    if v > target {
                        Some(v)
                    } else {
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

        match scan_layer_helper(map, target, &directions, (layer, layer)) {
            Some(v) => v,
            None => scan_layer(target, layer + 1, map),
        }
    }
    scan_layer(target, 1, &mut map)
}
#[cfg(test)]
mod tests {
    use super::*;

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
    fn it_solves_part_one() {
        assert_eq!(475, part_one(non_zero(277678)));
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
        assert_eq!(279138, part_two(277678));
    }

}
