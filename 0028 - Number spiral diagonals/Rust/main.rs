fn main() {
    // calculate top-right of every "layer" of spiral -> it is square of layer's side length -> calculate other corners by subtracting length of layer sides
    // formula for each "layer" simplifies to 4x^2 - 6x + 6
    let mut result: u64 = 1;

    for i in ((3 as u64)..1002).step_by(2) {
        result += (4 * i.pow(2)) - (6 * i) + 6
    }

    println!("{}", result);
}
