fn readline() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim_end().to_owned();
    return s;
}

fn readline_split() -> Vec<String> {
    let s = readline();
    let s = s.split_whitespace();
    let ret = s.map(|s| s.to_string()).collect::<Vec<_>>();
    return ret;
}

fn readline_numbers<T>() -> Vec<T>
where
    T: std::str::FromStr+Copy,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    return readline_split().iter().map(|si| si.parse::<T>().unwrap()).collect::<Vec<T>>();
}
