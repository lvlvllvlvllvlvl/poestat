import type { StatHandlers } from "./types/handlers";
import type { Trie } from "./index";
import type { Stat, StatsByFile, Token } from "./types/stats";

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
          node.terminal = stat_text || undefined;
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
          if (!node.child_map) {
            node.child_map = {};
          }
          if (!(word in node.child_map)) {
            node.child_map[word] = {};
          }
          node = node.child_map[word];
        }
        return [node];
      case "number":
        if (!node.num_child) {
          node.num_child = {};
        }
        return [node.num_child];
      case "enum":
        const handler = this.handlers[token.stat_value_handler];
        if (handler.type !== "relational" || !handler.values) {
          throw new Error(`handler ${token.stat_value_handler} has no values`);
        }
        const nodes: Trie[] = [];
        for (const [k, v] of Object.entries(handler.values)) {
          const added = this.addToTrie(node, { type: "literal", value: v });
          added.forEach((a) => (a.stat_value = parseInt(k)));
          nodes.push(...added);
        }
        return nodes;
      case "unknown":
        if (!node.any_child) {
          node.any_child = {};
        }
        return [node.any_child];
      case "nested":
        if (!node.stat_child) {
          node.stat_child = { stat_id: token.added_stat };
        }
        return [node.stat_child];
      default:
        throw Error("Unknown token type: " + (token as any).type);
    }
  };
}
