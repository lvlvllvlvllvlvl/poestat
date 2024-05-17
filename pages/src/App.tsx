import { ParseResult, parse, wasm } from "poestat";
import { Fragment, useEffect, useState } from "react";
import "./App.css";

const hello = await wasm;
const json = (await Promise.all(
  [
    `https://lvlvllvlvllvlvl.github.io/RePoE/stats_by_file.min.json`,
    `https://lvlvllvlvllvlvl.github.io/RePoE/stat_value_handlers.min.json`,
  ].map((u) => fetch(u).then((r) => r.text()))
)) as [string, string];

function App() {
  const [text, setText] = useState("");
  const [parsed, setParsed] = useState<ParseResult[]>();

  useEffect(() => {
    parse(text).then(setParsed);
  }, [text]);

  return (
    <>
      <h1>poe stat parser</h1>
      {hello(...json)}
      <div className="card">
        <textarea
          value={text}
          onChange={(e) => setText(e.currentTarget.value)}
          placeholder="paste stat text here"
        />
        {text &&
          parsed?.map((s) => (
            <Fragment key={s.text}>
              <h3>{s.text}</h3>
              {s.stats.map((stat) => (
                <p key={stat.id}>
                  {stat.id}: {stat.baseValue}
                </p>
              ))}
            </Fragment>
          ))}
      </div>
    </>
  );
}

export default App;
