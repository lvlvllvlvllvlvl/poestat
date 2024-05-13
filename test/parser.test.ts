import { expect, test } from "bun:test";
import { parse, LANGS } from "../ts";
import mods from "./data/parse-results.json";

import init, { RSError, hello } from "../pkg/poestat_wasm";

test("all mods (%p)", async () => {
  for (const e of Object.entries(mods)) {
    expect(await parse(e[0])).toEqual(e[1]);
  }
});

test.each(LANGS.map((l) => [l]))("bad data (%p)", async (lang) => {
  for (const e of ["bad data", "15% increased bad data", "Socketed bad data"]) {
    expect(await parse(e[0], lang)).toEqual([]);
  }
});

test("wasm", async () => {
  await init();
  const json = (await Promise.all(
    [
      `https://lvlvllvlvllvlvl.github.io/RePoE/stats_by_file.min.json`,
      `https://lvlvllvlvllvlvl.github.io/RePoE/stat_value_handlers.min.json`,
    ].map((u) => fetch(u).then((r) => r.text()))
  )) as [string, string];
  try {
    expect(hello(...json)).toBeGreaterThan(150000);
  } catch (e) {
    if (e instanceof RSError) {
      console.log(e.get_message(), e.get_location(), e.get_debug());
      e.free();
    } else {
      throw e;
    }
  }
});
