use my_project::plant;

extern crate mysql;

fn main() {
    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    // 如果将如下行取消注释代码将无法编译:
    //println!("The ID is {}", v.id);
}
