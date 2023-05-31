use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
};

use nu_ansi_term::{AnsiString, Color, Style};
use zellij_tile::prelude::{actions::Action, *};

#[derive(Default)]
struct State {
    tabs: Vec<TabInfo>,
    mode_info: ModeInfo,
    icon: bool,
}

#[derive(Eq, Hash, PartialEq)]
struct ActionInfo {
    name: String,
    icon: String,
    sort: usize,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        set_selectable(false);
        subscribe(&[EventType::ModeUpdate, EventType::TabUpdate]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::ModeUpdate(mode_info) => {
                if self.mode_info != mode_info {
                    should_render = true;
                }
                self.mode_info = mode_info;
                self.icon = self.mode_info.capabilities.arrow_fonts;
            }
            Event::TabUpdate(tabs) => {
                if self.tabs != tabs {
                    should_render = true;
                }
                self.tabs = tabs;
            }
            _ => {}
        };
        should_render
    }

    fn render(&mut self, _rows: usize, cols: usize) {
        if cols == 0 {
            // new tab
            return;
        }
        let mut line_buf = VecDeque::<AnsiString>::new();
        // 配置颜色
        let palette = self.mode_info.style.colors;
        let color_bg = match palette.bg {
            PaletteColor::Rgb(rgb) => {
                print!(
                    "\u{1b}[0;0H\u{1b}[48;2;{};{};{}m\u{1b}[0K",
                    rgb.0, rgb.1, rgb.2
                );
                Color::Rgb(rgb.0, rgb.1, rgb.2)
            }
            PaletteColor::EightBit(fixed) => {
                print!("\u{1b}[0;0H\u{1b}[48;5;{}m\u{1b}[0K", fixed);
                Color::Fixed(fixed)
            }
        };
        let color_text = palette_color_to_color(&palette.white);
        let color_active = palette_color_to_color(&palette.green);
        let color_tab = palette_color_to_color(&palette.white);

        // 渲染Tab
        let mut tab_buf = VecDeque::<AnsiString>::new();
        let mut tab_is_actived = false;
        let max_tab_count = 5;
        for tab in &self.tabs {
            if (self.mode_info.mode == InputMode::Normal)
                || (self.mode_info.mode == InputMode::Tab)
                || tab.active
            {
                let mut color = color_tab;
                if tab.active {
                    tab_is_actived = true;
                    color = color_active;
                }

                tab_buf.push_back(Style::new().on(color).fg(color_bg).paint(""));
                tab_buf.push_back(Style::new().on(color).fg(color_bg).paint(&tab.name));
                tab_buf.push_back(Style::new().on(color_bg).fg(color).bold().paint(""));

                if tab_buf.len() > max_tab_count * 3 {
                    tab_buf.pop_front();
                    tab_buf.pop_front();
                    tab_buf.pop_front();
                }

                if tab_buf.len() == max_tab_count * 3 && tab_is_actived {
                    break;
                }
            }
        }
        line_buf.append(&mut tab_buf);

        // 按action聚合快捷键 例如 Move ←↑→↓
        let mut action_keys = HashMap::<ActionInfo, RefCell<Vec<String>>>::new();
        for (key, actions) in self.mode_info.get_mode_keybinds() {
            let action = action_info(actions.first().unwrap());
            let mut key_string = key.to_string();
            // 长key Ctrl+n 与 PageDown 结尾添加分隔符
            if key_string.chars().count() > 1 {
                key_string += "/";
            }

            if let Some(ref_vec) = action_keys.get(&action) {
                ref_vec.borrow_mut().push(key_string);
            } else {
                let ref_vec = RefCell::new(Vec::<String>::new());
                ref_vec.borrow_mut().push(key_string);
                action_keys.insert(action, ref_vec);
            }
        }

        let mut vec_action_info: Vec<&ActionInfo> = action_keys.keys().collect();
        vec_action_info.sort_by(|a, b| a.sort.cmp(&b.sort));

        // 渲染快捷键
        for action in vec_action_info {
            let action_string = match self.icon {
                true => &action.icon,
                false => &action.name,
            };
            let mut keys = action_keys.get(action).unwrap().borrow_mut();
            keys.sort();

            let mut keys_string = String::new();
            for key in keys.iter() {
                keys_string += key;
            }
            // 去除 长key Ctrl+n 与 PageDown 结尾分隔符
            if let Some(k) = keys_string.strip_suffix('/') {
                keys_string = k.to_string();
            }
            line_buf.push_back(Style::new().on(color_bg).fg(color_text).paint(" / "));
            line_buf.push_back(Style::new().on(color_bg).fg(color_active).paint("<"));
            line_buf.push_back(
                Style::new()
                    .on(color_bg)
                    .fg(color_active)
                    .underline()
                    .paint(keys_string),
            );
            line_buf.push_back(Style::new().on(color_bg).fg(color_active).paint(">"));
            line_buf.push_back(
                Style::new()
                    .on(color_bg)
                    .fg(color_text)
                    .paint(action_string),
            );
        }

        let mut string_buf = String::new();
        let mut left_cols = cols;
        // 截取字符串
        for ansi_string in line_buf {
            let count = ansi_string.as_str().chars().count();
            if left_cols >= count {
                string_buf += &ansi_string.to_string();
                left_cols -= count;
            } else {
                break;
            }
        }
        print!("{}", string_buf);

        // 渲染model
        let string_model = mode_to_str(&self.mode_info.mode);
        print!(
            "\u{1b}[0;{}H{}{}{}",
            cols - string_model.chars().count() - 1,
            Style::new()
                .on(color_active)
                .fg(color_bg)
                .paint("")
                .to_string(),
            Style::new()
                .on(color_active)
                .fg(color_bg)
                .paint(string_model),
            Style::new().on(color_bg).fg(color_active).bold().paint("")
        );
        // String::from(" ").repeat(10);
    }
}

