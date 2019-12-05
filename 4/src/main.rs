fn main() {
    let res = (152085..670283)
        .map(|x| x.to_string())
        .map(|x| x.as_str().chars().map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .filter(|x| {
            let mut iter = x.iter().peekable();
            while let Some(z) = iter.next() {
                if let Some(y) = iter.peek() {
                    if z == *y {
                        return true
                    }
                }
            }
            false
        })
    .filter(|x| {
        let mut y = x.clone();
        y.sort();

        *x == y
    })
    .collect::<Vec<Vec<u32>>>();
    println!("{:?}", res.len());
}
