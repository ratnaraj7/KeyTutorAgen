// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use tauri::Manager;

mod fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SetMode {
    set_mode: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum RemapValue {
    Key(String),
    SetMode(SetMode),
}

type Remap = HashMap<String, RemapValue>;

#[derive(Debug, Serialize, Deserialize)]
struct Keymap {
    name: String,
    mode: Option<String>,
    remap: Remap,
}

#[derive(Debug, Serialize, Deserialize)]
struct Modmap {
    name: String,
    remap: Remap,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    keymap: Vec<Keymap>,
    modmap: Vec<Modmap>,
}

fn parse_config(config_str: &str) -> Result<Config, serde_yaml::Error> {
    serde_yaml::from_str(config_str)
}

#[tauri::command]
fn get_config(config_str: &str) -> Config {
    println!("hello");
    parse_config(config_str).unwrap()
}

#[derive(Clone, Serialize, Deserialize)]
struct ModeChangePayload {
    mode: String,
    remapes: Remap,
}

#[tauri::command]
fn start(app: tauri::AppHandle, output_file_path: &str, config_str: &str) {
    dbg!("start it");
    let config: Config = parse_config(config_str).unwrap();
    let cmd = format!("tail -1 {} | awk '/mode:/ {{print $2}}'", output_file_path);

    let _ = fs::file_changed::run_on_file_change(output_file_path.to_string(), move || {
        let mut mode = String::new();
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd.clone())
            .output()
            .unwrap();
        if output.status.success() {
            mode = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("Mode: {}", mode);
        } else {
            eprintln!(
                "Command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        dbg!(&mode);
        let remapes = config
            .keymap
            .iter()
            .filter(|k_group| {
                if let Some(kmode) = k_group.mode.as_ref() {
                    kmode == &mode
                } else {
                    false
                }
            })
            .fold(HashMap::new(), |mut acc, k_group| {
                k_group.remap.iter().for_each(|(key, value)| {
                    acc.insert(key.clone(), value.clone());
                });
                acc
            });
        let _ = app.emit_all("mode_changed", ModeChangePayload { mode, remapes });
    });
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start, get_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    #[test]
    fn should_parse() {
        let config_str = r#"
modmap:
  - name: mod_def
    remap:
      KEY_CAPSLOCK: KEY_ESC
      KEY_LEFTCTRL: KEY_RIGHTALT
      KEY_RIGHTCTRL: KEY_RIGHTALT
      KEY_LEFTSHIFT: KEY_RIGHTALT
keymap:
  - name: num
    remap:
      KEY_Q: KEY_0
      KEY_W: KEY_1
      KEY_E: KEY_2
      KEY_R: KEY_3
      KEY_T: KEY_4
      KEY_Y: KEY_5
      KEY_U: KEY_6
      KEY_I: KEY_7
      KEY_O: KEY_8
      KEY_P: KEY_9

      #disable others
      #KEY_Q: KEY_RIGHTALT
      #KEY_W: KEY_RIGHTALT
      #KEY_E: KEY_RIGHTALT
      #KEY_R: KEY_RIGHTALT
      #KEY_T: KEY_RIGHTALT
      #KEY_Y: KEY_RIGHTALT
      #KEY_U: KEY_RIGHTALT
      #KEY_I: KEY_RIGHTALT
      #KEY_O: KEY_RIGHTALT
      #KEY_P: KEY_RIGHTALT
      KEY_A: KEY_RIGHTALT
      KEY_S: KEY_RIGHTALT
      KEY_D: KEY_RIGHTALT
      KEY_F: KEY_RIGHTALT
      KEY_G: KEY_RIGHTALT
      KEY_H: KEY_RIGHTALT
      KEY_J: KEY_RIGHTALT
      KEY_K: KEY_RIGHTALT
      KEY_L: KEY_RIGHTALT
      KEY_Z: KEY_RIGHTALT
      KEY_X: KEY_RIGHTALT
      KEY_C: KEY_RIGHTALT
      KEY_V: KEY_RIGHTALT
      KEY_B: KEY_RIGHTALT
      KEY_N: KEY_RIGHTALT
      KEY_M: KEY_RIGHTALT
      KEY_SPACE: KEY_RIGHTALT
    mode: num

  - name: spec
    remap:
      KEY_Q: SHIFT-KEY_0
      KEY_W: SHIFT-KEY_1
      KEY_E: SHIFT-KEY_2
      KEY_R: SHIFT-KEY_3
      KEY_T: SHIFT-KEY_4
      KEY_Y: SHIFT-KEY_5
      KEY_U: SHIFT-KEY_6
      KEY_I: SHIFT-KEY_7
      KEY_O: SHIFT-KEY_8
      KEY_P: SHIFT-KEY_9
      KEY_LEFTBRACE: KEY_MINUS
      KEY_RIGHTBRACE: KEY_EQUAL

      KEY_A: KEY_LEFTBRACE
      KEY_S: KEY_RIGHTBRACE
      KEY_D: KEY_BACKSLASH
      KEY_F: KEY_SEMICOLON
      KEY_G: KEY_APOSTROPHE
      KEY_H: KEY_COMMA
      KEY_J: KEY_DOT
      KEY_K: KEY_SLASH
      KEY_L: SHIFT-KEY_LEFTBRACE
      KEY_SEMICOLON: SHIFT-KEY_MINUS
      KEY_APOSTROPHE: SHIFT-KEY_EQUAL

      KEY_Z: SHIFT-KEY_RIGHTBRACE
      KEY_X: SHIFT-KEY_BACKSLASH
      KEY_C: SHIFT-KEY_SEMICOLON
      KEY_V: SHIFT-KEY_APOSTROPHE
      KEY_B: SHIFT-KEY_COMMA
      KEY_N: SHIFT-KEY_DOT
      KEY_M: SHIFT-KEY_SLASH
      KEY_COMMA: KEY_GRAVE
      #KEY_DOT: SHIFT-KEY_GRAVE
      KEY_SPACE: KEY_TAB

      #disable all other keys
      KEY_0: KEY_RIGHTALT
      KEY_1: KEY_RIGHTALT
      KEY_2: KEY_RIGHTALT
      KEY_3: KEY_RIGHTALT
      KEY_4: KEY_RIGHTALT
      KEY_5: KEY_RIGHTALT
      KEY_6: KEY_RIGHTALT
      KEY_7: KEY_RIGHTALT
      KEY_8: KEY_RIGHTALT
      KEY_9: KEY_RIGHTALT
      KEY_MINUS: KEY_RIGHTALT
      KEY_EQUAL: KEY_RIGHTALT
      KEY_BACKSPACE: KEY_RIGHTALT
      KEY_ENTER: KEY_RIGHTALT
      #KEY_LEFTBRACE: KEY_RIGHTALT
      #KEY_RIGHTBRACE: KEY_RIGHTALT
      KEY_BACKSLASH: KEY_RIGHTALT
      #KEY_SEMICOLON: KEY_RIGHTALT
      #KEY_APOSTROPHE: KEY_RIGHTALT
      #KEY_COMMA: KEY_RIGHTALT
      KEY_GRAVE: KEY_RIGHTALT
      KEY_SLASH: KEY_RIGHTALT
      KEY_LEFT: KEY_RIGHTALT
      KEY_RIGHT: KEY_RIGHTALT
      KEY_UP: KEY_RIGHTALT
      KEY_DOWN: KEY_RIGHTALT
      KEY_INSERT: KEY_RIGHTALT
      KEY_DELETE: KEY_RIGHTALT
      KEY_HOME: KEY_RIGHTALT
      KEY_END: KEY_RIGHTALT
      KEY_PAGEUP: KEY_RIGHTALT
      KEY_PAGEDOWN: KEY_RIGHTALT
      KEY_TAB: KEY_RIGHTALT
    mode: spec

  - name: oth
    remap:
      KEY_I: KEY_INSERT
      KEY_H: KEY_LEFT
      KEY_J: KEY_DOWN
      KEY_K: KEY_UP
      KEY_L: KEY_RIGHT
      KEY_A: KEY_HOME
      KEY_S: KEY_END
      KEY_D: KEY_BACKSPACE
      KEY_C: CTRL-KEY_C
      KEY_E: KEY_ENTER

      #disable others
      KEY_0: KEY_RIGHTALT
      KEY_1: KEY_RIGHTALT
      KEY_2: KEY_RIGHTALT
      KEY_3: KEY_RIGHTALT
      KEY_4: KEY_RIGHTALT
      KEY_5: KEY_RIGHTALT
      KEY_6: KEY_RIGHTALT
      KEY_7: KEY_RIGHTALT
      KEY_8: KEY_RIGHTALT
      KEY_9: KEY_RIGHTALT
      KEY_MINUS: KEY_RIGHTALT
      KEY_EQUAL: KEY_RIGHTALT
      KEY_BACKSPACE: KEY_RIGHTALT
      KEY_ENTER: KEY_RIGHTALT
      KEY_LEFTBRACE: KEY_RIGHTALT
      KEY_RIGHTBRACE: KEY_RIGHTALT
      KEY_BACKSLASH: KEY_RIGHTALT
      KEY_SEMICOLON: KEY_RIGHTALT
      KEY_APOSTROPHE: KEY_RIGHTALT
      KEY_COMMA: KEY_RIGHTALT
      KEY_GRAVE: KEY_RIGHTALT
      KEY_SLASH: KEY_RIGHTALT
      KEY_LEFT: KEY_RIGHTALT
      KEY_RIGHT: KEY_RIGHTALT
      KEY_UP: KEY_RIGHTALT
      KEY_DOWN: KEY_RIGHTALT
      KEY_INSERT: KEY_RIGHTALT
      KEY_DELETE: KEY_RIGHTALT
      KEY_HOME: KEY_RIGHTALT
      KEY_END: KEY_RIGHTALT
      KEY_PAGEUP: KEY_RIGHTALT
      KEY_PAGEDOWN: KEY_RIGHTALT
      KEY_TAB: KEY_RIGHTALT

      KEY_Q: KEY_RIGHTALT
      KEY_W: KEY_RIGHTALT
      #KEY_E: KEY_RIGHTALT
      KEY_R: KEY_RIGHTALT
      KEY_T: KEY_RIGHTALT
      KEY_Y: KEY_RIGHTALT
      KEY_U: KEY_RIGHTALT
      #KEY_I: KEY_RIGHTALT
      KEY_O: KEY_RIGHTALT
      KEY_P: KEY_RIGHTALT
      #KEY_A: KEY_RIGHTALT
      #KEY_S: KEY_RIGHTALT
      #KEY_D: KEY_RIGHTALT
      KEY_F: KEY_RIGHTALT
      KEY_G: KEY_RIGHTALT
      #KEY_H: KEY_RIGHTALT
      #KEY_J: KEY_RIGHTALT
      #KEY_K: KEY_RIGHTALT
      #KEY_L: KEY_RIGHTALT
      KEY_Z: KEY_RIGHTALT
      KEY_X: KEY_RIGHTALT
      #KEY_C: KEY_RIGHTALT
      KEY_V: KEY_RIGHTALT
      KEY_B: KEY_RIGHTALT
      KEY_N: KEY_RIGHTALT
      KEY_M: KEY_RIGHTALT
      KEY_SPACE: KEY_RIGHTALT
    mode: oth

  - name: mode_chooser
    remap:
      KEY_N: { set_mode: num }
      KEY_S: { set_mode: spec }
      KEY_O: { set_mode: oth }
      KEY_M: { set_mode: with-mod }
      KEY_D: { set_mode: default }
    mode: mode_chooser

  - name: with-mod
    remap:
      KEY_A: { set_mode: alt }
      KEY_S: { set_mode: shift }
      KEY_C: { set_mode: ctrl }
    mode: with-mod

  - name: default
    remap:
      #disable all other keys
      KEY_0: KEY_RIGHTALT
      KEY_1: KEY_RIGHTALT
      KEY_2: KEY_RIGHTALT
      KEY_3: KEY_RIGHTALT
      KEY_4: KEY_RIGHTALT
      KEY_5: KEY_RIGHTALT
      KEY_6: KEY_RIGHTALT
      KEY_7: KEY_RIGHTALT
      KEY_8: KEY_RIGHTALT
      KEY_9: KEY_RIGHTALT
      KEY_MINUS: KEY_RIGHTALT
      KEY_EQUAL: KEY_RIGHTALT
      KEY_BACKSPACE: KEY_RIGHTALT
      KEY_ENTER: KEY_RIGHTALT
      KEY_LEFTBRACE: KEY_RIGHTALT
      KEY_RIGHTBRACE: KEY_RIGHTALT
      KEY_BACKSLASH: KEY_RIGHTALT
      KEY_SEMICOLON: KEY_RIGHTALT
      KEY_APOSTROPHE: KEY_RIGHTALT
      KEY_COMMA: KEY_RIGHTALT
      KEY_GRAVE: KEY_RIGHTALT
      KEY_SLASH: KEY_RIGHTALT
      KEY_LEFT: KEY_RIGHTALT
      KEY_RIGHT: KEY_RIGHTALT
      KEY_UP: KEY_RIGHTALT
      KEY_DOWN: KEY_RIGHTALT
      KEY_INSERT: KEY_RIGHTALT
      KEY_DELETE: KEY_RIGHTALT
      KEY_HOME: KEY_RIGHTALT
      KEY_END: KEY_RIGHTALT
      KEY_PAGEUP: KEY_RIGHTALT
      KEY_PAGEDOWN: KEY_RIGHTALT
      KEY_TAB: KEY_RIGHTALT
    mode: default

  - name: all
    remap:
      KEY_COMMA: KEY_CAPSLOCK
      KEY_DOT: { set_mode: mode_chooser }

  - name: shift
    remap:
      KEY_0: SHIFT-KEY_0
      KEY_1: SHIFT-KEY_1
      KEY_2: SHIFT-KEY_2
      KEY_3: SHIFT-KEY_3
      KEY_4: SHIFT-KEY_4
      KEY_5: SHIFT-KEY_5
      KEY_6: SHIFT-KEY_6
      KEY_7: SHIFT-KEY_7
      KEY_8: SHIFT-KEY_8
      KEY_9: SHIFT-KEY_9
      KEY_MINUS: SHIFT-KEY_MINUS
      KEY_EQUAL: SHIFT-KEY_EQUAL

      KEY_Q: SHIFT-KEY_Q
      KEY_W: SHIFT-KEY_W
      KEY_E: SHIFT-KEY_E
      KEY_R: SHIFT-KEY_R
      KEY_T: SHIFT-KEY_T
      KEY_Y: SHIFT-KEY_Y
      KEY_U: SHIFT-KEY_U
      KEY_I: SHIFT-KEY_I
      KEY_O: SHIFT-KEY_O
      KEY_P: SHIFT-KEY_P
      KEY_LEFTBRACE: SHIFT-KEY_LEFTBRACE
      KEY_RIGHTBRACE: SHIFT-KEY_RIGHTBRACE
      KEY_BACKSLASH: SHIFT-KEY_BACKSLASH

      KEY_A: SHIFT-KEY_A
      KEY_S: SHIFT-KEY_S
      KEY_D: SHIFT-KEY_D
      KEY_F: SHIFT-KEY_F
      KEY_G: SHIFT-KEY_G
      KEY_H: SHIFT-KEY_H
      KEY_J: SHIFT-KEY_J
      KEY_K: SHIFT-KEY_K
      KEY_L: SHIFT-KEY_L
      KEY_SEMICOLON: SHIFT-KEY_SEMICOLON
      KEY_APOSTROPHE: SHIFT-KEY_APOSTROPHE

      KEY_Z: SHIFT-KEY_Z
      KEY_X: SHIFT-KEY_X
      KEY_C: SHIFT-KEY_C
      KEY_V: SHIFT-KEY_V
      KEY_B: SHIFT-KEY_B
      KEY_N: SHIFT-KEY_N
      KEY_M: SHIFT-KEY_M
      KEY_COMMA: SHIFT-KEY_COMMA
      KEY_DOT: SHIFT-KEY_DOT
      KEY_SLASH: SHIFT-KEY_SLASH
      KEY_SPACE: SHIFT-KEY_SPACE
    mode: shift

  - name: ctrl
    remap:
      KEY_0: CTRL-KEY_0
      KEY_1: CTRL-KEY_1
      KEY_2: CTRL-KEY_2
      KEY_3: CTRL-KEY_3
      KEY_4: CTRL-KEY_4
      KEY_5: CTRL-KEY_5
      KEY_6: CTRL-KEY_6
      KEY_7: CTRL-KEY_7
      KEY_8: CTRL-KEY_8
      KEY_9: CTRL-KEY_9
      KEY_MINUS: CTRL-KEY_MINUS
      KEY_EQUAL: CTRL-KEY_EQUAL

      KEY_Q: CTRL-KEY_Q
      KEY_W: CTRL-KEY_W
      KEY_E: CTRL-KEY_E
      KEY_R: CTRL-KEY_R
      KEY_T: CTRL-KEY_T
      KEY_Y: CTRL-KEY_Y
      KEY_U: CTRL-KEY_U
      KEY_I: CTRL-KEY_I
      KEY_O: CTRL-KEY_O
      KEY_P: CTRL-KEY_P
      KEY_LEFTBRACE: CTRL-KEY_LEFTBRACE
      KEY_RIGHTBRACE: CTRL-KEY_RIGHTBRACE
      KEY_BACKSLASH: CTRL-KEY_BACKSLASH

      KEY_A: CTRL-KEY_A
      KEY_S: CTRL-KEY_S
      KEY_D: CTRL-KEY_D
      KEY_F: CTRL-KEY_F
      KEY_G: CTRL-KEY_G
      KEY_H: CTRL-KEY_H
      KEY_J: CTRL-KEY_J
      KEY_K: CTRL-KEY_K
      KEY_L: CTRL-KEY_L
      KEY_SEMICOLON: CTRL-KEY_SEMICOLON
      KEY_APOSTROPHE: CTRL-KEY_APOSTROPHE

      KEY_Z: CTRL-KEY_Z
      KEY_X: CTRL-KEY_X
      KEY_C: CTRL-KEY_C
      KEY_V: CTRL-KEY_V
      KEY_B: CTRL-KEY_B
      KEY_N: CTRL-KEY_N
      KEY_M: CTRL-KEY_M
      KEY_COMMA: CTRL-KEY_COMMA
      KEY_DOT: CTRL-KEY_DOT
      KEY_SLASH: CTRL-KEY_SLASH
      KEY_SPACE: CTRL-KEY_SPACE
    mode: ctrl

  - name: alt
    remap:
      KEY_0: ALT-KEY_0
      KEY_1: ALT-KEY_1
      KEY_2: ALT-KEY_2
      KEY_3: ALT-KEY_3
      KEY_4: ALT-KEY_4
      KEY_5: ALT-KEY_5
      KEY_6: ALT-KEY_6
      KEY_7: ALT-KEY_7
      KEY_8: ALT-KEY_8
      KEY_9: ALT-KEY_9
      KEY_MINUS: ALT-KEY_MINUS
      KEY_EQUAL: ALT-KEY_EQUAL

      KEY_Q: ALT-KEY_Q
      KEY_W: ALT-KEY_W
      KEY_E: ALT-KEY_E
      KEY_R: ALT-KEY_R
      KEY_T: ALT-KEY_T
      KEY_Y: ALT-KEY_Y
      KEY_U: ALT-KEY_U
      KEY_I: ALT-KEY_I
      KEY_O: ALT-KEY_O
      KEY_P: ALT-KEY_P
      KEY_LEFTBRACE: ALT-KEY_LEFTBRACE
      KEY_RIGHTBRACE: ALT-KEY_RIGHTBRACE
      KEY_BACKSLASH: ALT-KEY_BACKSLASH

      KEY_A: ALT-KEY_A
      KEY_S: ALT-KEY_S
      KEY_D: ALT-KEY_D
      KEY_F: ALT-KEY_F
      KEY_G: ALT-KEY_G
      KEY_H: ALT-KEY_H
      KEY_J: ALT-KEY_J
      KEY_K: ALT-KEY_K
      KEY_L: ALT-KEY_L
      KEY_SEMICOLON: ALT-KEY_SEMICOLON
      KEY_APOSTROPHE: ALT-KEY_APOSTROPHE

      KEY_Z: ALT-KEY_Z
      KEY_X: ALT-KEY_X
      KEY_C: ALT-KEY_C
      KEY_V: ALT-KEY_V
      KEY_B: ALT-KEY_B
      KEY_N: ALT-KEY_N
      KEY_M: ALT-KEY_M
      KEY_COMMA: ALT-KEY_COMMA
      KEY_DOT: ALT-KEY_DOT
      KEY_SLASH: ALT-KEY_SLASH
      KEY_SPACE: ALT-KEY_SPACE
    mode: alt

  #- name: ctrl-shift
  #  remap:
  #    KEY_0: ALT-KEY_0
  #    KEY_1: ALT-KEY_1
  #    KEY_2: ALT-KEY_2
  #    KEY_3: ALT-KEY_3
  #    KEY_4: ALT-KEY_4
  #    KEY_5: ALT-KEY_5
  #    KEY_6: ALT-KEY_6
  #    KEY_7: ALT-KEY_7
  #    KEY_8: ALT-KEY_8
  #    KEY_9: ALT-KEY_9
  #    KEY_MINUS: ALT-KEY_MINUS
  #    KEY_EQUAL: ALT-KEY_EQUAL

  #    KEY_Q: ALT-KEY_Q
  #    KEY_W: ALT-KEY_W
  #    KEY_E: ALT-KEY_E
  #    KEY_R: ALT-KEY_R
  #    KEY_T: ALT-KEY_T
  #    KEY_Y: ALT-KEY_Y
  #    KEY_U: ALT-KEY_U
  #    KEY_I: ALT-KEY_I
  #    KEY_O: ALT-KEY_O
  #    KEY_P: ALT-KEY_P
  #    KEY_LEFTBRACE: ALT-KEY_LEFTBRACE
  #    KEY_RIGHTBRACE: ALT-KEY_RIGHTBRACE
  #    KEY_BACKSLASH: ALT-KEY_BACKSLASH

  #    KEY_A: ALT-KEY_A
  #    KEY_S: ALT-KEY_S
  #    KEY_D: ALT-KEY_D
  #    KEY_F: ALT-KEY_F
  #    KEY_G: ALT-KEY_G
  #    KEY_H: ALT-KEY_H
  #    KEY_J: ALT-KEY_J
  #    KEY_K: ALT-KEY_K
  #    KEY_L: ALT-KEY_L
  #    KEY_SEMICOLON: ALT-KEY_SEMICOLON
  #    KEY_APOSTROPHE: ALT-KEY_APOSTROPHE

  #    KEY_Z: ALT-KEY_Z
  #    KEY_X: ALT-KEY_X
  #    KEY_C: ALT-KEY_C
  #    KEY_V: ALT-KEY_V
  #    KEY_B: ALT-KEY_B
  #    KEY_N: ALT-KEY_N
  #    KEY_M: ALT-KEY_M
  #    KEY_COMMA: ALT-KEY_COMMA
  #    KEY_DOT: ALT-KEY_DOT
  #    KEY_SLASH: ALT-KEY_SLASH
  #    KEY_SPACE: ALT-KEY_SPACE
  #  mode:

  #- name: ctrl-alt
  #  remap:
  #  mode:

  #- name: shift-alt
  #  remap:
  #  mode:

  #- name: ctrl-shift-alt
  #  remap:
  #  mode:

"#;

        let config = serde_yaml::from_str::<super::Config>(config_str);

        assert!(config.is_ok());
    }
}
