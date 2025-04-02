// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.


// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // 处理非法时间输入（超过23）
    if time_of_day > 23 {
        return None;
    }

    // 判断冰淇淋数量逻辑
    if time_of_day >= 22 {
        Some(0)  // 22点（含）后没有冰淇淋
    } else {
        Some(5)  // 22点前有5个冰淇淋
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        let icecreams = maybe_icecream(12);
        // 使用 unwrap() 获取 Option 内部值
        assert_eq!(icecreams.unwrap(), 5);
    }
}
