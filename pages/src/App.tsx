import { ParseResult, parse } from "poestat";
import { Fragment, useEffect, useState } from "react";
import "./App.css";

function App() {
  const [text, setText] = useState("");
  const [parsed, setParsed] = useState<ParseResult[]>();

  useEffect(() => {
    parse(text).then(setParsed);
  }, [text]);

  return (
    <>
      <h1>poe stat parser</h1>
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
