use std::io::Error;
fn print_elements(elements: &[String]) {
    // for e in elements {
    //     println!("{:#?}", e);
    // }

    elements
        .iter()
        .map(|x| format!("{} {}", x, x))
        .for_each(|x| println!("{:#?}", x))
}

fn shorten_strings(elements: &mut [String]) {
    /* let ans = elements
         .iter_mut()
         .for_each(|x|x.truncate(1));*/

    for mut x in elements {
        x.truncate(1);
    }
    return;
}
fn to_uppercase(elements: &[String]) -> Vec<String> {
    /*let res = elements
        .iter()
        .map(|x| x.to_uppercase())
        .collect();*/

    /*let res = elements
        .iter()
        .map(|x| x.to_uppercase())
        .collect::<Vec<String>>();*/

    let res = elements
        .iter()
        .map(|x| x.to_uppercase())
        .collect::<Vec<_>>();
    return res;
}

fn move_elements(a: Vec<String>, b: &mut Vec<String>) {
    a.into_iter().for_each(|x| b.push(x))
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    let res = elements
        .iter()
        .map(|x|
            x
                .chars()
                .map(|x| x.to_string())
                .collect()
        )
        .collect();

    return res;
}
// fn find_color_or(elements: &[String], color: String) {
//     let res: Vec<_> = elements
//         .into_iter()
//         .filter(|x| x.to_string() == color)
//         .collect();

//     println!("{:#?}", res)
// }

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    let res = elements
        .iter()
        .find(|x| x.contains(search)) // iterator consumer is find
        .map_or(String::from(fallback), |el| el.to_string());

    return res;
}
fn main() {
    println!("Hello, world!");

    let mut colors = vec![String::from("red"), String::from("green"), String::from("blue")];

    let mut color_iter = colors.iter();

    // println!("{:#?}", color_iter.next());
    // println!("{:#?}", color_iter.next());
    // println!("{:#?}", color_iter.next());
    // println!("{:#?}", color_iter.next());

    // print_elements(&colors);

    // shorten_strings(&mut colors);

    // let uppercase = to_uppercase(&colors);
    // println!("{:#?}", uppercase);

    // let mut res = vec![];
    // move_elements(colors, &mut res);
    // println!("{:#?}", res);

    // let res = explode(&colors);
    // println!("{:#?}", res);

    let res = find_color_or(&colors, &String::from("re"), &String::from("Orange"));
    println!("{:#?}", res);
}
