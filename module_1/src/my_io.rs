pub fn read_entry() -> String {
    let mut message = String::new();
    let stdin_reader = std::io::stdin();
    let read_res = stdin_reader.read_line(&mut message);
    
    if read_res.is_err() {
        println!("Error reading from stdin: {:?}", read_res.err());
    }

    message.trim().to_string()
}