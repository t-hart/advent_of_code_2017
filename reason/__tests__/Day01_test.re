open Jest;
open Day01;

describe("It sums lists correctly", () => {
  open Expect;

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
