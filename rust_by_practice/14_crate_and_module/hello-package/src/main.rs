mod front_of_house;
use hello_package::eat_at_restaurant;
use std::fmt::Result as FmtResult;
use std::io::Result as IoResult;
use std::collections::*;


fn main() {
    assert_eq!(front_of_house::hosting::seat_at_table(), "sit down please");
    assert_eq!(eat_at_restaurant(), "yummy yummy!");

    let _c1:HashMap<&str, i32> = HashMap::new();
    let mut c2 = BTreeMap::new();
    c2.insert(1, "a");
    let _c3: HashSet<i32> = HashSet::new();

    assert_eq!(hello_package::hosting::seat_at_table(), "sit down please");
    assert_eq!(hello_package::eat_at_restaurant(),"yummy yummy!");
}