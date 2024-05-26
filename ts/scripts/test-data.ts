import { parse } from "../";
import mods from "../test/data/all-mods.json";

Promise.all(Object.keys(mods).map((m) => parse(m).then((r) => [m, r])))
  .then(Object.fromEntries)
  .then((output) =>
    Bun.write(
      "ts/test/data/parse-results.json",
      JSON.stringify(output, undefined, 2)
    )
  );
