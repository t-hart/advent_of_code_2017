open Jest;
open Day01;
open Expect;

describe("It sums lists correctly", () => {
  test("It deals with following numbers", () =>
    expect(sumList([1, 1, 2, 2])) |> toBe(3)
  );
  test("It uses first as next for last", () =>
    expect(sumList([1, 1, 1, 1])) |> toBe(4)
  );
  test("Returns 0 if there are no matches", () =>
    expect(sumList([1, 2, 3, 4])) |> toBe(0)
  );
  test("Works on longer lists", () =>
    expect(sumList([9, 1, 2, 1, 2, 1, 2, 9])) |> toBe(9)
  );
  test("Returns the element if there is only one", () =>
    expect(sumList([6])) |> toBe(6)
  );
});

describe("Part two: it considers the digit halfway around the list", () => {
  test("it loops around", () =>
    expect(partTwo([1, 2, 1, 2])) |> toBe(6)
  );
  test("It doesn't add if there are no matches", () =>
    expect(partTwo([1, 2, 2, 1])) |> toBe(0)
  );
  test("It adds longer lists", () =>
    expect(partTwo([1, 2, 3, 4, 2, 5])) |> toBe(4)
  );
  test("It adds all matching values", () =>
    expect(partTwo([1, 2, 3, 1, 2, 3])) |> toBe(12)
  );
  test("It works", () =>
    expect(partTwo([1, 2, 1, 3, 1, 4, 1, 5])) |> toBe(4)
  );
});
