import { useEffect, useState } from "react";
import { Command } from "@tauri-apps/api/shell";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { emit, listen } from "@tauri-apps/api/event";
import reactLogo from "./assets/react.svg";
import "./app.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [filePath, setFilePath] = useState("");

  useEffect(() => {
    let unlisten: any;

    async function f() {
      unlisten = await listen("back-to-front", (event) => {
        console.log(`后端发给前端的消息： ${event.payload} ${new Date()}`);
      });
    }
    f();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  function runWithError() {
    for (let arg of [1, 2]) {
      invoke("command_with_error", { arg })
        .then((message) => {
          console.log("command_with_error=====", message);
        })
        .catch((message: string) => {
          console.error("command_with_error=====", message);
        });
    }
  }

  async function greet() {
    setGreetMsg(await invoke("greet", { name }));
  }

  async function runCommand() {
    // Command.create('list files', []);
    // const output = await Command.create('run-echo', 'message').execute();
    // console.log(output);

    const output = await Command.create("run-list", ["-la"]);
    console.log(output);
    console.log(JSON.stringify(output));

    const git = await Command.create("run-git-commit", [
      "commit",
      "-m",
      "the commit message",
    ]);
    // console.log('git====', git);

    // await new Command('npm', ['-v']).spawn();

    // const npm =   await new Command('npm', ['-v']).spawn();
    //   console.log('npm====', npm);
  }

  function runCustomCommand() {
    invoke("my_custom_command");
  }

  async function command2() {
    const result = await invoke("command_with_message", {
      message: "some message",
    });
    console.log("result:", result);
  }

  async function command_obj() {
    const obj = await invoke("command_with_object", {
      message: { name: "kenny", age: 40 },
    });
    console.log("obj====", obj);
  }

  async function async_command() {
    const result = await invoke("async_command", { arg: 32 });
    console.log("result====", result);
  }

  async function openDialog() {
    const files = await open();
    if (Array.isArray(files)) {
      setFilePath(files[0]);
    } else {
      setFilePath(files as string);
    }
    console.log("files====", files);
  }

  function emitMessage() {
    emit("front-to-back", "我是从前端传过来的值");
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <div className="row">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            greet();
          }}
        >
          <input
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          <button type="submit">Greet</button>
        </form>
      </div>
      <p>{greetMsg}</p>
      <div className="row">
        <button onClick={runCommand}>output</button>
        <button onClick={runCustomCommand}>runCustomCommand</button>
        <button onClick={command2}>command2</button>
      </div>
      <div className="row">
        <button onClick={command_obj}>command_obj</button>
        <button onClick={async_command}>async_command</button>
        <button onClick={runWithError}>runWithError</button>
      </div>
      <p>{filePath}</p>
      <div className="row">
        <button onClick={openDialog}>openDialog</button>
        <button onClick={emitMessage}>向Rust发消息</button>
      </div>
    </div>
  );
}

export default App;
