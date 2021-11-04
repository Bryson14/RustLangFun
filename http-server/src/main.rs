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
    let mut res = reqwest::blocking::get("http://reddit.com/")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    // println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());
    // println!("Body:\n{}", body);

    let mut a: [i32; 5] = [10, 20, 30, 40, 50];
    println!("a before: {:?}", a);
    move_int(&mut a);

    println!("a after: {:?}", a);

    Ok(())
}

fn move_int(i: &mut [i32; 5]) {
    for item in i.iter_mut() {
        *item *= 5;
    }
}
