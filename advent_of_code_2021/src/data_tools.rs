use std::fs::read_to_string;
/// The project is set up as follows...
/// ```text
/// C:.
/// ├───.vscode
/// ├───data
/// ├───src
/// └───target
/// ```
/// All the modules are under source, and `read_from_data_dir` provides an easy way to read in text data from the data folder.
pub fn read_from_data_dir(filename: &str) -> Result<String, String> {
    let data_file = format!("../advent_of_code_2021/data/{}", filename);
    let output = match read_to_string(data_file) {
        Ok(data) => data,
        Err(e) => return Err(e.to_string()),
    };
    Ok(output)
}

/// This turns a String from a line separated .txt file into a vector of numbers
/// // assert_eq!(string_to_vec_i32(String::from("1\n2\r\n3")).unwrap(), vec![1, 2, 3]);
pub fn string_to_vec_i32(s: String) -> Result<Vec<i32>, std::string::ParseError> {
    let output: Vec<i32> = s
        .lines()
        .map(|x| x.parse().expect("oh no, bad parsing"))
        .collect();
    Ok(output)
}

#[cfg(test)]
mod test {
    use super::{read_from_data_dir, string_to_vec_i32};

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

    #[test]
    fn test_parse_vec() {
        let s = String::from("1\r\n2\r\n3");
        let ans: Vec<i32> = string_to_vec_i32(s).unwrap();
        assert_eq!(ans, vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_vec2() {
        let s = String::from("1\n2\n3");
        let ans: Vec<i32> = string_to_vec_i32(s).unwrap();
        assert_eq!(ans, vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_vec3() {
        let s = String::from("1\n2\r\n3");
        let ans: Vec<i32> = string_to_vec_i32(s).unwrap();
        assert_eq!(ans, vec![1, 2, 3]);
    }

    #[test]
    #[should_panic]
    fn test_parse_vec_bad() {
        let s = String::from("1\r\n2\r\n3\nba");
        let ans = string_to_vec_i32(s);
        match ans {
            Ok(_) => unreachable!(),
            Err(_e) => assert_eq!(true, true),
        }
    }
}
