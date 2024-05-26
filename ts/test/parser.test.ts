import { expect, test } from "bun:test";
import { parse, wasm, LANGS } from "../parse";
import mods from "./data/parse-results.json";
import { ParseResult } from "..";

test("all mods (ts)", async () => {
  for (const e of Object.entries(mods)) {
    expect(await parse(e[0])).toEqual(e[1]);
  }
});

test("all mods (wasm)", async () => {
  const parse = await wasm;
  for (const [text, result] of Object.entries(mods)) {
    const tokens = text
      .trim()
      .toLocaleLowerCase("en-US")
      .normalize()
      .split(/\s+/);
    try {
      const found = parse(tokens);
      expect(found).toEqual(matcher(result));
    } catch (e) {
      console.log(tokens, result);
      throw e;
    }
  }
});

test("bad data (wasm)", async () => {
  const parse = await wasm;
  for (const text of [
    "bad data",
    "15% increased bad data",
    "Socketed bad data",
  ]) {
    const tokens = text
      .trim()
      .toLocaleLowerCase("en-US")
      .normalize()
      .split(/\s+/);
    try {
      expect(await parse(tokens)).toEqual([]);
    } catch (e) {
      console.log(tokens);
      throw e;
    }
  }
});

test.each(LANGS.map((l) => [l]))("bad data (%p)", async (lang) => {
  for (const e of ["bad data", "15% increased bad data", "Socketed bad data"]) {
    expect(await parse(e, lang)).toEqual([]);
  }
});

function matcher(result: ParseResult[]) {
  return result.map(({ text, stats }) => ({
    text,
    stats: expect.arrayContaining(
      stats.map((s) => ({
        ...s,
        baseValue: expect.closeTo(s.baseValue),
        parsedValue: expect.closeTo(s.parsedValue),
      }))
    ),
  }));
}
