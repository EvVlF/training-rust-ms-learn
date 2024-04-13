fn main() {
    {
        let mascot = String::from("ferris");
        let ferris = mascot;
        println!("{}", mascot) // We'll try to use mascot after we've moved ownership of the string data from mascot to ferris.
    }
}