use std::str::FromStr;

fn programm(args: Vec<String>) {
    let x = args
        .iter()
        .nth(1)
        .map(|s: &String| i32::from_str(s))
        .unwrap_or(Ok(0))
        .unwrap_or_else(|err| {
            println!("Error while parsing: {}", err);
            42
        });

    for i in x..10 {
        println!("{i}");
    }
}

fn main() {
    let args = std::env::args().collect();
    programm(args)
}

#[cfg(test)]
mod test {
    use super::programm;

    #[test]
    fn test_my_programm() {
        let test_vec = ["foo", "1"]
            .into_iter()
            .map(String::from)
            .collect();
        programm(test_vec);
    }

    #[test]
    fn test_my_programm_42() {
        let test_vec = ["foo", "42"]
            .into_iter()
            .map(String::from)
            .collect();
        programm(test_vec);
    }
}
