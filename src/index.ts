import trie from "../data/trie.json";
import tokensJson from "../data/tokens.json";
import handlersJson from "../data/handlers.json";
import type { Token } from "./types/stats";
import type { IntHandler, StatHandlers } from "./types/handlers";
import type { ParseResult, Trie } from "./types";

const tokens = tokensJson as Record<string, Token[]>;
const handlers = handlersJson as StatHandlers;

export function parse(
  mod: string,
  log: (...args: unknown[]) => void = () => {}
): ParseResult[] {
  const words = tokenise(mod);
  log(words);
  const results: ParseResult[] = [];
  let count = 0;
  while (count < words.length) {
    const found = searchTrie(words[0], words, 1, trie as Trie, log);
    if (!found.text || found.count === 0) {
      count = count + 1;
      continue;
    }
    const result: ParseResult = {
      text: found.text,
      stats: found.stats
        ? Object.entries(found.stats).map(([id, parsedValue]) => ({
            id,
            index: -1,
            parsedValue,
            baseValue: parsedValue,
          }))
        : [],
    };
    results.push(result);
    tokens[found.text].forEach((t) => {
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
      }
    });

    count += found.count;
    log(found);
  }

  log(results);
  return results;
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

interface IntermediateResult {
  text: string | undefined;
  count: number;
  values: number[];
  stats?: { [stat: string]: number };
}

function searchTrie(
  word: string,
  words: string[],
  i: number,
  node: Trie,
  log: (...args: any[]) => void
): IntermediateResult {
  log(
    i,
    word,
    Object.keys(node.childMap || {}),
    Boolean(node.numChild),
    Boolean(node.anyChild),
    node.terminal
  );

  if (i > words.length) {
    log(node);
    return {
      text: node.terminal,
      count: 0,
      values: node.statValue ? [node.statValue] : [],
    };
  }

  const results: IntermediateResult[] = [];

  if (node.childMap && word in node?.childMap) {
    const result = searchTrie(words[i], words, i + 1, node.childMap[word], log);
    if (result.text) {
      if (node.statValue) result.values.unshift(node.statValue);
      result.count++;
      results.push(result);
    }
  }
  if (node.numChild) {
    const match = /^(([+-])?\d*\.?\d+)(\D.*)?$/.exec(word);
    log(match);
    if (match) {
      const [, num, prefix, suffix] = match;
      if (
        prefix &&
        node.childMap &&
        "+" in node.childMap &&
        node.childMap["+"].numChild
      ) {
        const result = searchTrie(
          word.substring(1),
          words,
          i,
          node.childMap["+"],
          log
        );
        if (result.text) results.push(result);
      }
      if (suffix) {
        const result = searchTrie(suffix, words, i, node.numChild, log);
        if (result.text) {
          result.values.unshift(parseFloat(num));
          results.push(result);
        }
      } else {
        const result = searchTrie(words[i], words, i + 1, node.numChild, log);
        if (result.text) {
          result.values.unshift(parseFloat(num));
          result.count++;
          results.push(result);
        }
      }
    }
  }

  if (node.statChild) {
    const nested = searchTrie(word, words, i, trie as Trie, log);
    log("nested stat", nested);
    const result = searchTrie(
      words[i + nested.count - 1],
      words,
      i + nested.count,
      node.statChild,
      log
    );
    if (result.text) {
      result.count += nested.count;
      if (!result.stats) result.stats = {};
      if (node.statValue) result.stats[node.statValue] = 1;
      results.push(result);
    }
  }
  if (node.anyChild) {
    const result = searchTrie(words[i], words, i + 1, node.anyChild, log);
    if (result) {
      result.count++;
      results.push(result);
    }
  }

  let result = {
    text: node.terminal,
    count: 0,
    values: node.statValue ? [node.statValue] : [],
  };
  for (const r of results) {
    if (r.count > result.count) result = r;
  }

  return result;
}
