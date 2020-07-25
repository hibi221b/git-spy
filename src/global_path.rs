use lazy_static::lazy_static;
use std::path::PathBuf;

// $HOME/Downloads/git-spy-result
lazy_static! {
    pub static ref DOWNLOAD_DIR: PathBuf = {
        let mut download_dir = dirs::download_dir().unwrap();
        download_dir.push("git-spy-result");
        download_dir
    };
}

// $HOME/Downloads/git-spy-result/index.html
lazy_static! {
    pub static ref INDEX_HTML: PathBuf = {
        let mut index_html = dirs::download_dir().unwrap();
        index_html.push("git-spy-result");
        index_html.push("index.html");
        index_html
    };
}