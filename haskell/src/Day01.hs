module Day01 where

solve :: (Eq a, Num b) => ((a, a) -> b) -> [a] -> [a] -> b
solve f xs ys = sum . map f . filter (uncurry (==)) $ zip xs ys

partOne :: (Num a, Eq a) => [a] -> a
partOne [] = 0
partOne xs = solve fst xs shifted
  where
    shifted = tail $ cycle xs

partTwo :: (Num a, Eq a) => [a] -> a
partTwo xs = solve ((* 2) . fst) start end
  where
    half = length xs `div` 2
    (start, end) = splitAt half xs
