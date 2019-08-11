
pub mod instrument {
    pub mod woodwind {
        pub fn clarinet() {
            // 函数体
            println!(" sound - instrument - woodwind- clarinet ");
        }
    }

    pub fn clarinet() {
        super::breathe_in();
    }
}

fn breathe_in() {
    // 函数体
}

mod voice {}
