use femark::{ HTMLOutput, process_markdown_to_html };
// use std::env;
use std::fs;
use crate::fs::File;
use std::io::Write;
// use std::fs::metadata;
use std::path::PathBuf;
// const baseDir: &str = "./markdown";

fn main() {
    let markdown_dir = std::env::args().nth(1).expect("no markdown directory given");
    let out = std::env::args().nth(2).expect("no out dir given");
    // create an out dir if it doesn't exist
    let out_dir = PathBuf::from(&out);
    if !out_dir.exists() {
        fs::create_dir_all(&out_dir).expect("Could not create out dir");
    }

    recursive_read_dir(&markdown_dir, "", &out);
}

fn get_base_html<'a>(dir: &'a str, base_html: &'a str) -> String {
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let pathname = path.unwrap().path().display().to_string();
        let path_buf = PathBuf::from(pathname.clone());
        if path_buf.is_dir() {
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
    return base_html.to_string();
}

fn create_dirs(dir: &str, out: &str) {
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        let pathname = path.unwrap().path().display().to_string();
        let path_buf = PathBuf::from(pathname.clone());
        if path_buf.is_dir() {
            // create a dir in the out dir
            let out_dir = [out, "/"].join("");
            let dir_string = path_buf.file_name().unwrap().to_str().unwrap().to_string();
            let dir_path = [out_dir, dir_string].join("");
            fs::create_dir_all(&dir_path).expect("Could not create out dir");
            create_dirs(&pathname, &out);
        }
    }
}

fn recursive_read_dir(dir: &str, base_html: &str, out: &str) {
    let html = get_base_html(dir, base_html);
    let paths = fs::read_dir(dir).unwrap();
    create_dirs(dir, out);

    for path in paths {
        let pathname = path.unwrap().path().display().to_string();
        let path_buf = PathBuf::from(pathname.clone());
        if path_buf.is_dir() {
            recursive_read_dir(&pathname, &html, &out);
        } else {
            if pathname.ends_with("styles.css") {
                // create a styles file in the out dir
                let out_dir = [out, "/"].join("");
                let styles_string = "styles.css".to_string();

                let styles_path = [out_dir, styles_string].join("");
                let mut styles_file = File::create(styles_path.clone()).expect("creation failed");
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
            let Ok(HTMLOutput { content, toc: _, .. }) = process_markdown_to_html(&input) else {
                println!("Error");
                return;
            };
            let page = html.replace("<!-- MARKDOWN -->", &content);
            // println!("page: {}", page);
            // Create a file
            let out_dir = [out, "/"].join("");
            let index_html_string = "index.html".to_string();
            let index_markdown_string = "index.md".to_string();
            let home_page = [dir, "/", &index_markdown_string].join("");
            let index_html_path = match pathname {
                ref pathname if pathname == &home_page => [out_dir, index_html_string].join(""),
                _ => { pathname.replace(".md", "/index.html").replace("markdown/", &out_dir) }
            };

            let pretty_path_dir_name = index_html_path.replace("index.html", "");
            // create path if it doesn't exist
            if !PathBuf::from(&pretty_path_dir_name).exists() {
                fs::create_dir_all(&pretty_path_dir_name).expect("Could not create out dir");
            }
            println!("Writing file: {}", index_html_path);
            let mut data_file = File::create(index_html_path.clone()).expect("creation failed");
            // Write contents to the file
            data_file.write(page.as_bytes()).expect("write failed");
        }
    }
}
