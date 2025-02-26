mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}

        fn complain() {} 
    }
}

pub fn eat_at_restaurant() {
    // Call add_to_waitlist with **absolute path**:
    crate::front_of_house::hosting::add_to_waitlist();

    // Call with **relative path** 
    front_of_house::serving::take_payment();
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // FILL in the blank in three ways
        //1. using keyword `super`
        //2. using absolute path
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}