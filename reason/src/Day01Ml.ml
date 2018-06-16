let sumList nums =
  let rec sumListHelper current remaining total =
    match remaining with
    | [] -> total
    | next::rest ->
      let total' = if current == next then total + current else total
      in
      sumListHelper next rest total'
  in
  match nums with
  | [] -> 0
  | x::xs -> sumListHelper x (xs @ [x]) 0

let take n list =
  let rec take1 acc n rest =
    match (n, rest) with
    | (0, _) | (_, []) -> acc
    | (_, x::xs) -> take1 (acc @ [x]) (n-1) xs
  in
   take1 [] n list

let drop n list =
  let rec drop1 n rest =
    match ( n , rest ) with
    | (0, _) | (_, []) -> rest
    | (_, _::xs) -> drop1 (n-1) xs
  in
    drop1 n list

let partTwo nums =
  let half = (List.length nums) / 2 in
  let addEqual acc a b = if a == b then acc + a + b else acc in
  List.fold_left2 addEqual 0 (take half nums) (drop half nums)