// This exercise explores the `Cow` (Clone-On-Write) smart pointer. It can
// enclose and provide immutable access to borrowed data and clone the data
// lazily when mutation or ownership is required. The type is designed to work
// with general borrowed data via the `Borrow` trait.
// 本练习探讨 `Cow` (写时克隆) 智能指针。它可以封装并提供对借用数据的
// 不可变访问，并在需要可变或所有权时惰性地克隆数据。该类型被设计为通过
// `Borrow` trait 与通用借用数据一起工作。

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // Clones into a vector if not already owned.
            // 如果它还不拥有所有权，则克隆到一个向量中。
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // Clone occurs because `input` needs to be mutated.
        // 发生克隆是因为 `input` 需要被修改。
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // No clone occurs because `input` doesn't need to be mutated.
        // 没有发生克隆，因为 `input` 不需要被修改。
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        // TODO: Replace `todo!()` with `Cow::Owned(_)` or `Cow::Borrowed(_)`.
        // TODO: 将 `todo!()` 替换为 `Cow::Owned(_)` 或 `Cow::Borrowed(_)`。
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_no_mutation() {
        // We can also pass `vec` without `&` so `Cow` owns it directly. In this
        // case, no mutation occurs (all numbers are already absolute) and thus
        // also no clone. But the result is still owned because it was never
        // borrowed or mutated.
        // 我们也可以不带 `&` 传递 `vec`，这样 `Cow` 就能直接拥有它。
        // 在这种情况下，没有发生可变（所有数字都已是绝对值），因此也没有克隆。
        // 但结果仍然是拥有的，因为它从未被借用或可变。
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: Replace `todo!()` with `Cow::Owned(_)` or `Cow::Borrowed(_)`.
        // TODO: 将 `todo!()` 替换为 `Cow::Owned(_)` 或 `Cow::Borrowed(_)`。
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_mutation() {
        // Of course this is also the case if a mutation does occur (not all
        // numbers are absolute). In this case, the call to `to_mut()` in the
        // `abs_all` function returns a reference to the same data as before.
        // 当然，如果确实发生了可变（并非所有数字都是绝对值），情况也是如此。
        // 在这种情况下，`abs_all` 函数中对 `to_mut()` 的调用返回对与之前
        // 相同的数据的引用。
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: Replace `todo!()` with `Cow::Owned(_)` or `Cow::Borrowed(_)`.
        // TODO: 将 `todo!()` 替换为 `Cow::Owned(_)` 或 `Cow::Borrowed(_)`。
        assert!(matches!(input, todo!()));
    }
}
