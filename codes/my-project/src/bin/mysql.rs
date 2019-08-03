extern crate mysql;

use mysql as my;

fn main(){
    let pool = my::Pool::new("mysql://root:123456@localhost:3306/test").unwrap();
    // for mut stmt in pool.prepare("show tables").into_iter(){

    // }

    for row in pool.prep_exec("show tables", ()).unwrap(){
        println!("stmt is {:?}", row);
        //println!("table name is {:?}", mysql::from_row(row.unwrap()));
//        println!("table name is {}", (a));
//
    }
}