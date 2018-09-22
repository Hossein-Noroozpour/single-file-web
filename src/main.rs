use std::path::Path;
use std::fs;
use std::env;
use std::io::{ Read, Write };

fn main() {
    let mut log_file = fs::File::create("log.txt").unwrap();
    let exe_path = match env::current_exe() {
        Ok(p) => { p },
        Err(e) => {
            writeln!(log_file, "Error {} in current_exe.", e).unwrap();
            panic!("");
        },
    };
    writeln!(log_file, "Program {:?} started.", exe_path).unwrap();
    let exe_dir = match exe_path.parent() {
        Some(d) => { d },
        None => {
            writeln!(log_file, "Error in fetching directory of exe.").unwrap();
            panic!("");
        },
    };
    writeln!(log_file, "Program directory is {:?}.", exe_dir).unwrap();
    let mut css = String::new();
    let mut js = String::new();
    path_reader(exe_dir, &mut css, &mut js);
    let mut js_file = fs::File::create("total.js").unwrap();
    writeln!(js_file, "{}", js);
    let mut css_file = fs::File::create("total.css").unwrap();
    writeln!(css_file, "{}", css);
}

fn path_reader(p: &Path, css: &mut String, js: &mut String) {
    if p.is_dir() {
        for e in fs::read_dir(p).unwrap() {
            if let Ok(e) = e {
                path_reader(&e.path(), css, js);
            }
        }
    } else if p.is_file() {
        if let Some(s) = p.extension() {
            if s == "js" {
                fs::File::open(p).unwrap().read_to_string(js).unwrap();
                js.push('\n');
            } else if s == "css" {
                fs::File::open(p).unwrap().read_to_string(css).unwrap();
                css.push('\n');
            }
        }
    }
}
