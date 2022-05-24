mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
mod and_another{
    mod another_module {
        pub fn eat_at_restaurant() {
            // Absolute path
            crate::front_of_house::hosting::add_to_waitlist();
        
            // Relative path
        }
    }
}

// This works because items in a parent module can’t use 
//the private items inside child modules, 
//but items in child modules can use the items in their ancestor modules.
//eat_at_restraunt is an item in a child module of crate,
//front_of_house is an item defined in the ancestor module crate
//Therefore front_of_house can be used in eat_at_restraunt