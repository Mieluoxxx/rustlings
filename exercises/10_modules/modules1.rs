// TODO: 修复关于调用私有函数的编译错误。
mod sausage_factory {
    // 不要让这个模块外的任何人看到这个！
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
