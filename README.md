# build wasm
debug `cargo build --target wasm32-wasi`

release `cargo build -r --target wasm32-wasi`

# copy plugin
`mv zellij-hints-bar.wasm ~/.config/zellij`

# add config
`~/.config/zellij/layouts/default.kdl`

```
// osx system clipboard
copy_command "pbcopy"

plugins {
    hints-bar { path "/Users/charlesvhe/.config/zellij/zellij-hints-bar"; }
    //hints-bar { path "/Volumes/DATA/VSCodeProjects/zellij-hints-bar/target/wasm32-wasi/debug/zellij-hints-bar"; }
}

// toggle enabling icon
simplified_ui false

layout {
    pane
    pane size=1 borderless=true {
        plugin location="zellij:hints-bar"
    }
}

themes {
    default {
        fg 216 222 233
        bg 46 52 64
        black 59 66 82
        red 191 97 106
        green 163 190 140
        yellow 235 203 139
        blue 129 161 193
        magenta 180 142 173
        cyan 136 192 208
        white 229 233 240
        orange 208 135 112
    }
}

keybinds clear-defaults=true {
    normal {
        bind "Ctrl z" { SwitchToMode "prompt"; }
    }
    // prompt mode as a total entry to reduce shortcut key conflicts
    prompt {
        bind "f" { ToggleFloatingPanes; SwitchToMode "Normal"; }
        bind "F" { ToggleFocusFullscreen; SwitchToMode "Normal"; }
        
        bind "p" { SwitchToMode "Pane"; }
        bind "r" { SwitchToMode "Resize"; }
        bind "m" { SwitchToMode "Move"; }
        bind "s" { SwitchToMode "Scroll"; }
        bind "t" { SwitchToMode "Tab"; }

        bind "Ctrl q" { Quit; }
    }

    shared_except "normal" {
        bind "Enter" "Esc" { SwitchToMode "Normal"; }
    }

    resize {
        bind "h" "Left" { Resize "Increase Left"; }
        bind "j" "Down" { Resize "Increase Down"; }
        bind "k" "Up" { Resize "Increase Up"; }
        bind "l" "Right" { Resize "Increase Right"; }
        bind "H" { Resize "Decrease Left"; }
        bind "J" { Resize "Decrease Down"; }
        bind "K" { Resize "Decrease Up"; }
        bind "L" { Resize "Decrease Right"; }
        bind "=" "+" { Resize "Increase"; }
        bind "-" { Resize "Decrease"; }
    }

    pane {
        bind "h" "Left" { MoveFocus "Left"; }
        bind "l" "Right" { MoveFocus "Right"; }
        bind "j" "Down" { MoveFocus "Down"; }
        bind "k" "Up" { MoveFocus "Up"; }
        bind "p" { SwitchFocus; }
        bind "n" { NewPane; SwitchToMode "Normal"; }
        bind "d" { NewPane "Down"; SwitchToMode "Normal"; }
        bind "r" { NewPane "Right"; SwitchToMode "Normal"; }
        bind "x" { CloseFocus; SwitchToMode "Normal"; }
        bind "F" { ToggleFocusFullscreen; SwitchToMode "Normal"; }
        bind "z" { TogglePaneFrames; SwitchToMode "Normal"; }
        bind "f" { ToggleFloatingPanes; SwitchToMode "Normal"; }
        bind "e" { TogglePaneEmbedOrFloating; SwitchToMode "Normal"; }
        bind "c" { SwitchToMode "RenamePane"; PaneNameInput 0;}
    }

    move {
        bind "n" "Tab" { MovePane; }
        bind "p" { MovePaneBackwards; }
        bind "h" "Left" { MovePane "Left"; }
        bind "j" "Down" { MovePane "Down"; }
        bind "k" "Up" { MovePane "Up"; }
        bind "l" "Right" { MovePane "Right"; }
    }

    tab {
        bind "r" { SwitchToMode "RenameTab"; TabNameInput 0; }
        bind "h" "Left" "Up" "k" { GoToPreviousTab; }
        bind "l" "Right" "Down" "j" { GoToNextTab; }
        bind "n" { NewTab; SwitchToMode "Normal"; }
        bind "x" { CloseTab; SwitchToMode "Normal"; }
        bind "s" { ToggleActiveSyncTab; SwitchToMode "Normal"; }
        bind "1" { GoToTab 1; SwitchToMode "Normal"; }
        bind "2" { GoToTab 2; SwitchToMode "Normal"; }
        bind "3" { GoToTab 3; SwitchToMode "Normal"; }
        bind "4" { GoToTab 4; SwitchToMode "Normal"; }
        bind "5" { GoToTab 5; SwitchToMode "Normal"; }
        bind "6" { GoToTab 6; SwitchToMode "Normal"; }
        bind "7" { GoToTab 7; SwitchToMode "Normal"; }
        bind "8" { GoToTab 8; SwitchToMode "Normal"; }
        bind "9" { GoToTab 9; SwitchToMode "Normal"; }
        bind "Tab" { ToggleTab; }
    }

    scroll {
        bind "e" { EditScrollback; SwitchToMode "Normal"; }
        bind "s" { SwitchToMode "EnterSearch"; SearchInput 0; }
        bind "j" "Down" { ScrollDown; }
        bind "k" "Up" { ScrollUp; }
        bind "d" { HalfPageScrollDown; }
        bind "u" { HalfPageScrollUp; }
        bind "PageDown" "Right" "l" { PageScrollDown; }
        bind "PageUp" "Left" "h" { PageScrollUp; }
    }
    
    search {
        bind "j" "Down" { ScrollDown; }
        bind "k" "Up" { ScrollUp; }
        bind "PageDown" "Right" "l" { PageScrollDown; }
        bind "PageUp" "Left" "h" { PageScrollUp; }
        bind "d" { HalfPageScrollDown; }
        bind "u" { HalfPageScrollUp; }
        bind "n" { Search "down"; }
        bind "p" { Search "up"; }
        bind "c" { SearchToggleOption "CaseSensitivity"; }
        bind "w" { SearchToggleOption "Wrap"; }
        bind "o" { SearchToggleOption "WholeWord"; }
    }

    entersearch {
        bind "Esc" { SwitchToMode "Scroll"; }
        bind "Enter" { SwitchToMode "Search"; }
    }

    renametab {
        bind "Esc" { UndoRenameTab; SwitchToMode "Tab"; }
    }

    renamepane {
        bind "Esc" { UndoRenamePane; SwitchToMode "Pane"; }
    }
}

```