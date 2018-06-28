let operate = (mapFunc, input) =>
  input
  |> String.trim
  |> Js.String.split("\n")
  |> Array.to_list
  |> List.map(mapFunc)
  |> List.fold_left((+), 0);

type minMax = {
  min: int,
  max: int,
};

let splitWhitespace = s => Js.String.splitByRe([%re {|/\s+/|}], s);

let log = (~text as t=?, value) => {
  let outStr =
    switch (t) {
    | Some(text) => text ++ ": "
    | None => ""
    };
  print_endline({j|$outStr$value|j});
  value;
};

let partOne = input => {
  let mapFunc = line => {
    let foldFunc = (acc, x) =>
      x
      |> String.trim
      |> int_of_string
      |> (x => {min: min(x, acc.min), max: max(x, acc.max)});
    line
    |> String.trim
    |> splitWhitespace
    |> Array.to_list
    |> List.map(String.trim)
    |> List.fold_left(foldFunc, {min: max_int, max: min_int})
    |> (x => x.max - x.min);
  };

  operate(mapFunc, input);
};

let rec getEvenlyDivisible = (test, remaining) =>
  switch (remaining) {
  | [] => None
  | [x, ...xs] =>
    let max = max(test, x);
    let min = min(test, x);
    switch (max mod min) {
    | 0 => Some(max / min)
    | _ => getEvenlyDivisible(test, xs)
    };
  };

let rec getRes = remaining =>
  switch (remaining) {
  | [] => 0
  | [x, ...xs] =>
    switch (getEvenlyDivisible(x, xs)) {
    | Some(value) => value
    | None => getRes(xs)
    }
  };

let partTwo = input => {
  let mapFunc = line =>
    line
    |> String.trim
    |> splitWhitespace
    |> Array.to_list
    |> List.map(int_of_string)
    |> getRes;
  operate(mapFunc, input);
};
