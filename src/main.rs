
fn main() {
    //
}






fn read_usize() -> usize {
    let mut num = String::new();   
    std::io::stdin().read_line(&mut num).expect("error");
    return num.trim().parse::<usize>().expect("error");
}

fn read_isize() -> isize {
    let mut num: String = String::new();
    std::io::stdin().read_line(&mut num).expect("error");
    return num.trim().parse::<isize>().expect("error");
}

// int unsigned

fn read_u8() -> u8  {
    let mut num = String::new();   
    std::io::stdin().read_line(&mut num).expect("error");
    return num.trim().parse::<u8>().expect("error");
}

fn read_u16() -> u16 {
    let mut num = String::new();   
    std::io::stdin().read_line(&mut num).expect("error");
    return num.trim().parse::<u16>().expect("error");
}

fn read_u32() -> u32 {
    let mut num = String::new();   
    std::io::stdin().read_line(&mut num).expect("error");
    return num.trim().parse::<u32>().expect("error");
}

fn read_u64() -> u64 {
    let mut num = String::new();   
    std::io::stdin().read_line(&mut num).expect("error");
    return num.trim().parse::<u64>().expect("error");
}

fn read_u128() -> u128 {
    let mut num = String::new();   
    std::io::stdin().read_line(&mut num).expect("error");
    return num.trim().parse::<u128>().expect("error");
}


// int unsigned





fn read_str() -> String {
    let mut result = String::new();   
    std::io::stdin().read_line(&mut result).expect("error");
    return result;
}

// float

fn read_f32() -> f32 {
    let mut num = String::new();   
    std::io::stdin().read_line(&mut num).expect("error");
    return num.trim().parse::<f32>().expect("error");
}

fn read_f64() -> f64 {
    let mut num = String::new();   
    std::io::stdin().read_line(&mut num).expect("error");
    return num.trim().parse::<f64>().expect("error");
}

// float



// int signed


fn read_i8() -> i8 {
    let mut num = String::new();   
    std::io::stdin().read_line(&mut num).expect("error");
    return num.trim().parse::<i8>().expect("error");
}

fn read_i16() -> i16 {
    let mut num = String::new();   
    std::io::stdin().read_line(&mut num).expect("error");
    return num.trim().parse::<i16>().expect("error");
}

fn read_i32() -> i32 {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).expect("error");
    return num.trim().parse::<i32>().expect("error");
}

fn read_i64() -> i64 {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).expect("error");
    return num.trim().parse::<i64>().expect("error");
}

fn read_i128() -> i128 {
    let mut num = String::new();
    std::io::stdin().read_line(&mut num).expect("error");
    return num.trim().parse::<i128>().expect("error");
}
 
// int signed