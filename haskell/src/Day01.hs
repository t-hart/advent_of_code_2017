module Day01 where

partOne :: (Num a, Eq a) => [a] -> a
partOne [] = 0
partOne (x:xs) = sumList const $ zip <*> tail $ (x : xs ++ [x])

partTwo :: (Num a, Eq a) => [a] -> a
partTwo xs = sumList (+) $ uncurry zip $ splitAt half xs
  where
    half = length xs `div` 2

sumList :: (Num a, Eq a) => (a -> a -> a) -> [(a, a)] -> a
sumList eval = foldl (\acc x -> acc + f x) 0
  where
    f (a, b) =
      if a == b
        then eval a b
        else 0
