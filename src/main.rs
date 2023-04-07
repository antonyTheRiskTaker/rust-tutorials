// use std::slice;

fn main() {
    // let mut num = 5;

    // let r1 = &num as *const i32;
    // let r2 = &mut num as *mut i32;

    // Creating a raw pointer to an arbitrary memory address
    // let address = 0x012345usize;
    // let r = address as *const i32;

    // Dereferencing raw pointers within an `unsafe` block
    // unsafe {
    //     println!("r1 is: {}", *r1);
    //     println!("r2 is: {}", *r2);
    // }

    // unsafe {
    //     dangerous();
    // }

    // let mut v = vec![1, 2, 3, 4, 5, 6];

    // let r = &mut v[..];

    // let (a, b) = r.split_at_mut(3);

    // assert_eq!(a, &mut [1, 2, 3]);
    // assert_eq!(b, &mut [4, 5, 6]);

    // let address = 0x01234usize;
    // let r = address as *mut i32;

    // let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
}

// unsafe fn dangerous() {}

// fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = values.len();
//     let ptr = values.as_mut_ptr();

//     assert!(mid <= len);

//     unsafe {
//         (
//             slice::from_raw_parts_mut(ptr, mid),
//             slice::from_raw_parts_mut(ptr.add(mid), len - mid),
//         )
//     }

    // (&mut values[..mid], &mut values[mid..])
// }