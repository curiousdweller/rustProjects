#[cfg_attr(target_os = "mac", path = "linux.rs")]
pub mod hello {
    pub fn hit_there() {
        print!("Hello, world!");
    }

}

fn main() {
}