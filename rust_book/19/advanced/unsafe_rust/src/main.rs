

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    
    // let adress = 0x012345usize;
    // let r = adress as *const i32;

    // unsafe {
    //     println!("r is: {}", *r);
    // }
    
    unsafe fn dangerous () {}

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    // let (a,b) = r.split_at_mut(3);
    let (a,b) = split_at_mut(r, 4);

    assert_eq!(a, &mut [1, 2, 3, 4]);
    assert_eq!(b, &mut [5, 6]);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}

unsafe trait Foo {
}

unsafe impl Foo for i32 {
}

