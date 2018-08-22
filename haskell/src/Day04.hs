module Day04
  ( partOne
  , partTwo
  , validate
  , validate'
  ) where

import qualified Data.HashSet as Set
import Data.List

solve :: (String -> Bool) -> String -> Int
solve f = length . filter f . lines

next _ [] = True
next set (x:xs) = not (Set.member x set) && next (Set.insert x set) xs

partOne :: String -> Int
partOne = solve validate

validate :: String -> Bool
validate = next Set.empty . words

partTwo :: String -> Int
partTwo = solve validate'

validate' :: String -> Bool
validate' = next Set.empty . map sort . words
