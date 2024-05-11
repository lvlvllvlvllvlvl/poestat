import { expect, test } from "bun:test";
import mods from "../data/all-mods.json";
import { parse } from "../src";

test("all mods", () => {
  for (const e of Object.entries(mods)) {
    expect(parse(e[0]).map((v) => v.text)).toEqual(e[1]);
  }
});
