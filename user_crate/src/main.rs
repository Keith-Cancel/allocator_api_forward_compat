use api_traits::{AllocError, Allocator};
use store_derive::{Store, StoreDangling};

use core::alloc::Layout;
use core::ptr::NonNull;

#[derive(Store, StoreDangling)]
struct MyAlloc;

unsafe impl Allocator for MyAlloc {
    fn allocate(&self, _layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        println!("Allocating");
        Err(AllocError)
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
        println!("Deallocating");
    }
}

fn main() {
    let a = MyAlloc;
    _ = a.allocate(Layout::new::<u8>());
}
