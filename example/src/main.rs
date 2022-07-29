use wasmedge_hostfunctionexample_interface::*;

fn main() {
    // set_class_id(123);
    // set_class_name("WasmEdge");
    // let mut num: u32 = add_student("Alice");
    // println!("After adding Alice, num = {:?}", num);
    // num = add_student("Fred");
    // println!("After adding Fred, num = {:?}", num);
    // num = add_student("Terry");
    // println!("After adding Terry, num = {:?}", num);
    // print();

    println!("Key = 189");
    set_key_no(189);
    println!("Encrypt: ");
    encrypt("~");
    println!("Decrypt: ");
    decrypt("!");
    println!("---------");
    println!("Key = 1");
    set_key_no(1);
    println!("Encrypt: ");
    encrypt("abc");
    println!("Decrypt: ");
    decrypt("bcd");
}
