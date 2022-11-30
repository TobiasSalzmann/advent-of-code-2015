use crate::util;

pub fn main() {
    let input = util::parse_int_lines("resources/day1.txt");
    print!("{:?}", input)
}

pub fn foo(a: i32) -> i32 {
    a + 1
}

#[cfg(test)]
mod tests {
    use crate::day1::foo;

    #[test]
    fn foo_incerements_int() {
        assert_eq!(foo(665), 666);
    }
}
