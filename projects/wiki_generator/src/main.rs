use std::fs::{self, File};
use std::io::Write;
use std::path::{Path};
use pulldown_cmark::{Parser, Options, html};
use walkdir::WalkDir;
 
fn markdown_to_html(md: &str) -> String {
    let parser = Parser::new_ext(md, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
 
fn render_page(title: &str, body: &str) -> String {
    format!(r#"<!DOCTYPE html>
<html>
<head><meta charset="utf-8"><title>{}</title></head>
<body>
<nav><a href="index.html">🏠 Home</a></nav>
<hr/>
{}
</body></html>"#, title, body)
}
 
fn generate_html(input: &Path, output: &Path) {
    let markdown = fs::read_to_string(input).expect("Failed to read markdown file");
    let html = markdown_to_html(&markdown);
    let page = render_page(input.file_stem().unwrap().to_str().unwrap(), &html);
    let mut file = File::create(output).expect("Failed to create output file");
    file.write_all(page.as_bytes()).unwrap();
}
 
fn main() {
    let input_dir = Path::new("wiki");
    let output_dir = Path::new("public");
    fs::create_dir_all(output_dir).unwrap();
 
    let mut index_links = vec![];
 
    for entry in WalkDir::new(input_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
 
        if path.extension().and_then(|e| e.to_str()) == Some("md") {
            let rel_path = path.strip_prefix(input_dir).unwrap();
            let html_path = rel_path.with_extension("html");
            let output_path = output_dir.join(&html_path);
 
            fs::create_dir_all(output_path.parent().unwrap()).unwrap();
            generate_html(path, &output_path);
 
            let link = html_path.to_string_lossy().replace("\\", "/");
            index_links.push(format!(r#"<li><a href="{}">{}</a></li>"#, link, link));
        }
    }
 
    // Write index.html
    let index_html = format!(
        "<h1>📘 My Wiki</h1><ul>{}</ul>",
        index_links.join("\n")
    );
    let mut file = File::create(output_dir.join("index.html")).unwrap();
    file.write_all(render_page("My Wiki", &index_html).as_bytes()).unwrap();
 
    println!("✅ Wiki generated in ./public/");
}
