use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = num[0] as usize;
#[warn(unused_variables)]
    let m = num[1];
    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut c: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut a: Vec<usize> = vec![0; n];
    let mut b: Vec<i32> = vec![0; n];
    for i in 0..n {
        input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let num: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        a[i] = num[0] as usize;
        b[i] = num[1];
    }
    
    let mut total = 0;
    for i in 0..n {
        if b[i] < c[a[i] - 1] {
            total += b[i];
            c[a[i] - 1] -= b[i];
        } else {
            total += c[a[i] - 1];
            c[a[i] - 1] = 0;
        }
    }
    println!("{}", total);
}
