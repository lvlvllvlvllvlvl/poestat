import { expect, test } from "bun:test";
import mods from "../data/all-mods.json";
import { search } from "../src";

test("all mods", () => {
  for (const e of Object.entries(mods)) {
    expect(search(e[0])).toEqual(e[1]);
  }
});
