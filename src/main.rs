use femark::{ HTMLOutput, process_markdown_to_html };
// use std::env;
use std::fs;
// use std::fs::metadata;
use std::path::PathBuf;
const baseDir: &str = "./markdown";

fn main() {
    recursiveReadDir(baseDir);
}

fn recursiveReadDir(dir: &str) {
    let paths = fs::read_dir(dir).unwrap();
    println!("In dir: {}", dir);
    for path in paths {
        let pathname = path.unwrap().path().display().to_string();
        let pathBuf = PathBuf::from(pathname.clone());
        println!("Name: {}", pathname);

        if pathBuf.is_dir() {
            println!("Recursion: {}", pathname);
            recursiveReadDir(&pathname);
        } else {
            if !pathname.ends_with(".md") {
                println!("Not a markdown file");
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
            println!("Title: {}", content);
        }
    }
}
