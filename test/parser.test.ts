import { expect, test } from "bun:test";
import { parse } from "../ts";
import mods from "./data/parse-results.json";

import init, { hello } from "../pkg/poestat_wasm";

test("all mods", async () => {
  for (const e of Object.entries(mods)) {
    expect(await parse(e[0])).toEqual(e[1]);
  }
});

test("wasm", async () => {
  for (const e of Object.entries(mods)) {
    await init()
    expect(hello()).toEqual("hellp");
  }
});
