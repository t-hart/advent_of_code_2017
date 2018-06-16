let sumList = nums => {
  let rec sumList1 = (current, remaining, total) =>
    switch (remaining) {
    | [] => total
    | [next, ...rest] =>
      let newTotal = next === current ? total + current : total;
      sumList1(next, rest, newTotal);
    };

  switch (nums) {
  | [] => 0
  | [x, ...xs] => sumList1(x, xs @ [x], 0)
  };
};

let take = (n, list) => {
  let rec take1 = (acc, n, rest) =>
    switch (n, rest) {
    | (0, _)
    | (_, []) => acc
    | (_, [x, ...xs]) => take1(acc @ [x], n - 1, xs)
    };
  take1([], n, list);
};

let drop = (n, list) => {
  let rec drop1 = (n, rest) =>
    switch (n, rest) {
    | (0, _)
    | (_, []) => rest
    | (_, [_, ...xs]) => drop1(n - 1, xs)
    };
  drop1(n, list);
};

let partTwo = nums => {
  let half = List.length(nums) / 2;
  let addEqual = (acc, a, b) => a === b ? acc + a + b : acc;
  List.fold_left2(addEqual, 0, take(half, nums), drop(half, nums));
};
