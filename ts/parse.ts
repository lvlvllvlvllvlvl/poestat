import { Parser } from "./parser";
import { TrieBuilder } from "./trie-builder.js";
import type { StatsByFile } from "./types/stats.js";
import type { StatHandlers } from "./types/handlers.js";

/**
 * Codes taken from the 'preferred language' setting at https://www.pathofexile.com/my-account/preferences
 */
export const LANGS = [
  "English",
  "French",
  "German",
  "Japanese",
  "Korean",
  "Portuguese",
  "Russian",
  "Spanish",
  "Thai",
  "Traditional Chinese",
] as const;

export type Language = (typeof LANGS)[number];

export const getParser = (lang: Language = "English") =>
  fromUrls(
    `https://lvlvllvlvllvlvl.github.io/RePoE${lang === "English" ? "" : "/" + lang}/stats_by_file.min.json`,
    `https://lvlvllvlvllvlvl.github.io/RePoE${lang === "English" ? "" : "/" + lang}/stat_value_handlers.min.json`
  );

export const fromUrls = async (
  ...urls: [statsUrl: string, handlersUrl: string]
) => {
  const json = (await Promise.all(
    urls.map((u) => fetch(u).then((r) => r.text()))
  )) as [string, string];
  return fromJson(...json);
};

export const fromJson = async (statsJson: string, handlerJson: string) => {
  const stats: StatsByFile = JSON.parse(statsJson.normalize());

  const handlers: StatHandlers = JSON.parse(handlerJson.normalize());

  const tokens = Object.fromEntries(
    Object.entries(stats).map(([k, v]) => [k, v.tokens])
  );

  const implied = Object.fromEntries(
    Object.entries(stats)
      .filter(([, v]) => v.implied_stats)
      .map(([k, v]) => [k, v.implied_stats!])
  );

  const trie = new TrieBuilder(stats, handlers).build();

  return new Parser(trie, tokens, implied, handlers);
};

const tmp: { [lang in Language]?: Promise<Parser> } = {};

export const parse = async (text: string, languages = LANGS) => {
  for (const lang of languages) {
    const parser = await (tmp[lang] = tmp[lang] || getParser(lang));
    const result = parser.parse(text);
    if (result.length) return result;
  }
};
