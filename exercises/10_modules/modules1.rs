// TODO: Fix the compiler error about calling a private function.
// TODO: 修复调用私有函数的编译错误
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    // 不要让模块外的任何人看到这个！
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
