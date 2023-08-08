fn main() {
    // permutations with repetition (20 steps right, 20 steps down)
    // 40! / (20! * 20!) = 39 * 37 * 35 * 33 * 31 * 29 * 23 * 4
    let result: u64 = 39 * 37 * 35 * 33 * 31 * 29 * 23 * 4;
    println!("{}", result);
}
