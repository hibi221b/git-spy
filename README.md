![logo](https://user-images.githubusercontent.com/29950288/88449229-1adc4380-ce80-11ea-8bc2-05f53fb8d8dc.png)

[![travisci](https://img.shields.io/travis/hibi221b/git-spy?style=for-the-badge)](https://travis-ci.org/github/hibi221b/git-spy)
[![license](https://img.shields.io/github/license/hibi221b/git-spy?style=for-the-badge)](https://github.com/hibi221b/git-spy/blob/master/LICENSE)
[![crates.io](https://img.shields.io/crates/v/git-spy?style=for-the-badge)](https://crates.io/crates/git-spy)

# ~~Demo~~

# Prerequisite

- Chrome or Chromium

# Environment

| mac | linux | windows |
|:---:|:---:|:---:|
|○ |○ |△ |

# How to install

## cargo

### Recommend

```terminal
cargo install --git https://github.com/hibi221b/git-spy.git
```

### NOT recommend

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

- this command needs no arguments

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

## ~~[5] confirm headless_mode~~

- git-spy always open browser.

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

headless mode: false
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
  ...
  ...
  ...
  {
    "url": "https://github.com/databricks/click",
    "repo": "click",
    "desc": "The \"Command Line Interactive Controller for Kubernetes\"",
    "star": "1.2k",
    "keywords": "rust cli kuberentes"
  }
]
```
