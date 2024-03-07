import { AllKeys } from "@/lib/config";
import { invoke } from "@tauri-apps/api";
import { BaseDirectory, readTextFile, writeFile } from "@tauri-apps/api/fs";
import { useEffect, useState } from "react";
import { Button } from "./ui/button";

export const DISPLAY_VALUE_FILE_PATH = ".config/KeyTutorAgenDV.txt";

interface Keymap {
  mode: string;
  name: string;
  remap: { [key: string]: string | { set_mode: string } };
}

interface Modmap {
  mode: string;
  name: string;
  remap: { [key: string]: string | { set_mode: string } };
}

interface Maps {
  modmap: Modmap[];
  keymap: Keymap[];
}

export async function loadDisplayValues(): Promise<{ [key: string]: string }> {
  let loadedValues = await (async () => {
    try {
      return JSON.parse(
        await readTextFile(DISPLAY_VALUE_FILE_PATH, {
          dir: BaseDirectory.Home,
        }),
      );
    } catch (err) {
      return {};
    }
  })();

  let defaultvalues = {};
  Object.entries(AllKeys).forEach(([key, { displayValue }]) => {
    defaultvalues = {
      ...defaultvalues,
      [key + "-" + "default"]: displayValue,
    };
  });

  return { ...defaultvalues, ...loadedValues };
}

function saveDisplayValues(displayValues: {
  [key: string]: string;
}): Promise<void> {
  return writeFile(DISPLAY_VALUE_FILE_PATH, JSON.stringify(displayValues), {
    dir: BaseDirectory.Home,
  });
}

const DisplayValueSetter = ({ configStr }: { configStr: string }) => {
  const [config, setConfig] = useState<Maps>();
  const [displayValues, setDisplayValues] = useState<{
    [key: string]: string;
  }>();

  useEffect(() => {
    loadDisplayValues().then((data) => {
      setDisplayValues(data);
    });
  }, []);

  useEffect(() => {
    invoke<Maps>("get_config", { configStr })
      .then((data) => {
        setConfig(data);
      })
      .catch((_) => {});
  }, [configStr]);

  return (
    <div className="space-y-10">
      {displayValues &&
        config?.keymap?.map(({ remap, name, mode }) => {
          return (
            <div
              key={name}
              className="border-2 border-white  h-[50rem] overflow-scroll text-white space-y-4 relative"
            >
              <div className="sticky top-0 bg-slate-900 p-4 border-b-2 border-white">
                <p className="font-semibold">name: {name}</p>
                <p className="font-semibold">mode: {mode}</p>
              </div>
              <div className="space-y-8 p-4">
                {Object.entries(remap).map(([key, value]) => {
                  return (
                    <div key={key} className="space-y-2">
                      <p>
                        Display Value of `{key}` with value `{value.toString()}`
                        in mode `{mode}`:
                      </p>
                      <input
                        value={displayValues[key + "-" + mode]}
                        name={key}
                        onChange={(e) =>
                          setDisplayValues((prev) => ({
                            ...prev,
                            [key + "-" + mode]: e.target.value,
                          }))
                        }
                        className="px-4 py-2 bg-white/20"
                      />
                    </div>
                  );
                })}
              </div>
            </div>
          );
        })}
      <Button
        variant={"secondary"}
        onClick={() => displayValues && saveDisplayValues(displayValues)}
      >
        Save
      </Button>
    </div>
  );
};

export default DisplayValueSetter;
