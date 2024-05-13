import { parse } from "poestat";
import "./App.css";
import { Fragment, useEffect, useState } from "react";

function App() {
  const [text, setText] = useState("");
  const [parsed, setParsed] = useState();

  useEffect(() => {parse(text).then(setParsed)}, [text]);

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
          parsed.map((s) => (
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
