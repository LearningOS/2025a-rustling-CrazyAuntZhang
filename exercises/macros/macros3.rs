// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

mod macros {
    // 使用#[macro_export]标记宏，允许它被重导出
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    
    // 现在可以正确地重导出宏了
    pub use my_macro;
}

// 导入模块中的宏


fn main() {
    my_macro!();
}
    