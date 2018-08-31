module Day03 where

import qualified Data.Map as Map
import Data.Maybe
import Debug.Trace

partOne :: Integral a => a -> Maybe a
partOne 1 = Just 0
partOne x
  | x < 1 = Nothing
  | otherwise =
    let intermediate = (ceiling . sqrt . fromIntegral) x
        remainder = intermediate `mod` 2
        sqrtUpper = intermediate + 1 - remainder
        maxDist = intermediate - remainder
     in Just (maxDist - (sqrtUpper ^ 2 - x) `mod` maxDist)

partTwo :: Integral a => a -> a
partTwo target =
  let addPair (x1, y1) (x2, y2) = (x1 + x2, y1 + y2)
      options = addPair <$> [(x, y) | x <- terms, y <- terms, (x, y) /= (0, 0)]
        where
          terms = [-1, 0, 1]
      scan vmap layer =
        let moves =
              addPair <$> [(0, -1), (-1, 0), (0, 1), (1, 0)] >>=
              replicate (fromIntegral layer * 2)
         in case helper vmap moves (layer, layer) of
              Left vmap' -> scan vmap' (layer + 1)
              Right x -> x
        where
          helper vmap' [] _ = Left vmap'
          helper vmap' (x:xs) pos =
            let newPos = x pos
                value =
                  sum . mapMaybe (`Map.lookup` vmap') $ options <*> [newPos]
             in if value > target
                  then Right value
                  else helper (Map.insert newPos value vmap') xs newPos
   in scan (Map.singleton (0, 0) 1) 1
