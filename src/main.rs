fn main() {
    {
        let mascot = String::from("ferris");
        // transfer ownership of mascot to the variable ferris.
        let ferris = mascot;
    }
// ferris is dropped here. The string data memory will be freed here.
}