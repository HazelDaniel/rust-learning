pub mod conversion {
    #[derive(Debug)]
    pub struct Animal;

    pub fn reverse_string(string: &str) -> String {
        let mut reversed = String::new();

        for s in string.chars().rev() {
            reversed.push(s);
        }
        reversed
    }

    pub fn upper_string(string: &str) -> String {
        let mut upper = String::new();
        for s in string.chars() {
            upper.push_str(&s.to_uppercase().to_string());
        }
        upper
    }
}
