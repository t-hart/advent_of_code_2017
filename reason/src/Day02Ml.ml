let operate map_func input =
  input
  |> String.trim
  |> Js.String.split "\n"
  |> Array.to_list
  |> List.map map_func
  |> List.fold_left (+) 0

let split_whitespace str = Js.String.splitByRe [%re {|/\s+/|}] str

let trim_split s = s |> String.trim |> split_whitespace


type min_max = { min: int; max: int; }

let part_one input =
  let map_func line =
    let fold_func acc x =
      x
      |> String.trim
      |> int_of_string
      |> (fun (x) -> { min = min x acc.min; max = max x acc.max })
    in

    line
    |> trim_split
    |> Array.to_list
    |> List.fold_left fold_func { min = max_int; max = min_int }
    |> (fun (x) -> x.max - x.min)
  in
  operate map_func input

let rec get_evenly_divisible test remaining =
  match remaining with
  | [] -> None
  | x :: xs -> let max = max test x in
    let min = min test x in
    match max mod min with
    |0 -> Some (max / min)
    |_ -> get_evenly_divisible test xs


let rec get_res remaining =
  match remaining with
  | [] -> 0
  | x :: xs ->
    match get_evenly_divisible x xs with
    | Some(value) -> value
    | None -> get_res xs

let part_two input =
  let map_func line =
    line
    |> trim_split
    |> Array.to_list
    |> List.map int_of_string
    |> get_res
  in
  operate map_func input