import type { IntermediateResult, Trie } from "./types";

export function searchTrie(
  word: string,
  words: string[],
  i: number,
  node: Trie,
  root: Trie,
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
    const result = searchTrie(
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
        const result = searchTrie(
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
        const result = searchTrie(suffix, words, i, node.numChild, root, log);
        if (result.text) {
          result.values.unshift(parseFloat(num));
          results.push(result);
        }
      } else {
        const result = searchTrie(
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

  if (node.statChild) {
    const nested = searchTrie(word, words, i, root, root, log);
    log("nested stat", nested);
    const result = searchTrie(
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
  if (node.anyChild) {
    const result = searchTrie(words[i], words, i + 1, node.anyChild, root, log);
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
