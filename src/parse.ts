import { Parser } from "./parser";
import { TrieBuilder } from "./trie-builder.js";
import type { StatsByFile } from "./types/stats.js";
import type { StatHandlers } from "./types/handlers.js";

/**
 * Codes taken from the 'preferred language' setting at https://www.pathofexile.com/my-account/preferences
 */
export const LANGS = {
  English: "en-US",
  French: "fr-FR",
  German: "de-DE",
  Japanese: "ja-JP",
  Korean: "ko-KR",
  Portuguese: "pt-BR",
  Russian: "ru-RU",
  Spanish: "es-ES",
  Thai: "th-TH",
  /**
   * Chinese not present in the settings, should this be zh-Hant?
   */
  "Traditional Chinese": "zh-TW",
} as const;

export const forLang = (lang: keyof typeof LANGS = "English") =>
  createParser(
    `https://lvlvllvlvllvlvl.github.io/RePoE${lang === "English" ? "" : "/" + lang}/stats_by_file.min.json`,
    `https://lvlvllvlvllvlvl.github.io/RePoE${lang === "English" ? "" : "/" + lang}/stat_value_handlers.min.json`
  );

export const createParser = async (
  statsUrl = "https://lvlvllvlvllvlvl.github.io/RePoE/stats_by_file.min.json",
  handlersUrl = "https://lvlvllvlvllvlvl.github.io/RePoE/stat_value_handlers.min.json"
) => {
  const stats: StatsByFile = await fetch(statsUrl)
    .then((r) => r.text())
    .then((t) => JSON.parse(t.normalize()));

  const handlers: StatHandlers = await fetch(handlersUrl)
    .then((r) => r.text())
    .then((t) => JSON.parse(t.normalize()));

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

export const parse = async (text: string) => {
  for (const lang of Object.keys(LANGS)) {
    const result = (await forLang(lang as any)).parse(text);
    if (result.length) return result;
  }
};
