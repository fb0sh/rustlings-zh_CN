// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
// 当对集合中的元素执行操作时，迭代器是必不可少的。
// 本模块帮助你熟悉使用迭代器的结构，以及如何遍历可迭代集合中的元素。

fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterators() {
        let my_fav_fruits = ["banana", "custard apple", "avocado", "peach", "raspberry"];

        // TODO: Create an iterator over the array.
        // TODO: 创建一个用于数组的迭代器。
        let mut fav_fruits_iterator = todo!();

        assert_eq!(fav_fruits_iterator.next(), Some(&"banana"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: Replace `todo!()` //TODO: 替换`todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"avocado"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: Replace `todo!()` //TODO: 替换`todo!()`
        assert_eq!(fav_fruits_iterator.next(), Some(&"raspberry"));
        assert_eq!(fav_fruits_iterator.next(), todo!()); // TODO: Replace `todo!()` //TODO: 替换`todo!()`
    }
}
