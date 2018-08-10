module Day01Spec where

import Day01
import Test.Hspec
import Test.QuickCheck

spec :: Spec
spec = do
  partOneSpec
  partTwoSpec

partOneSpec :: Spec
partOneSpec =
  describe "Part one" $ do
    it "deals with following numbers" $
      partOne [1, 1, 2, 2] `shouldBe` (3 :: Int)
    it "uses head as next for last" $
      partOne (replicate 4 1) `shouldBe` (4 :: Int)
    it "deals with an arbitrary number of repeating numbers" $
      property $ \x n -> partOne (replicate (abs n) x) == abs n * x
    it "returns 0 if there are no matches" $
      partOne [1 .. 5] `shouldBe` (0 :: Int)
    it "works on longer lists" $
      partOne [9, 1, 2, 1, 2, 1, 2, 9] `shouldBe` (9 :: Int)
    it "works on single element lists" $
      property $ \x -> partOne [x] == (x :: Int)

partTwoSpec :: Spec
partTwoSpec =
  describe "Part two" $ do
    it "loops around" $ partTwo [1, 2, 1, 2] `shouldBe` (6 :: Int)
    it "doesn't add if there are no matches" $
      partTwo [1, 2, 2, 1] `shouldBe` (0 :: Int)
    it "adds for longer lists" $
      partTwo [1, 2, 3, 4, 2, 5] `shouldBe` (4 :: Int)
    it "adds all matching values" $
      partTwo [1, 2, 3, 1, 2, 3] `shouldBe` (12 :: Int)
    it "works" $ partTwo [1, 2, 1, 3, 1, 4, 1, 5] `shouldBe` (4 :: Int)
