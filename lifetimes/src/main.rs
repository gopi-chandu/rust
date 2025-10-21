fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;
    for lang in languages {
        if found {
            return lang;
        }
        if lang == current {
            found = true;
        }
    }

    languages.last().unwrap()
}

fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

fn longest_language<'a>(lang1: &'a str, lang2: &'a str) -> &'a str {
    if lang1.len() > lang2.len() {
        return lang1;
    }
    return lang2;
}

fn main() {
    let l = vec![
        String::from("python"),
        String::from("go"),
        String::from("java"),
        String::from("rust")
    ];
    let res = next_language(&l, "go");
    println!("Result 1 : {:#?} ", res);
    let res2 = last_language(&l);
    println!("Result 2 : {:#?} ", res2);
    let res3 = longest_language(&l[0], &l[1]);
    println!("Result 3 : {:#?} ", res3);
}
