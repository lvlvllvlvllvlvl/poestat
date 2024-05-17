import { TrieBuilder } from "../trie-builder";
import type { StatHandlers } from "../types/handlers";
import type { StatsByFile } from "../types/stats";

const stats: StatsByFile = await fetch(
  "https://lvlvllvlvllvlvl.github.io/RePoE/stats_by_file.min.json"
).then((r) => r.json());

const handlers: StatHandlers = await fetch(
  "https://lvlvllvlvllvlvl.github.io/RePoE/stat_value_handlers.min.json"
).then((r) => r.json());

const trie = new TrieBuilder(stats, handlers).build();

Bun.write("data/trie.json", JSON.stringify(trie, undefined, 2));
Bun.write("data/stats.json", JSON.stringify(stats, undefined, 2));
Bun.write("data/handlers.json", JSON.stringify(handlers, undefined, 2));
Bun.write(
  "data/tokens.json",
  JSON.stringify(
    Object.fromEntries(Object.entries(stats).map(([k, v]) => [k, v.tokens])),
    undefined,
    2
  )
);
Bun.write(
  "data/implied.json",
  JSON.stringify(
    Object.fromEntries(
      Object.entries(stats)
        .filter(([, v]) => v.implied_stats)
        .map(([k, v]) => [k, v.implied_stats])
    ),
    undefined,
    2
  )
);
