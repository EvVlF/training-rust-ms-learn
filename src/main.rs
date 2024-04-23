trait AsJson {
    fn as_json(&self) -> String;
}

fn send_data_as_json(value: &impl AsJson) {
    println!("Sending JSON data to server...");
    println!("-> {}", value.as_json());
    println!("Done!\n");
}

fn main() {}