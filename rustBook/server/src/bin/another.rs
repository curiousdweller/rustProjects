use server::*;
use syn::*;
use quote::*;
use procedural_macros;

fn main() {
    #[procedural_macros::config]
    pub trait Config<T>: std::fmt::Debug
    where T: Sized
    {
        
        type Event: From<T>;
    }


}

