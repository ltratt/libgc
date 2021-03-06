#![cfg_attr(feature = "rustgc", feature(gc))]
#![feature(core_intrinsics)]
#![feature(allocator_api)]
#![feature(alloc_layout_extra)]
#![feature(arbitrary_self_types)]
#![feature(dispatch_from_dyn)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(raw_vec_internals)]
#![feature(const_fn)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(maybe_uninit_ref)]
#[cfg(not(all(target_pointer_width = "64", target_arch = "x86_64")))]
compile_error!("Requires x86_64 with 64 bit pointer width.");

pub mod gc;
#[cfg(feature = "gc_stats")]
pub mod stats;

pub use gc::Gc;

use libgc_internal::GcAllocator;
pub use libgc_internal::GlobalAllocator;

static GC_ALLOCATOR: GcAllocator = GcAllocator;
pub static GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator;
