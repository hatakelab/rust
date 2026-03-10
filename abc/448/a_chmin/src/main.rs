use std::io;

fn main() {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    let input1_num: Vec<u8> = input1.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = input1_num[0] as usize;
    let mut x = input1_num[1];
    input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    let a: Vec<u8> = input1.split_whitespace().map(|x| x.parse().unwrap()).collect();
    for i in 0..n {
        if a[i] < x {
            x = a[i];
            println!("1");
        } else {
            println!("0");
        }
    }
}
