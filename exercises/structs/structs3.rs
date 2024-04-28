// structs3.rs
//
// Structs contain data, but can also have logic. In this exercise we have
// defined the Package struct and we want to test some logic attached to it.
// Make the code compile and the tests pass!
//
// Execute `rustlings hint structs3` or use the `hint` watch subcommand for a
// hint.



struct Package {
    grams: i32,
}

impl Package {
    fn is_international(&self) -> bool {
        self.grams > 200
    }

    fn get_fees(&self, cents_per_gram: i32) -> i32 {
        self.grams * cents_per_gram
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_international() {
        let domestic_package = Package { grams: 100 };
        let international_package = Package { grams: 250 };

        assert!(!domestic_package.is_international());
        assert!(international_package.is_international());
    }

    #[test]
    fn test_get_fees() {
        let package = Package { grams: 150 };
        let cents_per_gram = 30;

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}

