fn main() {
// Declare vector, initialize with three values
    let mut index_vec = vec![15, 3, 46];

    // Access vector with out-of-bounds index
    let beyond = index_vec[10];
    println!("{}", beyond);
}