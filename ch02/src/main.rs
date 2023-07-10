// 值是资源，所有权是对资源拥有的权利
// 所有权基本规则：
//     1 一个值拥有一个所有者
//     2 任何一个时刻，一个值只有一个所有者
//     3 当所有者所在作用域结束的时候，值会被释放
            // - 作用域是被花括号包围的区域
// RALL
    // 栈->堆 自动释放

// - 占用固定内存   栈
// - 占用不固定内存  堆

// move
//   当数据存放在堆上时，将一个变量赋值给另一个变量，意味着把堆上所有权转移给新的所有者，堆上的数据本身没有被复制，原来的所有者不在拥有数据，并且标记为空，所有权的Move语义

// copy
//   当数据存放在栈上时，将一个变量赋值给另一个变量，意味着把栈上的数据复制了一份给新的所有者，原来所有者仍然拥有原来的值，这就是值得Copy语义
    // 1. 对于固定大小的类型，将一个变量赋值给另一个变量时，实际上是为新变量开辟了新的内存空间，并把值拷贝过来，这就是copy。
    // 最终结果是产生了一个新的所有者以及对应的值，并且新变量和原变量之间没有任何关系


// borrow
// mut borrow

// 借用规则：
    // 1. 同一个作用域中，一个资源只有一个可变借用(&mut T)，也就说拥有可变借用后就不能在拥有不可变借用(&T)
    // 2. 同一个作用域中，一个资源可有有多个不可变借用(&T)
    // 3. 借用在它离开作用域后释放

    use std::vec;



fn move_value() {
    let num = 1;
    let num2 = num;
    println!("num = {}, num2 = {}", num, num2);
}


fn borrow_value() {
    fn borrow(s: &String) {
        println!("s = {}", s);
    }
    let s = String::from("next borrow");
    borrow(&s);

    
}

fn copy_value() {
    let s = String::from("hello, world");
    let s1 = s.clone();
    println!("s1 = {}, s2 = {}", s1, s);
}

fn mut_borrow_value() {
    fn mut_borrow(s: &mut String) {
        s.push_str(", world");
    }


    let mut s = String::from("hello");
    mut_borrow(&mut s);
    println!("s = {}", s);
}


fn data_type() {
    let mut s = "rust";
    let size_of_s = std::mem::size_of_val(s);
    let ptr_of_s = s.as_ptr();

    println!("s = {}, size_of_s = {}, ptr_of_s = {:p}", s, size_of_s, ptr_of_s);

    s = "go";

    let size_of_s = std::mem::size_of_val(s);
    let ptr_of_s = s.as_ptr();
    println!("s = {}, size_of_s = {}, ptr_of_s = {:p}", s, size_of_s, ptr_of_s);


    let ptr_of_rust = "rust".as_ptr();
    println!("ptr_of_rust = {:p}", ptr_of_rust);


    let mut  s_string = String::from("h");
    let size_of_s_string = s_string.len();
    let ptr_of_s_string = s_string.as_ptr();

    println!("s_string = {}, size_of_s_string = {}, ptr_of_s_string = {:p}", s_string, size_of_s_string, ptr_of_s_string);

    s_string = String::from("next value from the ss_string");
    let size_of_s_string = s_string.len();
    let ptr_of_s_string = s_string.as_ptr();
    println!("s_string = {}, size_of_s_string = {}, ptr_of_s_string = {:p}", s_string, size_of_s_string, ptr_of_s_string);


    let mut vector = vec![1];
    let size_of_vector = vector.len();
    let ptr_of_vector = vector.as_ptr();

    println!("vector = {:?}, size_of_vector = {}, ptr_of_vector = {:p}", vector, size_of_vector, ptr_of_vector);

    vector.push(2);
    vector.push(3);
    vector.push(4);

    println!("vector = {:?}, size_of_vector = {}, ptr_of_vector = {:p}", vector, size_of_vector, ptr_of_vector);



}


fn ownership() {
    // 所有权与字符串

    // 1. 字符串字面量存放在只读数据段中，声明后很少去修改
    // 2. 动态变化的字符串存放在堆上，通过栈内存管理堆内存

    let ptr_owner = "rust";
    let mut heap_owner = String::from("rust");

    let ptr_copy = ptr_owner;

    println!("ptr_owner = {:p}, ptr_copy = {:p}", ptr_owner, ptr_copy);


    let _heap_new = heap_owner;

    // println!("heap_owner = {:p}", heap_owner); errror: 所有权移交给了_heap_new
    println!("{}", _heap_new);


    {
        let s = String::from("hello");
        let s_new = s;
        // 此处离开作用域
    }

    // println!("s is {}", s_new); 无法使用s_new，因为它已经离开作用域被丢弃

    heap_owner = String::from("next of "); // 重新赋值，注意原变量不能使用是因为转移所有权后标注为空了，而不是立即清除了
    println!("{:?}", heap_owner);
    // 所有权与slice

    // 1. 字符串实际上是一个特殊的slice,它仅代表有效的utf-8编码的字符
    // 2. slice中可以包含任意类型, 不能使用[T]直接使用，需要使用指针（引用）类型Vec<T>

    let str_slice = vec!["rust", "go", "java"];
    let str_slice_new = str_slice;
    let u32_slice = Vec::<u32>::new();
    let new_u32_slice = u32_slice;

    // println!("str_slice = {:?}, str_slice_new = {:?}, u32_slice = {:?}, new_u32_slice = {:?}", str_slice, str_slice_new, u32_slice, new_u32_slice); error: move value
    
    // 独占资源

    let mut dynamic_source = String::from("content");

    let role1 = dynamic_source;
    let role2 = role1;

    // 避免资源被多个变量同时访问，导致资源被修改

    // 所有权与共享容器Rc<T>, 它适用于单线程
    // 使用共享容器包裹动态资源
    use std::rc::Rc;
    use std::sync::Arc;
    let mut dynamic_source = vec![1, 2, 3];
    let mut container = Rc::new(dynamic_source);
   
    let mut role1 = (*container).clone();
    role1.push(1);
    let role2 = container.clone();

    println!("container = {:?}, role1 = {:?}, role2 = {:?}", container, role1, role2); // 通过共享容器访问资源，此时共享资源有三个所有者，可以同时访问

    // 多线程Arc<T>
    let dynamic_source = String::from("rust");
    let container = Arc::new(dynamic_source);

    let role1 = container.clone();
    let role2 = container.clone();

    println!("container = {:?}, role1 = {:?}, role2 = {:?}", container, role1, role2);



}







fn main() {
    move_value();
    copy_value();
    borrow_value();
    mut_borrow_value();


    data_type();


    ownership();
}
