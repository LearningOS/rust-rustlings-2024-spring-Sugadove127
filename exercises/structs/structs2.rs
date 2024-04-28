// structs2.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs2` or use the `hint` watch subcommand for a
// hint.



#[derive(Debug, PartialEq)]
struct OrderTemplate {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn main() {
    let order_template = OrderTemplate {
        name: String::from("Hacker in Rust"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: true,
        made_by_email: true,
        item_number: 5,
        count: 1,
    };

    // Define `your_order` as an instance of `OrderTemplate`
    let your_order = OrderTemplate {
        name: String::from("Hacker in Rust"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: true,
        made_by_email: true,
        item_number: 5,
        count: 1,
    };

    assert_eq!(your_order.name, "Hacker in Rust");
    assert_eq!(your_order.year, order_template.year);
    assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
    assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
    assert_eq!(your_order.made_by_email, order_template.made_by_email);
    assert_eq!(your_order.item_number, order_template.item_number);
    assert_eq!(your_order.count, 1);
}


