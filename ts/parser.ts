import type { IntermediateResult, ParseResult, Trie } from "./";
import type { IntHandler, StatHandlers } from "./types/handlers";
import type { Token } from "./types/stats";

export class Parser {
  constructor(
    readonly trie: Trie,
    readonly tokens: Record<string, Token[]>,
    readonly implied: Record<string, Record<string, number>>,
    readonly handlers: StatHandlers
  ) {
    for (const h of Object.values(handlers)) {
      if (!h.type) {
        console.log(h);
        throw new Error();
      }
    }
  }

  parse = (
    mod: string,
    log: (...args: unknown[]) => void = () => {}
  ): ParseResult[] => {
    const words = this.tokenise(mod);
    log(mod, words);
    const results: ParseResult[] = [];
    let index = 0;
    while (index < words.length) {
      const found = this.searchTrie(
        words[index],
        words,
        index + 1,
        this.trie,
        this.trie,
        log
      );
      if (!found.text || found.count === 0) {
        index = index + 1;
        continue;
      }
      const result: ParseResult = {
        text: found.text,
        stats: [],
      };
      results.push(result);
      this.processResult(found, result);

      index += found.count;
      log(found);
    }

    log(results);
    return results;
  };

  processResult = (input: IntermediateResult, output: ParseResult) => {
    this.processTokens(input, output);
    this.processImplications(input, output);
  };

  processTokens = (input: IntermediateResult, output: ParseResult) => {
    this.tokens[input.text!].forEach((t) => {
      if (t.type === "number") {
        const parsedValue = input.values.shift();
        if (parsedValue === undefined) {
          throw new Error("Missing value for stat" + t.stat);
        }
        const baseValue = (t.stat_value_handlers || []).reduceRight(
          (n, h) => this.reverseHandler(n, this.handlers[h] as IntHandler),
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
        this.processResult(nested, output);
      }
    });
  };

  processImplications(input: IntermediateResult, output: ParseResult) {
    const stats = this.implied[input.text!];
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

  reverseHandler(
    n: number,
    { addend = 0, multiplier = 1, divisor = 1 }: IntHandler
  ): number {
    return ((n - addend) * divisor) / multiplier;
  }

  tokenise(str: string) {
    return str.trim().toLocaleLowerCase("en-US").normalize().split(/\s+/);
  }

  searchTrie(
    word: string,
    words: string[],
    i: number,
    node: Trie,
    root: Trie,
    log: (...args: any[]) => void,
    canRecurse = true
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
      const result = this.searchTrie(
        words[i],
        words,
        i + 1,
        node.childMap[word],
        root,
        log
      );
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
          const result = this.searchTrie(
            word.substring(1),
            words,
            i,
            node.childMap["+"],
            root,
            log
          );
          if (result.text) results.push(result);
        }
        if (suffix) {
          const result = this.searchTrie(
            suffix,
            words,
            i,
            node.numChild,
            root,
            log
          );
          if (result.text) {
            result.values.unshift(parseFloat(num));
            results.push(result);
          }
        } else {
          const result = this.searchTrie(
            words[i],
            words,
            i + 1,
            node.numChild,
            root,
            log
          );
          if (result.text) {
            result.values.unshift(parseFloat(num));
            result.count++;
            results.push(result);
          }
        }
      }
    }

    if (node.statChild && canRecurse) {
      const nested = this.searchTrie(word, words, i, root, root, log, false);
      log("nested stat", nested);
      if (nested.text) {
        const result = this.searchTrie(
          words[i + nested.count - 1],
          words,
          i + nested.count,
          node.statChild,
          root,
          log
        );
        if (result.text) {
          result.count += nested.count;
          result.nested = nested;
          results.push(result);
        }
      }
    }
    if (node.anyChild) {
      const result = this.searchTrie(
        words[i],
        words,
        i + 1,
        node.anyChild,
        root,
        log
      );
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
}
