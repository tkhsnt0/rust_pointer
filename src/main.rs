use std::mem::size_of;
static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];
fn main() {
    let a = 42 as usize;
    let b = &B;
    let c = Box::new(C);
    let d: i64 = 42;
    let d_ptr = &d;
    let d_raw_ptr = &d as *const i64;
    let d_addr: usize = unsafe { std::mem::transmute(d_ptr) };

    let f = 10;
    let f_box = Box::new(10);

    //a
    println!("a (usize):");
    println!("place:{:p}", &a);
    println!("size:{:?} byte", size_of::<usize>());
    println!("value:{:?}", a);
    println!();
    //b
    println!("b (reference to B):");
    println!("place:{:p}", &b);
    println!("size:{:?} byte", size_of::<&[u8; 10]>());
    println!("reference(place):{:p}", b);
    println!("reference(value):{:?}", b);
    println!();
    //c
    println!("c (C in Box):");
    println!("place:{:p}", &c);
    println!("size:{:?} byte", size_of::<Box<[u8]>>());
    println!("reference(place):{:p}", c);
    println!("reference(value):{:?}", c);
    println!();
    //B
    println!("B (10 byte array):");
    println!("place:{:p}", &B);
    println!("size:{:?} byte", size_of::<[u8; 10]>());
    println!("reference(value):{:?}", B);
    println!();
    //C
    println!("C (11 byte array):");
    println!("place:{:p}", &C);
    println!("size:{:?} byte", size_of::<[u8; 11]>());
    println!("reference(value):{:?}", C);
    println!();
    //d
    println!("d (int64):");
    println!("place:{:p}", &d);
    println!("size:{:?} byte", size_of::<i64>());
    println!("value:{:?}", d);
    println!();
    //d_ptr
    println!("d_ptr (&i64)):");
    println!("place:{:p}", &d_ptr);
    println!("size:{:?} byte", size_of::<&i64>());
    println!("reference(place):{:p}", d_ptr);
    println!("reference(value):{:?}", d_ptr); //参照はデリファレンスされる
    println!();
    //d_raw_ptr
    println!("d_raw_ptr (*const i64)):");
    println!("place:{:p}", &d_raw_ptr);
    println!("size:{:?} byte", size_of::<*const i64>());
    println!("reference(place):{:p}", d_raw_ptr);
    println!("reference(value):{:?}", d_raw_ptr); //生ポインタはデリファレンスされない
    println!();
    //d_addr
    println!("d_addr (usize)):");
    println!("place:{:p}", &d_addr);
    println!("size:{:?} byte", size_of::<usize>());
    println!("reference(value):0x{:0x?}", d_addr);
    println!();
    //f
    println!("f (i32):");
    println!("place:{:p}", &f);
    println!("size:{:?} byte", size_of::<i32>());
    println!("value:{:?}", f);
    println!();
    //f_box
    println!("f_box (Box<i32>):");
    println!("place:{:p}", &f_box); //f_box自体はスタック領域に存在する
    println!("size:{:?} byte", size_of::<Box<i32>>());
    println!("reference(place):{:p}", f_box); //fの中の値は参照先のヒープ領域のアドレス
    println!("reference(value):{:?}", f_box);
}
