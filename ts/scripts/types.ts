import { compile } from "json-schema-to-typescript";

fetch(
  "https://raw.githubusercontent.com/lvlvllvlvllvlvl/RePoE/master/RePoE/schema/stats_by_file.schema.json"
)
  .then((r) => r.json())
  .then((j) => {
    Bun.write("schema/stats.schema.json", JSON.stringify(j));
    return j;
  })
  .then((s) => compile(s, "StatsByFile"))
  .then((t) => Bun.write("ts/types/stats.d.ts", t));

fetch(
  "https://raw.githubusercontent.com/lvlvllvlvllvlvl/RePoE/master/RePoE/schema/stat_value_handlers.schema.json"
)
  .then((r) => r.json())
  .then((j) => {
    Bun.write("schema/handlers.schema.json", JSON.stringify(j));
    return j;
  })
  .then((s) => compile(s as any, "StatHandlers"))
  .then((t) => Bun.write("ts/types/handlers.d.ts", t));
