pub trait HelloMacro {
    fn hello_macro() {
        println!("Hello my type is {}", 50 );
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
