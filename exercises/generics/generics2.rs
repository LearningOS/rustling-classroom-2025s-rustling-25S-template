// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.


// 引入泛型参数 T
struct Wrapper<T> {
    value: T,
}

// 为所有类型 T 实现方法
impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        // 类型推断为 Wrapper<u32>
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        // 类型推断为 Wrapper<&str>
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
