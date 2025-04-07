# Allocator API - Forward Compat
So one the concerns I saw with stabilizing even part of the Allocator trait was the desire for to work with the Store API ([RFC 3446](https://github.com/rust-lang/rfcs/pull/3446)). Some of that discussion revolved around wanting to make the Store Traits a super trait of the Allocator trait. This little demo repository shows how that can be done and allow forward compatibility with that desire.

My idea is simple: define the Allocator trait as is but with those additional traits. However, that adds some wrinkles then. The store traits are not stable nor at this time of writing this in nightly.

At the very least if the names of the Store traits are decided upon it would be possible to make those traits a super trait of the Allocator trait. Even if they are initially just stubs. Then the next issue is hiding those Store traits in a way that is forward compatible. We can't have users implementing store traits directly since any code they write could get broken.

Well rust already has a way to do that with derive macros. The macro can be a stub for now but if/when the store traits are stabilized then the macro can be updated to emit the desired code for the store traits. So then we just need a way to make the traits sealed but still useable by the derive macros. So users don't go trying to directly implement them until it's stabilized.

Well luckily declarative macros 2.0 has a way for us to do that. Just make the traits public, but put them in a non-public module. When using declarative macros 2.0 we can still access the traits. So this makes the trait sealed except through the macro. Then we can use derive macros as a more convenient way to use these macros.

Lastly, we probably don't want people at the moment to be able to use these derives on anything but a type that implements an Allocator. Well we can simple use a dummy function with a trait bound to make sure that the user doesn't try to use the derives on something that doesn't implement the Allocator trait.
