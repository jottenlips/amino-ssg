use femark::{ HTMLOutput, process_markdown_to_html };
// use std::env;
use std::fs;
// use std::fs::metadata;
use std::path::PathBuf;
// const baseDir: &str = "./markdown";

fn main() {
    let markdownDir = std::env::args().nth(1).expect("no markdown directory given");
    let out = std::env::args().nth(2).expect("no out dir given");

    recursiveReadDir(&markdownDir, "");
}

fn getBaseHtml<'a>(dir: &'a str, baseHtml: &'a str) -> String {
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let pathname = path.unwrap().path().display().to_string();
        let pathBuf = PathBuf::from(pathname.clone());
        if pathBuf.is_dir() {
            continue;
        } else {
            if pathname.ends_with("base.html") {
                // return html read in
                let input = fs
                    ::read_to_string(&pathname)
                    .expect("Should have been able to read the file");
                return input;
            }
        }
    }
    return baseHtml.to_string();
}

fn recursiveReadDir(dir: &str, baseHtml: &str) {
    let html = getBaseHtml(dir, baseHtml);
    println!("html: {}", html);
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        let pathname = path.unwrap().path().display().to_string();
        let pathBuf = PathBuf::from(pathname.clone());
        if pathBuf.is_dir() {
            recursiveReadDir(&pathname, &html);
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
            let page = html.replace("<!-- MARKDOWN -->", &content);
            println!("page: {}", page);
        }
    }
}
