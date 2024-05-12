import handlersJson from "../data/handlers.json";
import tokensJson from "../data/tokens.json";
import implied from "../data/implied.json";
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
    processResult(found, result);

    index += found.count;
    log(found);
  }

  log(results);
  return results;
}

function processResult(input: IntermediateResult, output: ParseResult) {
  processTokens(input, output);
  processImplications(input, output);
}

function processTokens(input: IntermediateResult, output: ParseResult) {
  tokens[input.text!].forEach((t) => {
    if (t.type === "number") {
      const parsedValue = input.values.shift();
      if (parsedValue === undefined) {
        throw new Error("Missing value for stat" + t.stat);
      }
      const baseValue = (t.stat_value_handlers || []).reduceRight(
        (n, h) => reverseHandler(n, handlers[h] as IntHandler),
        parsedValue
      );
      output.stats.push({
        id: t.stat,
        index: t.index,
        parsedValue,
        baseValue,
      });
    } else if (t.type === "enum") {
      const parsedValue = input.values.shift();
      if (parsedValue === undefined) {
        throw new Error("Missing value for stat" + t.stat);
      }
      output.stats.push({
        id: t.stat,
        index: t.index,
        parsedValue,
        baseValue: parsedValue,
      });
    } else if (t.type === "nested") {
      const nested = input.nested;
      if (!nested?.text) {
        throw new Error("Missing nested stat for " + input.text);
      }
      output.text = output.text.replace("{0}", nested.text);
      processResult(nested, output);
    }
  });
}

function processImplications(input: IntermediateResult, output: ParseResult) {
  const stats = implied[input.text as keyof typeof implied];
  if (!stats) return;
  for (const [id, value] of Object.entries(stats)) {
    if (value) {
      output.stats.push({
        id,
        index: -1,
        parsedValue: value,
        baseValue: value,
      });
    }
  }
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
