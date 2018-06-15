module Day01 exposing (..)


sumListHelper : Int -> List Int -> Int -> Int
sumListHelper current remaining total =
    case remaining of
        [] ->
            total

        next :: xs ->
            let
                newTotal =
                    if current == next then
                        total + current
                    else
                        total
            in
                sumListHelper next xs newTotal


sumList : List Int -> Int
sumList nums =
    case nums of
        x :: xs ->
            sumListHelper x (xs ++ [ x ]) 0

        [] ->
            0
