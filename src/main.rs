fn main() {
    println!("vec = {:?}", digitize(35231));
    println!("vec = {:?}", square_sum(vec![5, 3, 4]));
    assert_eq!(are_you_playing_banjo("Martin"), "Martin does not play banjo");
    assert_eq!(are_you_playing_banjo("Rikke"), "Rikke plays banjo");
    assert_eq!(are_you_playing_banjo("bravo"), "bravo does not play banjo");
    assert_eq!(are_you_playing_banjo("rolf"), "rolf plays banjo");
    assert_eq!(string_to_number("1234"), 1234);
    assert_eq!(string_to_number("605"), 605);
    assert_eq!(string_to_number("1405"), 1405);
    assert_eq!(string_to_number("-7"), -7);

    println!("{}", abbrev_name("Bill Hegazy"));
    println!("{:?}", invert(&vec![0, 2, 3, 4,-1]));
    println!("{}", boolean_to_string(true));
    println!("time = {:?}", past(0, 1, 1));
    println!("{}", remove_char("eloquent"));
    println!("{}", two_sort(&["bitcoin", "take", "over", "the", "world", "maybe", "who", "knows", "perhaps"]));
}

fn digitize(n: u64) -> Vec<u8> {
    let mut vec = vec![0];
    if n > 0 {
        vec = n.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8).rev()
            .collect();
    }
    vec
}

fn square_sum(vec: Vec<i32>) -> i32 {
    let mut sum = 0;
    for n in vec {
        sum += n * n
    }
    sum
    // vec.iter().map(|s| s * s).sum()
}

fn are_you_playing_banjo(name: &str) -> String {
    let c = name.chars().nth(0).unwrap();
    match c {
        'r' | 'R' => name.to_owned() + " plays banjo",
        _ => name.to_owned() + " does not play banjo"
    }
    // match &name[0..1] {
    //     "R" | "r" => format!("{} plays banjo", name),
    //     _ => format!("{} does not play banjo", name)
    // }
}

fn string_to_number(s: &str) -> i32 {
    let number: i32 = s.parse().unwrap();
    number
    //   s.parse::<i32>().unwrap_or(0)
}

fn abbrev_name(name: &str) -> String {
    let vec: Vec<char> = name.split(" ").map(|n| n.chars().nth(0).unwrap()).collect();

    format!("{}.{}", vec[0].to_uppercase(), vec[1].to_uppercase())
    // name.split(' ')
    //     .map(|x| x.chars().nth(0).unwrap().to_string().to_uppercase())
    //     .collect::<Vec<_>>()
    //     .join(".")
}

fn invert(values: &[i32]) -> Vec<i32> {
    values.iter().map(|v| -v).collect::<Vec<_>>()
    // values.iter().map(|x| -x).collect()
}

fn boolean_to_string(b: bool) -> String {
    b.to_string()
}

fn past(h: i32, m: i32, s: i32) -> i32 {
    let h_to_sec = h * 60 * 60;
    let m_to_sec = m * 60;
    let time_in_milli = (h_to_sec + m_to_sec + s) * 1000;
    time_in_milli
}

pub fn remove_char(s: &str) -> String {
    let mut c = s.chars();
    c.next();
    c.next_back();
    c.as_str().to_owned()

    // s[1..s.len() - 1].to_string

    // s.chars().skip(1).take(s.chars().count() - 2).collect()

    // let mut st = s.to_string();
    // st.pop();
    // st.remove(0);
    // st
}

fn two_sort(arr: &[&str]) -> String {
    let mut a = arr.to_vec();
    a.sort();
    let mut s: String = a.first().unwrap().chars().map(|c| format!("{}***", c)).collect();
    s.truncate(s.len()-3);
    s
    // Refactor
    // let mut a = arr.to_vec();
    // a.sort();
    // a.first().unwrap().chars().map(|c| c.to_string()).collect::<Vec<_>>().join("***")

    // arr.iter().min().unwrap().chars().map(|c| c.to_string()).collect::<Vec<_>>().join("***")
}