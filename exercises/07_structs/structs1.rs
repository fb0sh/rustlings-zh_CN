struct ColorRegularStruct {
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?
    // 定义一个结构体 Color，添加测试 `regular_structs` 期望的字段。
    // RGB 颜色的最小值是 0，最大值是 255。
}

// TODO: 添加测试 `tuple_structs` 期望的字段
struct ColorTupleStruct(/* TODO: Add the fields that the test `tuple_structs` expects */);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can optionally experiment here.
    // 你可以在这里进行可选的实验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        // TODO: 实例化一个普通结构体（regular struct）
        // let green =

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        // TODO: 实例化一个元组结构体（tuple struct）
        // let green =

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        // TODO: 实例化一个单元结构体（unit struct）
        // let unit_struct =
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
