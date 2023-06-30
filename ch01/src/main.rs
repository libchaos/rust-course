
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
