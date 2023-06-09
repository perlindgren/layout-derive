#![feature(min_specialization)]
use heapless::Vec;
use layout_derive::Layout;
use layout_trait::GetLayout;

#[derive(Layout)]
struct Simple {
    a: u32,
    b: u64,
}

fn main() {
    let mut layout: Vec<layout_trait::Layout, 8> = Vec::new();
    let a = Simple { a: 0, b: 0 };
    a.get_layout(&mut layout);
    println!("{:#x?}", layout);

    // u32
    assert_eq!(layout[0].size, 4);

    // u64
    assert_eq!(layout[1].size, 8);
}
