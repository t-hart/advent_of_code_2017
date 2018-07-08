use std;

type Square = std::num::NonZeroU32;
type LayerIndex = u32;

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
            // let dist = square
            //     .get()
            //     .checked_sub(corner)
            //     .unwrap_or(corner - square.get());
            if dist <= layer.index {
                layer.index - dist
            } else {
                helper(layer, corner - layer.dist_to_corners(), square)
            }
        }
        helper(self, self.end.get(), x)
    }

    fn new(index: u32) -> Layer {
        let end = match index {
            0 => 1,
            _ => (1..=index).fold(1, |acc, x| acc + x * 8),
        };
        Layer {
            index,
            end: std::num::NonZeroU32::new(end).unwrap(),
        }
    }
}

fn get_layer(x: Square) -> Option<Layer> {
    // this means that we can easily find out which layer a square is in by
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

// the number of steps will always be in the range (layer-1) ..= ((layer-1)*2)
// the distance is equal to (layer - 1) + (distance to middle of row/column)
fn part_one(starting_square: Square) -> u32 {
    match get_layer(starting_square) {
        Some(layer) => layer.offset_from_center(starting_square) + layer.index,
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn non_zero(x: u32) -> std::num::NonZeroU32 {
        std::num::NonZeroU32::new(x).unwrap()
    }

    fn assert_layer(index: u32, start: u32, square: u32) {
        let layer = Layer {
            index,
            end: non_zero(start),
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

}
