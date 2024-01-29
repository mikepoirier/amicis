pub mod test;

pub trait Greetable {
    fn greet(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Name(String);

impl Name {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl Greetable for Name {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.0)
    }
}

impl<T> Greetable for T
where
    T: AsRef<str>,
{
    fn greet(&self) -> String {
        format!("Hello, {}!", self.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_greet_works() {
        let name = Name::new("Bugs Bunny");

        let greeting = name.greet();

        assert_eq!(greeting, "Hello, Bugs Bunny!");
    }

    #[test]
    fn str_greet_works() {
        let name = "Bugs Bunny";

        let greeting = name.greet();

        assert_eq!(greeting, "Hello, Bugs Bunny!");
    }

    #[test]
    fn string_greet_works() {
        let name = String::from("Bugs Bunny");

        let greeting = name.greet();

        assert_eq!(greeting, "Hello, Bugs Bunny!");
    }
}
