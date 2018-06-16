module Day01Tests exposing (..)

import Day01 exposing (..)
import Expect exposing (Expectation)
import Test exposing (..)


summing : Test
summing =
    describe "It fulfills the goals of day 1"
        [ describe "Part one: It sums lists correctly"
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
        , describe "Part two: It considers the digit halfway around the list"
            [ test "It loops around" <|
                \_ ->
                    [ 1, 2, 1, 2 ]
                        |> partTwo
                        |> Expect.equal 6
            , test "It doesn't add if there are no matches" <|
                \_ ->
                    [ 1, 2, 2, 1 ]
                        |> partTwo
                        |> Expect.equal 0
            , test "It adds for longer lists" <|
                \_ ->
                    [ 1, 2, 3, 4, 2, 5 ]
                        |> partTwo
                        |> Expect.equal 4
            , test "It adds all matching values" <|
                \_ ->
                    [ 1, 2, 3, 1, 2, 3 ]
                        |> partTwo
                        |> Expect.equal 12
            , test "It works" <|
                \_ ->
                    [ 1, 2, 1, 3, 1, 4, 1, 5 ]
                        |> partTwo
                        |> Expect.equal 4
            ]
        ]
