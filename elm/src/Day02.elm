module Day02 exposing (..)

import Random


operate : (String -> Int) -> String -> Int
operate mapFunc string =
    string
        |> String.trim
        |> String.lines
        |> List.map mapFunc
        |> List.sum


partOne : String -> Int
partOne input =
    let
        mapFunc line =
            line
                |> String.words
                |> List.foldl
                    (\x acc ->
                        x
                            |> String.toInt
                            |> Result.andThen
                                (\x ->
                                    Ok ({ min = min x acc.min, max = max x acc.max })
                                )
                            |> Result.withDefault acc
                    )
                    { min = Random.maxInt, max = Random.minInt }
                |> \x -> x.max - x.min
    in
        operate mapFunc input


getEvenlyDivisible : Int -> List Int -> Maybe Int
getEvenlyDivisible test remaining =
    case remaining of
        [] ->
            Nothing

        x :: xs ->
            let
                maxVal =
                    max test x

                minVal =
                    min test x
            in
                case maxVal % minVal of
                    0 ->
                        Just <| maxVal // minVal

                    _ ->
                        getEvenlyDivisible test xs


getRes : List Int -> Int
getRes remaining =
    case remaining of
        [] ->
            0

        x :: xs ->
            case getEvenlyDivisible x xs of
                Just val ->
                    val

                Nothing ->
                    getRes xs


partTwo : String -> Int
partTwo input =
    let
        mapFunc line =
            line
                |> String.words
                |> List.filterMap (String.toInt >> Result.toMaybe)
                |> getRes
    in
        operate mapFunc input
