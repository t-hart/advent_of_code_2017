module Day05
  ( partOne
  , partTwo
  ) where

import qualified Data.Map as Map

escape ::
     (Integral a, Integral b)
  => a
  -> b
  -> (a -> a -> a -> a)
  -> Map.Map a a
  -> b
escape index total f maze =
  case Map.lookup index maze of
    Nothing -> total
    Just x ->
      escape (index + x) (total + 1) f (Map.insertWith (f x) index 1 maze)

toMaze :: String -> Map.Map Int Int
toMaze = Map.fromAscList . zip [0 ..] . map read . lines

partOne :: String -> Int
partOne = escape 0 0 (const (+)) . toMaze

partTwo :: String -> Int
partTwo = escape 0 0 f . toMaze
  where
    f x =
      if x >= 3
        then flip (-)
        else (+)
