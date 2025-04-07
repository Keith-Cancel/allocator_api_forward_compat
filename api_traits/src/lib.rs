#![feature(decl_macro)]

use core::alloc::Layout;
use core::ptr::NonNull;

pub mod hidden {
    // Just the names from the proposed RFC 3446 for the storage api traits.
    pub unsafe trait StoreDangling {}
    pub unsafe trait Store: StoreDangling {}
    // Marker traits for Store.
    pub unsafe trait StoreStable {}
    pub unsafe trait StoreNoOpResolve: StoreStable {}
    pub unsafe trait StorePinning: StoreStable {}

    // This just makes sure that these are only ever used on Allocators and
    // no other types.
    pub macro assert_is_allocator($t:ty) {
        const _: fn() = || {
            fn assert_impl<T: $crate::Allocator>() {}
            assert_impl::<$t>();
        };
    }
}

pub mod macros {
    // Take advantage of the fact declarative macros 2.0 allows
    // us to hide what the details of the store API look like.
    // Since the only way to implement the store API is to use
    // the macros. As the macros are updated they follow the impl
    // till the store trait is stabilized.
    //
    // This way users wanting to make an allocator using the
    // Allocator API can just use the derive macros, and it will
    // be forward compatible with the store API.
    pub macro __impl_store($t:ty) {
        $crate::hidden::assert_is_allocator!($t);
        unsafe impl $crate::hidden::Store for $t {}
    }
    pub macro __impl_store_dangling($t:ty) {
        $crate::hidden::assert_is_allocator!($t);
        unsafe impl $crate::hidden::StoreDangling for $t {}
    }
    // These probably don't need hidden just being marker traits but for now
    // why not.
    pub macro __impl_store_stable($t:ty) {
        $crate::hidden::assert_is_allocator!($t);
        unsafe impl $crate::hidden::StoreStable for $t {}
    }
    pub macro __impl_store_no_op_resolve($t:ty) {
        $crate::hidden::assert_is_allocator!($t);
        unsafe impl $crate::hidden::StoreNoOpResolve for $t {}
    }
    pub macro __impl_store_pinning($t:ty) {
        $crate::hidden::assert_is_allocator!($t);
        unsafe impl $crate::hidden::StorePinning for $t {}
    }
}

// Just a stub of the allocator trait for illustrative purposes.
pub struct AllocError;

// Maybe add hidden::StoreNoOpResolve + hidden::StorePinning?
pub unsafe trait Allocator: hidden::Store {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError>;
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout);
    // The rest of the methods that are in the allocator trait
    // ...
    // ...
    // ...
}
