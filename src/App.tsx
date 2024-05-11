import { parse } from "poestat";
import "./App.css";
import { Fragment, useState } from "react";

function App() {
  const [text, setText] = useState("");

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
          parse(text, console.debug).map((s) => (
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
