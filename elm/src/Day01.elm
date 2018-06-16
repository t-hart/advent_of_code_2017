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


partTwoHelper : ( Int, Int ) -> ( List Int, List Int ) -> Int -> Int
partTwoHelper ( cl, cr ) ( l, r ) total =
    List.map2
        (\x y ->
            if x == y then
                x + y
            else
                0
        )
        l
        r
        |> List.foldl (+) 0


partTwo : List Int -> Int
partTwo nums =
    let
        half =
            (List.length nums) // 2

        addFunc =
            (\x y ->
                if x == y then
                    x + y
                else
                    0
            )
    in
        List.map2 addFunc (List.take half nums) (List.drop half nums)
            |> List.foldl (+) 0
