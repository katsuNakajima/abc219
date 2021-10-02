#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn main() {
    let x = parse_line!(String);
    let x_arr = x.as_str().chars();
    let n = parse_line!(i32);
    let mut s = Vec::new();
    let mut ord = vec![0; 26];
    for (i, c) in x_arr.enumerate() {
        ord[((c as u8) - b'a') as usize] = i;
    }
    for _ in 0..n {
        let a = parse_line!(String);
        s.push(a);
    }
    s.sort_by_cached_key(|s| {
        s.chars()
            .map(|c| ord[((c as u8) - b'a') as usize])
            .collect::<Vec<_>>()
    });
    for s in &s {
        println!("{}", s);
    }
}
