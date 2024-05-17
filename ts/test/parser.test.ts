import { expect, test } from "bun:test";
import { parse, wasm, LANGS } from "../parse";
import mods from "./data/parse-results.json";
import { RSError } from "../../pkg/poestat_wasm";

test("all mods", async () => {
  for (const e of Object.entries(mods)) {
    expect(await parse(e[0])).toEqual(e[1]);
  }
});

test("wasm", async () => {
  const hello = await wasm;
  try {
    expect(hello()).toBeGreaterThan(1000);
  } catch (e) {
    if (e instanceof RSError) {
      console.log(e.get_message(), e.get_location(), e.get_debug());
      e.free();
    } else {
      throw e;
    }
  }
});

test.each(LANGS.map((l) => [l]))("bad data (%p)", async (lang) => {
  for (const e of ["bad data", "15% increased bad data", "Socketed bad data"]) {
    expect(await parse(e[0], lang)).toEqual([]);
  }
});
