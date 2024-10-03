
fn main() {
    let x = read_int();
    println!("{}", x);
    let name = read_str();
    println!("{}", name);
}

fn read_int() -> i128 {
    let mut num = String::new();   
    std::io::stdin().read_line(&mut num).expect("error");
    return num.trim().parse::<i128>().expect("error");
}

fn read_str() -> String {
    let mut result = String::new();   
    std::io::stdin().read_line(&mut result).expect("error");
    return result;
}