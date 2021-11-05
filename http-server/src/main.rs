use error_chain::error_chain;
use reqwest;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let mut res = reqwest::blocking::get("http://www.google.com/")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}

fn move_int(i: &mut Vec<i32>) {
    for item in i.iter_mut() {
        print!("{} -> {:p}  ", item, item);
        *item *= 5;
    }
}
