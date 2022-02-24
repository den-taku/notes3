use std::arch::asm;

fn mul(a: u64, b: u64) -> u128 {
    let lo: u64;
    let hi: u64;

    unsafe {
        asm!(
            "mul {a}",
            a = in(reg) a,
            inlateout("rax") b => lo,
            lateout("rdx") hi
        );
    }

    ((hi as u128) << 64) + lo as u128
}

fn add_twice(a: u64, b: u64) -> u64 {
    let mut a = a;
    unsafe {
        asm!(
            "add {a}, {b}",
            "add {a}, {b}",
            a = inout(reg) a,
            b = in(reg) b
        )
    }
    a
}

fn main() {
    let mut a = 0;
    let mut b = 0;
    println!("{a}, {b}");
    (a, b) = (4, 5);
    println!("{a}, {b}");

    let mut x: u64 = 4;
    unsafe {
        asm!(
            "mov {tmp}, {x}",
            "shl {tmp}, 1",
            "shl {x}, 2",
            "add {x}, {tmp}",
            x = inout(reg) x,
            tmp = out(reg) _,
        );
    }
    println!("{x}");
    println!("5 * 80 = {}", mul(5, 80));

    let mut a: u64 = 4;
    let b: u64 = 4;
    let c: u64 = 4;
    unsafe {
        asm!(
            "add {a}, {b}",
            "add {a}, {c}",
            a = inout(reg) a,
            b = in(reg) b,
            c = in(reg) c,
        );
    }
    println!("{a}");

    let mut a: u64 = 4;
    let b: u64 = 4;
    unsafe {
        asm!("add {a}, {b}", a = inlateout(reg) a, b = in(reg) b);
    }
    println!("{a}");

    assert_eq!(add_twice(5, 10), 5 + 10 + 10);
}
