fn main() {
    // You can optionally experiment here.
// 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // Tuple indexing syntax.
        let second = numbers.1;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
