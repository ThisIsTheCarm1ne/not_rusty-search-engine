extern crate regex;

use std::fs;
use std::env;
use regex::Regex;

struct Document {
    path: String,
    title: String,
    content: String,
}

//searches for substring
fn search_text(doc: Document, search_query: String){

    let text_str = doc.content.as_str();
    let search_query_str = search_query.as_str();

    if let Some(result) = text_str.find(search_query_str) {
        if let Some(inner) = text_str.get(result..) {
            println!("Found - {inner}");
            println!("{}", doc.path);
            println!("{}", doc.title);
            println!("{}", text_str);
        }
    }
}

//clean pages: remove html tags, extra spaces, convert chars to lower case
fn clean_text(line: String) -> String{

    let mut clean_text: &str;
    let mut clean_file = String::from("");
    let mut first_line = true;
	
    //define a regex
    let re = Regex::new(r"(?m)<\w+>(.*?)</\w+>").unwrap();
    //this removes href
    let re_link = Regex::new(r#"(?m)<a\s\w*href=[[:print:]]*">"#).unwrap();

    for single_line in line.lines(){
        let caps = re.captures(single_line);

        match caps {
            Some(text) => clean_text = text.get(1).map_or("", |m| m.as_str()),
            None => clean_text = ""
        }

        let clean_line = re_link.replace_all(clean_text, "").to_string().to_lowercase();

        if clean_line == ""{
            continue
        }

        clean_file.push_str(&clean_line);
        //checks if it is the first line, extracts the title        
        if first_line == true {
            clean_file.push_str(&"\n");
            first_line = false;
            continue;
        }

        clean_file.push_str(&" ");
    }

    return clean_file;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3{
        println!("USAGE - <command_name> <dir with files> <search query>");
        return;
    }

    let file_path = fs::read_dir(&args[1]).unwrap();


	for path in file_path {
        let path_string = path.unwrap().path().display().to_string();
        let contents = fs::read_to_string(&path_string)
            .expect("Should have been able to read the file");
        let clean_file_text = clean_text(contents);
        //grabs title(look up the clean_text function)
        let title: Vec<&str> = clean_file_text.split("\n").collect();

        let doc = Document {
            path: path_string,
            title: title[0].to_string(),
            content: clean_file_text,
        };

        search_text(doc, args[2].clone());
    }
}
