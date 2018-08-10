module Day01 where

partOne :: (Num a, Eq a) => [a] -> a
partOne [] = 0
partOne (x:xs) = helper x (xs ++ [x]) 0
  where
    helper _ [] total = total
    helper y (z:zs) total = helper z zs newTotal
      where
        newTotal =
          if y == z
            then y + total
            else total

partTwo :: (Num a, Eq a) => [a] -> a
partTwo xs = sum $ zipWith add (take half xs) (drop half xs)
  where
    half = length xs `div` 2
    add a b =
      if a == b
        then a + b
        else 0
