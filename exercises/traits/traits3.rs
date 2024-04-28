// traits3.rs
//
// Your task is to implement the Licensed trait for both structures and have
// them return the same information without writing the same function twice.
//
// Consider what you can add to the Licensed trait.
//
// Execute `rustlings hint traits3` or use the `hint` watch subcommand for a
// hint.



pub trait Licensed {
    fn licensing_info(&self) -> String;

    // Add a default function to provide default licensing information
    fn default_licensing_info(&self) -> String {
        String::from("Some information")
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {
    // Implement licensing_info using the default_licensing_info function
    fn licensing_info(&self) -> String {
        format!("{} for version {}", self.default_licensing_info(), self.version_number)
    }
}

impl Licensed for OtherSoftware {
    // Implement licensing_info using the default_licensing_info function
    fn licensing_info(&self) -> String {
        format!("{} for version {}", self.default_licensing_info(), self.version_number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), format!("{} for version {}", licensing_info, 1));
        assert_eq!(other_software.licensing_info(), format!("{} for version {}", licensing_info, "v2.0.0"));
    }
}

