import Keyboard, { KeyboardProps } from "@/components/Keyboard";
import { appWindow } from "@tauri-apps/api/window";
import { useEffect, useState } from "react";

export default function App() {
  const [modeChangedPayload, setModeChangedPayload] = useState<KeyboardProps>();

  useEffect(() => {
    appWindow.hide();
    const unlisten = appWindow.listen<KeyboardProps>(
      "mode_changed",
      (event) => {
        if (event.payload.mode != "default") {
          appWindow.show();
        } else {
          appWindow.hide();
        }
        setModeChangedPayload(event.payload);
      },
    );

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return <Keyboard {...modeChangedPayload} />;
}