// zellij color to nu_ansi_term color
fn palette_color_to_color(palette_color: &PaletteColor) -> Color {
    match palette_color {
        PaletteColor::Rgb(rgb) => Color::Rgb(rgb.0, rgb.1, rgb.2),
        PaletteColor::EightBit(fixed) => Color::Fixed(fixed.clone()),
    }
}

fn mode_to_str(m: &InputMode) -> String {
    match m {
        InputMode::Normal => String::from("Normal"),
        InputMode::Locked => String::from("Locked"),
        InputMode::Resize => String::from("Resize"),
        InputMode::Pane => String::from("Pane"),
        InputMode::Tab => String::from("Tab"),
        InputMode::Scroll => String::from("Scroll"),
        InputMode::EnterSearch => String::from("EnterSearch"),
        InputMode::Search => String::from("Search"),
        InputMode::RenameTab => String::from("Rename"),
        InputMode::RenamePane => String::from("Rename"),
        InputMode::Session => String::from("Session"),
        InputMode::Move => String::from("Move"),
        InputMode::Prompt => String::from("Prompt"),
        InputMode::Tmux => String::from("Tmux"),
    }
}

// action to name and icon nerdfonts.com/cheat-sheet nf-md-
fn action_info(action: &Action) -> ActionInfo {
    match action {
        // shared nf-md-
        Action::Quit => ActionInfo {
            name: String::from("Quit"),
            icon: String::from("󰩈"),
            sort: 1000,
        },
        Action::Detach => ActionInfo {
            name: String::from("Detach"),
            icon: String::from("󱘖"),
            sort: 990,
        },

        Action::SwitchToMode(m) => match m {
            InputMode::Normal => ActionInfo {
                name: mode_to_str(m),
                icon: String::from("󰆍"),
                sort: 100,
            },
            InputMode::Locked => ActionInfo {
                name: mode_to_str(m),
                icon: String::from("󰍁"),
                sort: 110,
            },
            InputMode::Resize => ActionInfo {
                name: mode_to_str(m),
                icon: String::from("󰁌"),
                sort: 125,
            },
            InputMode::Pane => ActionInfo {
                name: mode_to_str(m),
                icon: String::from("󰄱"),
                sort: 120,
            },
            InputMode::Tab => ActionInfo {
                name: mode_to_str(m),
                icon: String::from("󰉖"),
                sort: 170,
            },
            InputMode::Scroll => ActionInfo {
                name: mode_to_str(m),
                icon: String::from("󰒺"),
                sort: 140,
            },
            InputMode::EnterSearch => ActionInfo {
                name: mode_to_str(m),
                icon: String::from("󱎸"),
                sort: 150,
            },
            InputMode::Search => ActionInfo {
                name: mode_to_str(m),
                icon: String::from("󰍉"),
                sort: 145,
            },
            InputMode::RenameTab => ActionInfo {
                name: mode_to_str(m),
                icon: String::from("󰷎"),
                sort: 175,
            },
            InputMode::RenamePane => ActionInfo {
                name: mode_to_str(m),
                icon: String::from("󰏭"),
                sort: 135,
            },
            InputMode::Session => ActionInfo {
                name: mode_to_str(m),
                icon: String::from(""),
                sort: 180,
            },
            InputMode::Move => ActionInfo {
                name: mode_to_str(m),
                icon: String::from("󰁁"),
                sort: 130,
            },
            InputMode::Prompt => ActionInfo {
                name: mode_to_str(m),
                icon: String::from("󰆅"),
                sort: 185,
            },
            InputMode::Tmux => ActionInfo {
                name: mode_to_str(m),
                icon: String::from("󰰤"),
                sort: 190,
            },
        },
        Action::ToggleFloatingPanes => ActionInfo {
            name: String::from("Floating"),
            icon: String::from("󱣵"),
            sort: 150,
        },
        Action::ToggleFocusFullscreen => ActionInfo {
            name: String::from("Fullscreen"),
            icon: String::from("󰊓"),
            sort: 160,
        },
        //pane
        Action::NewPane(od, _) => match od {
            None => ActionInfo {
                name: String::from("New"),
                icon: String::from("󰜄"),
                sort: 100,
            },
            Some(_) => ActionInfo {
                name: String::from("NewDirection"),
                icon: String::from("󰜶"),
                sort: 105,
            },
        },
        Action::MoveFocus(_) => ActionInfo {
            name: String::from("MoveFocus"),
            icon: String::from("󰋱"),
            sort: 110,
        },
        Action::SwitchFocus => ActionInfo {
            name: String::from("Switch"),
            icon: String::from("󰽐"),
            sort: 120,
        },
        Action::TogglePaneFrames => ActionInfo {
            name: String::from("Frames"),
            icon: String::from("󰃐"),
            sort: 140,
        },
        Action::TogglePaneEmbedOrFloating => ActionInfo {
            name: String::from("Embed"),
            icon: String::from("󱥧"),
            sort: 170,
        },
        Action::CloseFocus => ActionInfo {
            name: String::from("Close"),
            icon: String::from("󰅘"),
            sort: 190,
        },
        // move
        Action::MovePane(od) => match od {
            Some(_) => ActionInfo {
                name: String::from("MoveDirection"),
                icon: String::from("󰁁"),
                sort: 200,
            },
            None => ActionInfo {
                name: String::from("Move"),
                icon: String::from("󰑐"),
                sort: 200,
            },
        },
        Action::MovePaneBackwards => ActionInfo {
            name: String::from("Backwards"),
            icon: String::from("󰕍"),
            sort: 210,
        },
        // resize
        Action::Resize(r, od) => match (r, od) {
            (Resize::Increase, None) => ActionInfo {
                name: String::from("Increase"),
                icon: String::from("󰁌"),
                sort: 300,
            },
            (Resize::Decrease, None) => ActionInfo {
                name: String::from("Decrease"),
                icon: String::from("󰁄"),
                sort: 310,
            },
            (Resize::Increase, Some(_)) => ActionInfo {
                name: String::from("IncreaseDirection"),
                icon: String::from("󰹷"),
                sort: 320,
            },
            (Resize::Decrease, Some(_)) => ActionInfo {
                name: String::from("DecreaseDirection"),
                icon: String::from("󰘕"),
                sort: 330,
            },
        },
        // tab
        Action::ToggleTab => ActionInfo {
            name: String::from("Tab"),
            icon: String::from("󰾷"),
            sort: 410,
        },
        Action::GoToPreviousTab => ActionInfo {
            name: String::from("Previous"),
            icon: String::from("󱃭"),
            sort: 420,
        },
        Action::GoToNextTab => ActionInfo {
            name: String::from("Next"),
            icon: String::from("󱃩"),
            sort: 430,
        },
        Action::GoToTab(_) => ActionInfo {
            name: String::from("Go#"),
            icon: String::from("󰴊"),
            sort: 440,
        },
        Action::NewTab(_, _, _, _, _) => ActionInfo {
            name: String::from("New"),
            icon: String::from("󰮝"),
            sort: 400,
        },
        Action::ToggleActiveSyncTab => ActionInfo {
            name: String::from("Sync"),
            icon: String::from("󰌹"),
            sort: 450,
        },
        Action::CloseTab => ActionInfo {
            name: String::from("Close"),
            icon: String::from("󰮞"),
            sort: 490,
        },
        // scroll
        Action::EditScrollback => ActionInfo {
            name: String::from("Scrollback"),
            icon: String::from("󰕍"),
            sort: 500,
        },
        Action::ScrollDown => ActionInfo {
            name: String::from("Down"),
            icon: String::from("󰒺"),
            sort: 510,
        },
        Action::ScrollUp => ActionInfo {
            name: String::from("Up"),
            icon: String::from("󰒽"),
            sort: 520,
        },
        Action::HalfPageScrollDown => ActionInfo {
            name: String::from("HalfPageDown"),
            icon: String::from("󰄼"),
            sort: 530,
        },
        Action::HalfPageScrollUp => ActionInfo {
            name: String::from("HalfPageUp"),
            icon: String::from("󰄿"),
            sort: 540,
        },
        Action::PageScrollDown => ActionInfo {
            name: String::from("PageDown"),
            icon: String::from("󰶹"),
            sort: 550,
        },
        Action::PageScrollUp => ActionInfo {
            name: String::from("PageUp"),
            icon: String::from("󰶼"),
            sort: 560,
        },
        //search
        Action::Search(_) => ActionInfo {
            name: String::from("Search"),
            icon: String::from("󰍉"),
            sort: 700,
        },
        Action::SearchToggleOption(_) => ActionInfo {
            name: String::from("Option"),
            icon: String::from("󱡴"),
            sort: 710,
        },
        // rename
        Action::UndoRenameTab => ActionInfo {
            name: String::from("UndoRename"),
            icon: String::from("󰕍"),
            sort: 480,
        },
        Action::UndoRenamePane => ActionInfo {
            name: String::from("UndoRename"),
            icon: String::from("󰕍"),
            sort: 180,
        },
        _ => ActionInfo {
            name: String::from("None"),
            icon: String::from("󱥀"),
            sort: 1010,
        },
    }
}
