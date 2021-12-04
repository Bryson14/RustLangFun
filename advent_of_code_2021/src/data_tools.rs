use std::fs::read_to_string;

pub fn read_from_data_dir(filename: &str) -> Result<String, String> {
    let data_file = format!("../advent_of_code_2021/data/{}", filename);
    let output = match read_to_string(data_file) {
        Ok(data) => data,
        Err(e) => return Err(e.to_string()),
    };
    Ok(output)
}

#[cfg(test)]
mod test {
    use super::read_from_data_dir;

    #[test]
    fn test_good_file() {
        let data = read_from_data_dir("test_data.txt").unwrap();
        assert_eq!(data, "Here is some good data!\r\n\r\nOk");
    }

    #[test]
    fn test_missing_file() {
        let data = read_from_data_dir("some_stuff.txt");
        assert_eq!(data.is_err(), true);
    }
}
