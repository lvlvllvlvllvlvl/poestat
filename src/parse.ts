import handlersJson from "../data/handlers.json";
import tokensJson from "../data/tokens.json";
import trie from "../data/trie.json";
import { searchTrie } from "./searchTrie";
import type { IntermediateResult, ParseResult } from "./types";
import type { IntHandler, StatHandlers } from "./types/handlers";
import type { Token } from "./types/stats";

const tokens = tokensJson as Record<string, Token[]>;
const handlers = handlersJson as StatHandlers;

export function parse(
  mod: string,
  log: (...args: unknown[]) => void = () => {}
): ParseResult[] {
  const words = tokenise(mod);
  log(mod, words);
  const results: ParseResult[] = [];
  let index = 0;
  while (index < words.length) {
    const found = searchTrie(words[index], words, index + 1, trie, trie, log);
    if (!found.text || found.count === 0) {
      index = index + 1;
      continue;
    }
    const result: ParseResult = {
      text: found.text,
      stats: [],
    };
    results.push(result);
    processTokens(found, result);

    index += found.count;
    log(found);
  }

  log(results);
  return results;
}

function processTokens(found: IntermediateResult, result: ParseResult) {
  tokens[found.text!].forEach((t) => {
    if (t.type === "number") {
      const parsedValue = found.values.shift();
      if (parsedValue === undefined) {
        throw new Error("Missing value for stat" + t.stat);
      }
      const baseValue = (t.stat_value_handlers || []).reduceRight(
        (n, h) => reverseHandler(n, handlers[h] as IntHandler),
        parsedValue
      );
      result.stats.push({
        id: t.stat,
        index: t.index,
        parsedValue,
        baseValue,
      });
    } else if (t.type === "enum") {
      const parsedValue = found.values.shift();
      if (parsedValue === undefined) {
        throw new Error("Missing value for stat" + t.stat);
      }
      result.stats.push({
        id: t.stat,
        index: t.index,
        parsedValue,
        baseValue: parsedValue,
      });
    } else if (t.type === "nested") {
      const nested = found.nested;
      if (!nested?.text) {
        throw new Error("Missing nested stat for " + found.text);
      }
      result.text = result.text.replace("{0}", nested.text);
      processTokens(nested, result);
    }
  });
}

function reverseHandler(
  n: number,
  { addend = 0, multiplier = 1, divisor = 1 }: IntHandler
): number {
  return ((n - addend) * divisor) / multiplier;
}

function tokenise(str: string) {
  return str.trim().toLocaleLowerCase("en-US").normalize().split(/\s+/);
}
