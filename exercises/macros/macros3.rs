// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

// smlj.. https://doc.rust-lang.org/book/ch19-06-macros.html?highlight=macro_export#declarative-macros-with-macro_rules-for-general-metaprogramming

mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
