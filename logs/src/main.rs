use std::fs;
use std::io::Error;

fn extract_errors(text: &str) -> Vec<String> {
    let lines = text.split("\n");
    let mut result = vec![];
    for line in lines {
        if line.starts_with("ERROR") {
            result.push(line.to_string());
        }
    }

    result
}

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("../random.txt")?;
    let error_logs = extract_errors(&text);
    fs::write("../errors.txt", error_logs.join("\n"))?;
    Ok(())
}

// fn main() {
//     let text = fs::read_to_string("../random.txt").expect("Failed to read txt");

//     let error_logs = extract_errors(&text);

//     fs::write("../errors.txt", error_logs.join("\n")).expect("failed to write txt")
// }

// fn main() {
//     let data = fs::read_to_string("../random.txt");
//     let mut error_logs = vec![];
//     match data {
//         Ok(value) => {
//             println!("Successfully read the txt");
//             error_logs = extract_errors(value.as_str());
//             // println!("{:#?}",error_logs)
//             // println!("{:#?}",error_lines)
//         }
//         Err(error) => println!("Error reading the file"),
//     }

//     println!("{:#?}", error_logs);

//     // println!("{:#?}", data);
//     // match divide(6.0, 3.0) {
//     //     Ok(value_divided) => {
//     //         println!("Value {}", value_divided);
//     //     }
//     //     Err(error_value) => {
//     //         println!("Error {}", error_value);
//     //     }
//     // }
// }

// fn divide(a: f64, b: f64) -> Result<f64, Error> {
//     if b == 0.0 {
//         return Err(Error::other("Cannot divide by zero."));
//     }
//     Ok(a / b)
// }
