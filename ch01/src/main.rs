// 创建一个Rust工程，

// （1）添加一个一层子模块，循环打印从’a’~’Z’ 之间的所有字符

// （2）添加一个二层子模块，循环打印从’A’~’z’ 之间的所有字符

// （3）使用Cargo编译运行此工程

// （需要自行发现其中的细节，一个考点是：ascii码字符的顺序）


mod fisrt_layer {

    pub fn print_upper_a_to_z() {
        let mut char_vec = Vec::new();
        for c in 'A'..='Z' {
            char_vec.push(c);
        }
        for c in 'a' ..='z' {
            char_vec.push(c);
        }

        for c in char_vec.clone() {
            print!("{} ", c);
        }
        println!();
    }
    pub mod secend_layer {
        
        pub fn print_a_to_upper_z() {
            let mut char_vec = Vec::new();
            
            for c in 'a' ..='z' {
                char_vec.push(c);
            }

            for c in 'A'..='Z' {
                char_vec.push(c);
            }
           
    
            for c in char_vec.clone() {
                print!("{} ", c);
            }
            println!();
          
        }
    }
}




fn main() {
    fisrt_layer::print_upper_a_to_z();
    fisrt_layer::secend_layer::print_a_to_upper_z();
}
