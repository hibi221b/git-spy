![logo](https://user-images.githubusercontent.com/29950288/88449229-1adc4380-ce80-11ea-8bc2-05f53fb8d8dc.png)

[![travisci](https://img.shields.io/travis/hibi221b/git-spy?style=for-the-badge)](https://travis-ci.org/github/hibi221b/git-spy)
[![license](https://img.shields.io/github/license/hibi221b/git-spy?style=for-the-badge)](https://github.com/hibi221b/git-spy/blob/master/LICENSE)

# Demo

![git-spy](https://user-images.githubusercontent.com/29950288/89114451-72e5fc00-d4b7-11ea-9495-7064e454e6e2.gif)

# Prerequisite

- Chrome or Chromium

# Environment

| mac | linux | windows |
|:---:|:---:|:---:|
|○ |○ |△ |

# How to install

## cargo

```terminal
cargo install git-spy
```

# Usage

```terminal
git-spy VERSION
hibi221b
command-line tool to efficiently collect github repository

USAGE:
    git-spy

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

# How to use

```terminal
$ git-spy
```

- this command need not arguments

## [1] input language you want to search

- you can select one language
- rust, c, c++, go, swift, javascript, python, etc...

## [2] input lower bound of github star

- 100, 500, 1000, etc...

## [3] input upper bound of github star

- The upper bound needs to be greater than the lower bound.
- 100, 500, 1000, etc... or *

## [4] input topics 

- you can select topics you want to search
- cli, web, gui, terminal, etc... or nothing

## [5] confirm headless_mode

- press enter: hide chrome 
- press n:     visualize chrome

```terminal

  _______   __  .___________.         _______. .______   ____    ____ 
 /  _____| |  | |           |        /       | |   _  \  \   \  /   / 
|  |  __   |  | `---|  |----`______ |   (----` |  |_)  |  \   \/   /  
|  | |_ |  |  |     |  |    |______| \   \     |   ___/    \_    _/   
|  |__| |  |  |     |  |         .----)   |    |  |          |  |     
 \______|  |__|     |__|         |_______/     |__|          |__|  
                        
                             https://github.com/hibi221b/git-spy

▼ language
rust <-------------------------------------------------------------------------------------- [1]
▼ lower bound of github star.
1000 <-------------------------------------------------------------------------------------- [2]
▼ upper bound of github star. (number or *)
* <----------------------------------------------------------------------------------------- [3]
▼ topic words.(cli, web, gui, terminal, etc...) If you don't need, press enter.
cli terminal command tool <----------------------------------------------------------------- [4]
▼ press enter: hide chrome, input `n`: visualize chrome
 <------------------------------------------------------------------------------------------ [5]
headless mode: true
search query:  language:rust stars:1000..* topic:"cli terminal command tool"

info: 27 repository results (The maximum number of repositories git-spy can get is 100)
info: hit 10 repositories
info: page 1/10
get:  https://github.com/alacritty/alacritty
get:  https://github.com/sharkdp/bat
get:  https://github.com/BurntSushi/ripgrep
get:  https://github.com/sharkdp/fd
get:  https://github.com/denisidoro/navi
get:  https://github.com/BurntSushi/xsv
get:  https://github.com/Rigellute/spotify-tui
get:  https://github.com/sharkdp/hyperfine
get:  https://github.com/imsnif/bandwhich
get:  https://github.com/sharkdp/hexyl
info: go to the next page

info: hit 10 repositories
info: page 2/10
get:  https://github.com/timvisee/ffsend
get:  https://github.com/svenstaro/genact
get:  https://github.com/XAMPPRocky/tokei
get:  https://github.com/imazen/imageflow
get:  https://github.com/fdehau/tui-rs
get:  https://github.com/sharkdp/pastel
get:  https://github.com/rustwasm/wasm-pack
get:  https://github.com/jmacdonald/amp
get:  https://github.com/vi/websocat
get:  https://github.com/gyscos/cursive
info: go to the next page

info: hit 7 repositories
info: page 3/10
get:  https://github.com/jhspetersson/fselect
get:  https://github.com/cloudflare/wrangler
get:  https://github.com/extrawurst/gitui
get:  https://github.com/chmln/sd
get:  https://github.com/killercup/cargo-edit
get:  https://github.com/redox-os/termion
get:  https://github.com/databricks/click

info: 27 repositories acquired. ($HOME/Downloads/git-spy-result/xxxxx.json)
info: scraping successfully finished.
```

# Result
- create `$HOME/Downloads/git-spy-result`
- create `$HOME/Downloads/git-spy-result/index.html`
- create `$HOME/Downloads/git-spy-result/XXX.json`

# open $HOME/Downloads/git-spy-result/index.html

you open the index.html file and open this saved json file and display it.

- `$HOME/Downloads/git-spy-result/index.html`

<img width="1678" alt="screenshot" src="https://user-images.githubusercontent.com/29950288/89114468-b93b5b00-d4b7-11ea-890e-1ca984bc5430.png">

- `$HOME/Downloads/git-spy-result/XXX.json`

```json
[
  {
    "url": "https://github.com/alacritty/alacritty",
    "repo": "alacritty",
    "desc": "A cross-platform, GPU-accelerated terminal emulator",
    "star": "24.1k",
    "keywords": "terminal-emulators macos gpu linux windows rust terminal opengl bsd vte"
  },
  {
    "url": "https://github.com/sharkdp/bat",
    "repo": "bat",
    "desc": "A cat(1) clone with wings.",
    "star": "21.1k",
    "keywords": "syntax-highlighting git rust cli terminal command-line tool"
  },
  {
    "url": "https://github.com/BurntSushi/ripgrep",
    "repo": "ripgrep",
    "desc": "ripgrep recursively searches directories for a regex pattern",
    "star": "20.8k",
    "keywords": "ripgrep gitignore search cli command-line regex recursively-search command-line-tool grep"
  },
  {
    "url": "https://github.com/sharkdp/fd",
    "repo": "fd",
    "desc": "A simple, fast and user-friendly alternative to 'find'",
    "star": "14.3k",
    "keywords": "search rust cli terminal command-line tool filesystem regex"
  },
  {
    "url": "https://github.com/denisidoro/navi",
    "repo": "navi",
    "desc": "An interactive cheatsheet tool for the command-line and application launchers",
    "star": "7.4k",
    "keywords": "cheatsheets shell bash rust cli snippets terminal command-line snippet"
  },
  {
    "url": "https://github.com/BurntSushi/xsv",
    "repo": "xsv",
    "desc": "A fast CSV command line toolkit written in Rust.",
    "star": "6.2k",
    "keywords": "rust cli csv command-line"
  },
  {
    "url": "https://github.com/Rigellute/spotify-tui",
    "repo": "spotify-tui",
    "desc": "Spotify for the terminal written in Rust 🚀",
    "star": "6.1k",
    "keywords": "rust spotify-tui spotify-api cli spotify terminal terminal-based"
  },
  {
    "url": "https://github.com/sharkdp/hyperfine",
    "repo": "hyperfine",
    "desc": "A command-line benchmarking tool",
    "star": "5.7k",
    "keywords": "benchmark rust cli terminal command-line tool"
  },
  {
    "url": "https://github.com/imsnif/bandwhich",
    "repo": "bandwhich",
    "desc": "Terminal bandwidth utilization tool",
    "star": "5.5k",
    "keywords": "cli networking dashboard bandwidth"
  },
  {
    "url": "https://github.com/sharkdp/hexyl",
    "repo": "hexyl",
    "desc": "A command-line hex viewer",
    "star": "5.3k",
    "keywords": "rust command-line tool binary-data hexadecimal"
  },
  {
    "url": "https://github.com/timvisee/ffsend",
    "repo": "ffsend",
    "desc": "📬 Easily and securely share files from the command line. A fully featured Firefox Send client.",
    "star": "4.9k",
    "keywords": "encryption rust cli file-upload file-sharing firefox-send"
  },
  {
    "url": "https://github.com/svenstaro/genact",
    "repo": "genact",
    "desc": "🌀 A nonsense activity generator",
    "star": "4.1k",
    "keywords": "cli webassembly wasm fake useless nonsense"
  },
  {
    "url": "https://github.com/XAMPPRocky/tokei",
    "repo": "tokei",
    "desc": "Count your code, quickly.",
    "star": "3.6k",
    "keywords": "statistics badge linux tokei windows macos rust cli code cloc"
  },
  {
    "url": "https://github.com/imazen/imageflow",
    "repo": "imageflow",
    "desc": "High-performance image manipulation for web servers. Includes imageflow_server, imageflow_tool, and libimageflow",
    "star": "3.3k",
    "keywords": "imagemagick tool lib image-manipulation image-compression image-server"
  },
  {
    "url": "https://github.com/fdehau/tui-rs",
    "repo": "tui-rs",
    "desc": "Build terminal user interfaces and dashboards using Rust",
    "star": "3.2k",
    "keywords": "dashboard rust tui terminal"
  },
  {
    "url": "https://github.com/sharkdp/pastel",
    "repo": "pastel",
    "desc": "A command-line tool to generate, analyze, convert and manipulate colors",
    "star": "2.9k",
    "keywords": "rust cli terminal command-line tool colors color-converter color-space"
  },
  {
    "url": "https://github.com/rustwasm/wasm-pack",
    "repo": "wasm-pack",
    "desc": "📦✨ your favorite rust -> wasm workflow tool!",
    "star": "2.8k",
    "keywords": "registry rust-wasm rust cli npm package wasm"
  },
  {
    "url": "https://github.com/jmacdonald/amp",
    "repo": "amp",
    "desc": "A complete text editor for your terminal.",
    "star": "2.2k",
    "keywords": "rust vim terminal text-editor"
  },
  {
    "url": "https://github.com/vi/websocat",
    "repo": "websocat",
    "desc": "Command-line client for WebSockets, like netcat (or curl) for ws:// with advanced socat-like functions",
    "star": "1.9k",
    "keywords": "curl websockets socat netcat cli command-line proxy websocket-server websocket-client command-line-tool rfc-6455"
  },
  {
    "url": "https://github.com/gyscos/cursive",
    "repo": "cursive",
    "desc": "A Text User Interface library for the Rust programming language",
    "star": "1.8k",
    "keywords": "rust tui ncurses terminal"
  },
  {
    "url": "https://github.com/jhspetersson/fselect",
    "repo": "fselect",
    "desc": "Find files with SQL-like queries",
    "star": "1.7k",
    "keywords": "rust sql cli files utility query tool filesystem find sql-like"
  },
  {
    "url": "https://github.com/cloudflare/wrangler",
    "repo": "wrangler",
    "desc": "🤠 wrangle your cloudflare workers",
    "star": "1.5k",
    "keywords": "rust cli serverless cloudflare-workers"
  },
  {
    "url": "https://github.com/extrawurst/gitui",
    "repo": "gitui",
    "desc": "Blazing 💥 fast terminal-ui for git written in rust 🦀",
    "star": "1.4k",
    "keywords": "rust async git terminal tui command-line-tool command-line-interface"
  },
  {
    "url": "https://github.com/chmln/sd",
    "repo": "sd",
    "desc": "Intuitive find & replace CLI (sed alternative)",
    "star": "1.4k",
    "keywords": "regex rust terminal command-line text-processing"
  },
  {
    "url": "https://github.com/killercup/cargo-edit",
    "repo": "cargo-edit",
    "desc": "A utility for managing cargo dependencies from the command line.",
    "star": "1.3k",
    "keywords": "cargo rust cli"
  },
  {
    "url": "https://github.com/redox-os/termion",
    "repo": "termion",
    "desc": "Mirror of https://gitlab.redox-os.org/redox-os/termion",
    "star": "1.3k",
    "keywords": "tty supports-redox rust terminal tui"
  },
  {
    "url": "https://github.com/databricks/click",
    "repo": "click",
    "desc": "The \"Command Line Interactive Controller for Kubernetes\"",
    "star": "1.2k",
    "keywords": "rust cli kuberentes"
  }
]
```
