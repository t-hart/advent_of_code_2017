module Day01Tests exposing (..)

import Day01 exposing (..)
import Expect exposing (Expectation)
import Test exposing (..)


summing : Test
summing =
    describe "It sums lists correctly"
        [ test "It deals with following numbers" <|
            \_ ->
                [ 1, 1, 2, 2 ]
                    |> sumList
                    |> Expect.equal 3
        , test "It uses first as next for last" <|
            \_ ->
                [ 1, 1, 1, 1 ]
                    |> sumList
                    |> Expect.equal 4
        , test "It returns 0 if no matches" <|
            \_ ->
                [ 1, 2, 3, 4 ]
                    |> sumList
                    |> Expect.equal 0
        , test "It works on longer lists" <|
            \_ ->
                [ 9, 1, 2, 1, 2, 1, 2, 9 ]
                    |> sumList
                    |> Expect.equal 9
        , test "It returns the element if there is only one" <|
            \_ ->
                [ 2 ]
                    |> sumList
                    |> Expect.equal 2
        ]
