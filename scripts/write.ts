import { trie, stats, handlers } from "./create-trie.js";

Bun.write("data/trie.json", JSON.stringify(trie, undefined, 2));
Bun.write("data/stats.json", JSON.stringify(stats, undefined, 2));
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
Bun.write("data/handlers.json", JSON.stringify(handlers, undefined, 2));
