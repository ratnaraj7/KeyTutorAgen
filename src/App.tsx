import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Button } from "./components/ui/button";
import DisplayValueSetter from "@/components/DisplayValueSetter";
import { open } from "@tauri-apps/api/dialog";
import { readTextFile } from "@tauri-apps/api/fs";

function App() {
  const [configStr, setConfigStr] = useState<string>();
  const [configFilePath, setConfigFilePath] = useState<string>();
  const [outputFilePath, setOutputFilePath] = useState<string>();

  async function getConfig() {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Config File",
          extensions: ["yml"],
        },
      ],
    });

    if (typeof selected === "string") {
      setConfigFilePath(selected);
      const contents = await readTextFile(selected);
      setConfigStr(contents);
    } else {
    }
  }

  async function getOutputFilePath() {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Output File",
          extensions: ["log"],
        },
      ],
    });

    if (typeof selected === "string") {
      setOutputFilePath(selected);
    } else {
    }
  }

  //async function checkIfPresent() {}
  //async function checkIfVersionIsSupported() {}
  //async function getMaps() {}
  //async function listenMode() {}

  function startTheServer() {
    if (outputFilePath && configStr) {
      invoke("start", { outputFilePath, configStr: configStr }).catch((err) => {
        console.log(err);
      });
    }
  }

  return (
    <div className="bg-slate-900 p-4 min-h-[100vh] w-full space-y-4">
      {
        <div className="space-y-4">
          <div className="flex gap-x-2 items-center">
            <Button onClick={() => getConfig()} variant={"secondary"}>
              {!configStr ? "Load config file" : "Change config file"}
            </Button>
            <div>
              <p className="font-semibold text-white">{configFilePath}</p>
            </div>
          </div>

          <div className="flex gap-x-2 items-center">
            <Button onClick={() => getOutputFilePath()} variant={"secondary"}>
              {!outputFilePath ? "Load output file" : "Change output file"}
            </Button>
            <div>
              <p className="font-semibold text-white">{outputFilePath}</p>
            </div>
          </div>
        </div>
      }

      <Button
        disabled={!configStr || !outputFilePath}
        variant={"outline"}
        onClick={startTheServer}
      >
        Start
      </Button>

      {configStr && (
        <>
          <pre className="text-white border-2 h-[50rem] overflow-scroll p-4">
            {"#your config file\n\n"}
            {configStr}
          </pre>
          <DisplayValueSetter configStr={configStr} />
        </>
      )}
    </div>
  );
}

export default App;
