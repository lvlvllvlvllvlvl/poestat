import { expect, test } from "bun:test";
import mods from "./data/parse-results.json";
import { parse } from "../src";

test("all mods", () => {
  for (const e of Object.entries(mods)) {
    expect(parse(e[0])).toEqual(e[1]);
  }
});
