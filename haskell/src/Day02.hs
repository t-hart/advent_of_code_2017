module Day02 where

import Data.Maybe

nums :: String -> [[Int]]
nums = map (map read . words) . lines

solve :: ([Int] -> Int) -> String -> Int
solve f = sum . map f . nums

partOne :: String -> Int
partOne = solve (\xs -> maximum xs - minimum xs)

partTwo :: String -> Int
partTwo = solve calc
  where
    calc [] = 0
    calc (x:xs) = fromMaybe (calc xs) (divisible x xs)
      where
        divisible _ [] = Nothing
        divisible y (z:zs) =
          let hi = max y z
              lo = min y z
           in case hi `mod` lo of
                0 -> Just (hi `div` lo)
                _ -> divisible y zs
