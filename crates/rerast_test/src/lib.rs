pub fn get_file_contents(filename: &std::path::Path) -> std::io::Result<String> {
    let mut result = String::new();
    use std::io::Read;
    r#try!(r#try!(std::fs::File::open(filename)).read_to_string(&mut result));
    Ok(result)
}
