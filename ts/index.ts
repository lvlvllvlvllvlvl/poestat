export { Parser } from "./parser";
export * from "./parse";

export type * from "./types/handlers";
export type * from "./types/stats";
export interface Trie {
  childMap?: { [token: string]: Trie };
  numChild?: Trie;
  anyChild?: Trie;
  statChild?: Trie;
  statId?: string;
  statValue?: number;
  terminal?: string;
}

export interface ParseResult {
  text: string;
  stats: ParsedStat[];
}

export interface IntermediateResult {
  text: string | undefined;
  count: number;
  values: number[];
  nested?: IntermediateResult;
}

export interface ParsedStat {
  /**
   * The id of the row in Stats.dat
   */
  id: string;
  /**
   * The template variable in the stat text (e.g. {0})
   */
  index: number;
  /**
   * The value as read from the mod that was parsed
   */
  parsedValue: number;
  /**
   * Approximate value of the raw value of this stat as per the game files
   * (for instance if the game stores the stat as ms but displays it as seconds,
   * this will be the ms value).
   */
  baseValue: number;
}
