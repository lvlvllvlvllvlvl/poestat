import type { StatHandlers } from "./types/handlers.js";
import type { Trie } from "./types/index.js";
import type { Stat, StatsByFile, Token } from "./types/stats.js";

export class TrieBuilder {
  trie: Trie = {};
  constructor(
    readonly stats: StatsByFile,
    readonly handlers: StatHandlers
  ) {}

  tokenise = (str: string) => {
    return str.trim().toLocaleLowerCase("en-US").normalize().split(/\s+/);
  };

  build = () => {
    for (const [stat_text, { tokens }] of Object.entries(this.stats)) {
      let nodes = [this.trie];
      for (const token of tokens) {
        nodes = nodes.flatMap((node) => this.addToTrie(node, token));
      }
      for (const node of nodes) {
        if (node.terminal) {
          const existing = this.stats[node.terminal];
          const newNode = this.stats[stat_text];
          if (!this.isCompatible(existing, newNode)) {
            // console.warn("Mismatched terminal node", existing, newNode);
          }
        } else {
          node.terminal = stat_text;
        }
      }
    }
    return this.trie;
  };

  isCompatible = (existing: Stat, other: Stat) => {
    if (existing.tokens.length !== other.tokens.length) {
      return false;
    }
    for (let i = 0; i < other.tokens.length; i++) {
      if (existing.tokens[i].type !== other.tokens[i].type) {
        return false;
      }
    }
    return true;
  };

  addToTrie = (node: Trie, token: Token): Trie[] => {
    switch (token.type) {
      case "literal":
        for (const word of this.tokenise(token.value)) {
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
        const handler = this.handlers[token.stat_value_handler];
        if (handler.type !== "relational" || !handler.values) {
          throw new Error(`handler ${token.stat_value_handler} has no values`);
        }
        const nodes: Trie[] = [];
        for (const [k, v] of Object.entries(handler.values)) {
          const added = this.addToTrie(node, { type: "literal", value: v });
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
  };
}
