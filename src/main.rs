use femark::{ HTMLOutput, process_markdown_to_html };
// use std::env;
use std::fs;
// use std::fs::metadata;
use std::path::PathBuf;
const baseDir: &str = "./markdown";

fn main() {
    recursiveReadDir(baseDir);
}

fn recursiveReadDir(dir: &str, baseHtml: &str) {
    let paths = fs::read_dir(dir).unwrap();
    let mut html = baseHtml;
    for path in paths {
        let pathname = path.unwrap().path().display().to_string();
        let pathBuf = PathBuf::from(pathname.clone());
        if pathBuf.is_dir() {
            recursiveReadDir(&pathname);
        } else {
            if !pathname.ends_with(".md") {
                continue;
            }
            let input = fs
                ::read_to_string(&pathname)
                .expect("Should have been able to read the file");
            // Processes Markdown without looking for frontmatter
            let Ok(HTMLOutput { content, toc, .. }) = process_markdown_to_html(&input) else {
                println!("Error");
                return;
            };
            println!("page: {}", pathname);
        }
    }
}
