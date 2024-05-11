import type { Trie } from "../src/index.js";
import type { StatHandlers } from "../src/types/handlers.js";
import type { Stat, StatsByFile, Token } from "../src/types/stats.js";

export const trie: Trie = {};

export const stats: StatsByFile = await fetch(
  "https://lvlvllvlvllvlvl.github.io/RePoE/stats_by_file.min.json"
).then((r) => r.json());

export const handlers: StatHandlers = await fetch(
  "https://lvlvllvlvllvlvl.github.io/RePoE/stat_value_handlers.min.json"
).then((r) => r.json());

export function tokenise(str: string) {
  return str.trim().toLocaleLowerCase("en-US").normalize().split(/\s+/);
}

for (const [stat_text, { tokens }] of Object.entries(stats)) {
  let nodes = [trie];
  for (const token of tokens) {
    nodes = nodes.flatMap((node) => addToTrie(node, token));
  }
  for (const node of nodes) {
    if (node.terminal) {
      const existing = stats[node.terminal];
      const newNode = stats[stat_text];
      if (!isCompatible(existing, newNode)) {
        console.error("Mismatched terminal node", existing, newNode);
        throw new Error("Mismatched terminal node.");
      }
    } else {
      node.terminal = stat_text;
    }
  }
}

export function isCompatible(existing: Stat, other: Stat) {
  if (existing.tokens.length !== other.tokens.length) {
    return false;
  }
  for (let i = 0; i < other.tokens.length; i++) {
    if (existing.tokens[i].type !== other.tokens[i].type) {
      return false;
    }
  }
  return true;
}

export function addToTrie(node: Trie, token: Token): Trie[] {
  switch (token.type) {
    case "literal":
      for (const word of tokenise(token.value)) {
        if (!node.childMap) {
          node.childMap = {};
        }
        if (!(word in node.childMap)) {
          node.childMap[word] = {};
        }
        node = node.childMap[word];
      }
      return [node];
    case "number":
      if (!node.numChild) {
        node.numChild = {};
      }
      return [node.numChild];
    case "enum":
      const handler = handlers[token.stat_value_handler];
      if (handler.type !== "relational" || !handler.values) {
        throw new Error(`handler ${token.stat_value_handler} has no values`);
      }
      const nodes: Trie[] = [];
      for (const [k, v] of Object.entries(handler.values)) {
        const added = addToTrie(node, { type: "literal", value: v });
        added.forEach((a) => (a.statValue = parseInt(k)));
        nodes.push(...added);
      }
      return nodes;
    case "unknown":
      if (!node.anyChild) {
        node.anyChild = {};
      }
      return [node.anyChild];
    case "nested":
      if (!node.statChild) {
        node.statChild = { statId: token.added_stat };
      }
      return [node.statChild];
    default:
      throw Error("Unknown token type: " + (token as any).type);
  }
}
