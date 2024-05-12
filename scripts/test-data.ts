import { parse } from "../src";
import mods from "../test/data/all-mods.json";

Promise.all(Object.keys(mods).map((m) => parse(m).then((r) => [m, r])))
  .then(Object.fromEntries)
  .then((output) =>
    Bun.write(
      "test/data/parse-results.json",
      JSON.stringify(output, undefined, 2)
    )
  );
