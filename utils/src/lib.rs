pub fn load_input(name: &str) -> String {
    let tests_path = format!("../inputs/{name}");
    if let Ok(input) = std::fs::read_to_string(tests_path) {
        return input;
    }
    
    let bins_path = format!("./inputs/{name}");
    if let Ok(input) = std::fs::read_to_string(bins_path) {
        return input;
    }
    
    let current_dir = std::env::current_dir().unwrap();
    panic!("Cannot find input file '{name}'. Current dir '{current_dir:?}'");
}
