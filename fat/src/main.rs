use std::ops::Deref;

fn as_raw_bytes<'a, T: ?Sized>(x: &'a T) -> &'a [u8] {
    unsafe { std::slice::from_raw_parts(x as *const T as *const u8, std::mem::size_of_val(x)) }
}

pub struct S<T: ?Sized> {
    pub x: u8,
    pub y: T,
}

fn main() {
    let arr: [u16; 3] = [1, 2, 3];
    let arrref = &arr;
    let arrptr = &arr as *const [u16; 3];
    let arrslice = &arr[..];
    let arrsliceptr = &arr[..] as *const [u16];
    let strslice = "hoge";
    let clos = |x, y| x + y;
    let closref: &Fn(u8, u8) -> u8 = &clos;
    let mut a = 0;
    let clos2 = |x| {
        a += x;
        a
    };
    let clos2ref: &FnMut(u8) -> u8 = &clos2;
    let sarr: Box<S<[u16]>> = Box::new(S { x: 3, y: [1, 2, 3] });
    let sarrref = sarr.deref();
    let sslice: Box<S<[u16]>> = Box::new(S { x: 3, y: [1, 2, 3] });
    let ssliceref = sslice.deref();
    println!("arrref = {:?}", as_raw_bytes(arrref));
    println!("&arrref = {:?}", as_raw_bytes(&arrref));
    println!("&arrptr = {:?}", as_raw_bytes(&arrptr));
    println!("&arrslice = {:?}", as_raw_bytes(&arrslice));
    println!("&arrsliceptr = {:?}", as_raw_bytes(&arrsliceptr));
    println!("strslice = {:?}", as_raw_bytes(strslice));
    println!("&strslice = {:?}", as_raw_bytes(&strslice));
    println!("closref = {:?}", as_raw_bytes(closref));
    println!("&closref = {:?}", as_raw_bytes(&closref));
    println!("clos2ref = {:?}", as_raw_bytes(clos2ref));
    println!("&clos2ref = {:?}", as_raw_bytes(&clos2ref));
    println!("sarrref = {:?}", as_raw_bytes(sarrref));
    println!("&sarrref = {:?}", as_raw_bytes(&sarrref));
    println!("ssliceref = {:?}", as_raw_bytes(ssliceref));
    println!("&ssliceref = {:?}", as_raw_bytes(&ssliceref));
}
