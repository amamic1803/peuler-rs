fn main() {
    let mut curr_triangle: u64 = 286;
    loop {
        let triangle_value: u64 = (curr_triangle * (curr_triangle + 1)) / 2;
        if is_pentagonal(triangle_value) && is_hexagonal(triangle_value) {
            println!("{}", triangle_value);
            break;
        }
        curr_triangle += 1;
    }
}

fn is_pentagonal(num: u64) -> bool {
    return if (1.0 + (1.0 + 24.0 * (num as f64)).sqrt()) % 6.0 == 0.0 {
        true
    } else {
        false
    }
}

fn is_hexagonal(num: u64) -> bool {
    return if (1.0 + (1.0 + 8.0 * (num as f64)).sqrt()) % 4.0 == 0.0 {
        true
    } else {
        false
    }
}