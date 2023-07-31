use std::fs;

pub fn get(path: &str) -> (String, usize) {
    let path_root = "/home/tasty/Development/rust-book/src/chapters/c20_multithreaded_web_server/templates/html";
    let full_path = format!("{}/{}", path_root, path);
    println!("Got `{path}`, getting `{full_path}`");

    let contents = fs::read_to_string(full_path).unwrap();
    let length = contents.len();

    println!("Flie contents are: {contents}");

    (contents, length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        // This is a shitty test since it will need to be updated any time hello.html is changed
        // And that file won't be used anyway, so this a bad test a second time
        let (html, length) = get("hello.html");

        assert_eq!(length, 99);
        assert_eq!(html.len(), length);
    }
}
