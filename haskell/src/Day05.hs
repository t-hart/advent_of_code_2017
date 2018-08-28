module Day05
  ( partOne
  , partTwo
  ) where

import qualified Data.Map as Map

escape :: (Integral a, Integral b) => a -> b -> Map.Map a a -> b
escape index total maze =
  case Map.lookup index maze of
    Nothing -> total
    Just x -> escape (index + x) (total + 1) (Map.insert index (x + 1) maze)

toMaze :: String -> Map.Map Int Int
toMaze = Map.fromAscList . zip [0 ..] . map read . lines

partOne :: String -> Int
partOne = escape 0 0 . toMaze

partTwo :: String -> Int
partTwo = length
