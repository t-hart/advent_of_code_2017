let rec sumListHelper current remaining total =
  match remaining with
  | [] -> total
  | next::rest ->
    let total' = if current == next then total + current else total
    in
    sumListHelper next rest total'

let sumList nums =
  match nums with
  | [] -> 0
  | x::xs -> sumListHelper x (List.concat [xs; [x]]) 0
