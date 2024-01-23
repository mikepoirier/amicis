pub mod test;

pub fn greet(name: impl AsRef<str>) -> String {
    format!("Hello, {}!", name.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_works() {
        let name = "Bugs Bunny";

        let greeting = greet(name);

        assert_eq!(greeting, "Hello, Bugs Bunny!");
    }
}
