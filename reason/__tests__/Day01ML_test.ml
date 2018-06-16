open Jest
open Expect
open Day01Ml

let () =

  describe "It sums lists correctly" (fun () ->
    test "it deals with following numbers" (fun () ->
      expect (sumList [1; 1; 2; 2]) |> toBe 3
  );
    test "it uses first as next for last" (fun () ->
      expect (sumList [1; 1; 1; 1]) |> toBe 4
  );
    test "it returns 0 if there are no matches" (fun () ->
      expect (sumList [1; 2; 3; 4]) |> toBe 0
  );
    test "it works on longer lists" (fun () ->
      expect (sumList [9; 1; 2; 1; 2; 1; 2; 9]) |> toBe 9
  );
    test "it returns the element if there is only one" (fun () ->
      expect (sumList [6]) |> toBe 6
  );
);

  describe "Part two: it considers the digit halfway around the list" (fun () ->
      test "it loops around" (fun () ->
          expect (partTwo [ 1; 2; 1; 2 ] ) |> toBe 6
        );
      test "It doesn't add if there are no matches" (fun () ->
          expect (partTwo [ 1; 2; 2; 1 ] ) |> toBe 0
        );
      test "It adds longer lists" (fun () ->
          expect (partTwo [1; 2; 3; 4; 2; 5] ) |> toBe 4
        );
      test "It adds all matching values" (fun () ->
          expect (partTwo [ 1; 2; 3; 1; 2; 3 ] ) |> toBe 12
        );
      test "It works" (fun () ->
          expect (partTwo [ 1; 2; 1; 3; 1; 4; 1; 5 ] ) |> toBe 4
        );
    );
