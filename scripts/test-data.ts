import { parse, type ParseResult } from "../src";
import mods from "../data/all-mods.json";

const output = Object.fromEntries(Object.keys(mods).map((m) => [m, parse(m)]));

Bun.write("data/parse-results.json", JSON.stringify(output, undefined, 2));
