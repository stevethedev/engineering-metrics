import { processVars } from "./process-vars";

describe("processVars", () => {
  it("should return an empty object if no vars are passed", () => {
    expect(processVars()).toEqual({});
  });

  it("should return an empty object if an empty object is passed", () => {
    expect(processVars({})).toEqual({});
  });

  it("should return an empty object if an object with no valid vars is passed", () => {
    expect(
      processVars({
        foo: undefined as unknown as string,
        bar: null as unknown as string,
        baz: "",
      })
    ).toEqual({});
  });

  it("should return an object with valid vars", () => {
    expect(processVars({ foo: "bar", baz: 1 })).toEqual({
      "--foo": "bar",
      "--baz": 1,
    });
  });

  it("should return an object with valid vars and ignore invalid vars", () => {
    expect(
      processVars({
        foo: "bar",
        baz: 1,
        qux: undefined as unknown as string,
        quux: null as unknown as string,
      })
    ).toEqual({ "--foo": "bar", "--baz": 1 });
  });

  it("should process vars with a leading --", () => {
    expect(processVars({ "--foo": "bar", "--baz": 1 })).toEqual({
      "--foo": "bar",
      "--baz": 1,
    });
  });

  it("should process vars with object values", () => {
    expect(processVars({ foo: { default: "baz" } })).toEqual({
      "--foo": "baz",
    });

    expect(
      processVars({
        foo: { default: "baz", tablet: "tab-value", desktop: "dt-value" },
        fooBar: { default: "baz", tablet: "tab-value", desktop: "dt-value" },
      })
    ).toEqual({
      "--foo": "baz",
      "--foo--tablet": "tab-value",
      "--foo--desktop": "dt-value",
      "--foo-bar": "baz",
      "--foo-bar--tablet": "tab-value",
      "--foo-bar--desktop": "dt-value",
    });
  });
});
