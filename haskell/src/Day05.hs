module Day05
  ( partOne
  , partTwo
  ) where

import Data.List
import qualified Data.Map as Map

partOne :: String -> Int
partOne =
  length .
  foldrWithIndex (\i x map -> Map.insert i x map) Map.empty . map read . lines

partTwo :: String -> Int
partTwo = length
