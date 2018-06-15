let rec sumListHelper = (current, remaining, total) =>
  switch (remaining) {
  | [] => total
  | [next, ...rest] =>
    let newTotal = next === current ? total + current : total;
    sumListHelper(next, rest, newTotal);
  };

let sumList = nums =>
  switch (nums) {
  | [] => 0
  | [x, ...xs] => sumListHelper(x, List.concat([xs, [x]]), 0)
  };
