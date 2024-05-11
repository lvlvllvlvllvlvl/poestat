import { parse } from "../src";
import mods from "../test/data/all-mods.json";

const output = Object.fromEntries(Object.keys(mods).map((m) => [m, parse(m)]));

Bun.write("test/data/parse-results.json", JSON.stringify(output, undefined, 2));
