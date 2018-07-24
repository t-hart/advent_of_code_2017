use std;

// types
type Square = std::num::NonZeroU32;
type LayerIndex = u32;

// part 1

#[derive(Debug, Eq, PartialEq)]
struct Layer {
    index: LayerIndex,
    end: Square,
}

impl Default for Layer {
    fn default() -> Layer {
        Layer {
            index: 0,
            end: std::num::NonZeroU32::new(1).unwrap(),
        }
    }
}

impl Layer {
    fn dist_to_corners(&self) -> u32 {
        self.index * 2
    }

    // The amount of numbers in each layer increases by 8 each consecutive layer
    // (With the first layer being an exception)
    // i.e. 1, 8, 16, 24, 32, 40, 48 ...
    fn squares_in_layer(index: u32) -> u32 {
        match index {
            0 => 1,
            _ => index * 8,
        }
    }

    fn offset_from_center(&self, x: Square) -> u32 {
        fn helper(layer: &Layer, corner: u32, square: Square) -> u32 {
            let dist = std::cmp::max(corner, square.get()) - std::cmp::min(corner, square.get());
            if dist <= layer.index {
                layer.index - dist
            } else {
                helper(layer, corner - layer.dist_to_corners(), square)
            }
        }
        helper(self, self.end.get(), x)
    }

    fn new(index: u32) -> Layer {
        Layer {
            index,
            end: std::num::NonZeroU32::new((0..=index).fold(1, |acc, x| acc + x * 8)).unwrap(),
        }
    }
}

fn get_layer(x: Square) -> Option<Layer> {
    // this means that we can easily scan_layer out which layer a square is in by
    // counting the number of squares in each layer and adding them
    // together until we reach or surpass the number of our square
    fn get_layer_helper(layer: Layer, target: Square) -> Option<Layer> {
        if layer.end >= target {
            Some(layer)
        } else {
            let next_layer_idx = layer.index + 1;
            Layer::squares_in_layer(next_layer_idx)
                .checked_add(layer.end.get())
                .and_then(std::num::NonZeroU32::new)
                .and_then(|sum| {
                    get_layer_helper(
                        Layer {
                            index: next_layer_idx,
                            end: sum,
                        },
                        target,
                    )
                })
        }
    }

    get_layer_helper(Layer::default(), x)
}

fn calculate_mapped(target: u32) -> u32 {
    let mut map = std::collections::HashMap::new();
    map.insert((0, 0), 1);

    fn scan_layer(
        target: u32,
        layer: i32,
        map: &mut std::collections::HashMap<(i32, i32), u32>,
    ) -> u32 {
        fn scan_layer_helper(
            map: &mut std::collections::HashMap<(i32, i32), u32>,
            target: u32,
            directions: &[(i32, i32)],
            current_pos: (i32, i32),
        ) -> Option<u32> {
            fn insert_new(
                index: (i32, i32),
                map: &mut std::collections::HashMap<(i32, i32), u32>,
            ) -> u32 {
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

        let directions: Vec<(i32, i32)> = [(0, -1), (-1, 0), (0, 1), (1, 0)]
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

// execute
fn part_one(starting_square: Square) -> u32 {
    match get_layer(starting_square) {
        Some(layer) => layer.offset_from_center(starting_square) + layer.index,
        None => 0,
    }
}

fn part_two(x: u32) -> u32 {
    calculate_mapped(x)
    // let first = LayerPt2::first();
    // compute(x, &first, &mut LayerPt2::next(&first))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn non_zero(x: u32) -> std::num::NonZeroU32 {
        std::num::NonZeroU32::new(x).unwrap()
    }

    fn assert_layer(index: u32, end: u32, square: u32) {
        let layer = Layer {
            index,
            end: non_zero(end),
        };
        assert_eq!(layer, get_layer(non_zero(square)).unwrap());
        assert_eq!(layer, Layer::new(index))
    }

    #[test]
    fn it_gets_the_right_layer() {
        assert_layer(0, 1, 1);
        assert_layer(1, 9, 2);
        assert_layer(1, 9, 9);
        assert_layer(2, 25, 10);
        assert_layer(2, 25, 23);
        assert_layer(3, 49, 26);
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
