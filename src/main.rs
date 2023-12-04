use femark::{ HTMLOutput, process_markdown_to_html };
// use std::env;
use std::fs;
use crate::fs::File;
use std::io::Write;
// use std::fs::metadata;
use std::path::PathBuf;
// const baseDir: &str = "./markdown";

fn main() {
    let markdownDir = std::env::args().nth(1).expect("no markdown directory given");
    let out = std::env::args().nth(2).expect("no out dir given");
    // create an out dir if it doesn't exist
    let outDir = PathBuf::from(&out);
    if !outDir.exists() {
        fs::create_dir_all(&outDir).expect("Could not create out dir");
    }

    recursiveReadDir(&markdownDir, "", &out);
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

fn createDirs(dir: &str, out: &str) {
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let pathname = path.unwrap().path().display().to_string();
        let pathBuf = PathBuf::from(pathname.clone());
        if pathBuf.is_dir() {
            // create a dir in the out dir
            let outDir = [out, "/"].join("");
            let dirString = pathBuf.file_name().unwrap().to_str().unwrap().to_string();
            let dirPath = [outDir, dirString].join("");
            fs::create_dir_all(&dirPath).expect("Could not create out dir");
            println!("dirPath: {}", dirPath);
            createDirs(&pathname, &out);
        }
    }
}

fn recursiveReadDir(dir: &str, baseHtml: &str, out: &str) {
    let html = getBaseHtml(dir, baseHtml);
    let paths = fs::read_dir(dir).unwrap();
    createDirs(dir, out);

    for path in paths {
        let pathname = path.unwrap().path().display().to_string();
        let pathBuf = PathBuf::from(pathname.clone());
        if pathBuf.is_dir() {
            recursiveReadDir(&pathname, &html, &out);
        } else {
            if pathname.ends_with("styles.css") {
                // create a styles file in the out dir
                let outDir = [out, "/"].join("");
                let stylesString = "styles.css".to_string();

                let stylesPath = [outDir, stylesString].join("");
                let mut styles_file = File::create(stylesPath.clone()).expect("creation failed");
                let styles = fs
                    ::read_to_string(&pathname)
                    .expect("Should have been able to read the file");
                styles_file.write(styles.as_bytes()).expect("write failed");
                continue;
            }
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
            // println!("page: {}", page);
            // Create a file
            let outDir = [out, "/"].join("");
            let indexHtmlString = "index.html".to_string();
            let indexMarkdownString = "index.md".to_string();
            let homePage = [dir, "/", &indexMarkdownString].join("");
            let indexHtmlPath = match pathname {
                ref pathname if pathname == &homePage => [outDir, indexHtmlString].join(""),
                _ => { pathname.replace(".md", "/index.html").replace("markdown/", &outDir) }
            };

            let prettyPathDirName = indexHtmlPath.replace("index.html", "");
            // create path if it doesn't exist
            if !PathBuf::from(&prettyPathDirName).exists() {
                fs::create_dir_all(&prettyPathDirName).expect("Could not create out dir");
            }
            println!("indexHtmlPath: {}", indexHtmlPath);
            let mut data_file = File::create(indexHtmlPath.clone()).expect("creation failed");
            // Write contents to the file
            data_file.write(page.as_bytes()).expect("write failed");
        }
    }
}
