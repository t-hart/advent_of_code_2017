module Day03Spec where

import Data.Maybe
import Day03
import Test.Hspec
import Test.QuickCheck

spec :: Spec
spec = do
  partOneSpec
  partTwoSpec

returnType :: (Int -> Maybe a) -> Property
returnType f =
  property $ \x ->
    let assert =
          if x < 1
            then isNothing
            else isJust
     in (assert . f) x

partOneSpec :: Spec
partOneSpec =
  describe "Part one" $ do
    it "returns the right Maybe value based on input" $ returnType partOne
    it "solves for arbitrary nums" $
      map partOne [1, 12, 23, 1024] `shouldBe` map Just [0, 3, 2, 31]
    it "solves the puzzle" $ partOne puzzleInput `shouldBe` Just 475

partTwoSpec :: Spec
partTwoSpec =
  describe "Part two" $ do
    it "returns 1 on negative or 0 input" $
      property $ \x -> partTwo (-abs x) == (1 :: Int)
    it "solves for arbitrary nums" $
      map partTwo [330, 329, 747, 746, 1] `shouldBe` [351, 330, 806, 747, 2]
    it "solves the puzzle" $ partTwo puzzleInput `shouldBe` 279138

puzzleInput :: Int
puzzleInput = 277678
