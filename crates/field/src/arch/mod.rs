// Copyright 2023-2025 Irreducible Inc.

use cfg_if::cfg_if;

mod arch_optimal;
mod shared;
mod strategies;

cfg_if! {
	if #[cfg(all(target_arch = "x86_64"))] {
		#[allow(dead_code)]
		mod portable;

		mod x86_64;
		pub use x86_64::{packed_128, packed_256, packed_512, packed_aes_128, packed_aes_256, packed_aes_512, packed_ghash_128, packed_ghash_256, packed_ghash_512, m128::M128, M256};
	} else if #[cfg(target_arch = "aarch64")] {
		#[allow(dead_code)]
		mod portable;

		mod aarch64;
		pub use aarch64::{packed_128, packed_aes_128, packed_ghash_128, M128};
		pub use portable::{packed_256::{self, M256}, packed_512, packed_aes_256, packed_aes_512, packed_ghash_256, packed_ghash_512};
	} else if #[cfg(target_arch = "wasm32")] {
		#[allow(dead_code)]
		mod portable;

		mod wasm32;
		pub use wasm32::{packed_ghash_128, packed_ghash_256};
		pub use portable::{packed_128::{self, M128}, packed_256::{self, M256}, packed_512, packed_aes_128, packed_aes_256, packed_aes_512, packed_ghash_512};
	} else {
		// Portable fallback (covers wasm64/memory64, which is neither
		// target_arch="wasm32" nor a native arch). `portable::packed_128::M128`
		// is `u128`; the extra `pub use u128 as M128` here was a duplicate that
		// breaks the build on wasm64 (E0252). Willow wasm64 verifier fix.
		mod portable;
		pub use portable::{packed_128::{self, M128}, packed_256::{self, M256}, packed_512, packed_aes_128, packed_aes_256, packed_aes_512, packed_ghash_128, packed_ghash_256, packed_ghash_512};
	}
}

pub use arch_optimal::*;
pub(crate) use portable::packed_arithmetic::{interleave_mask_even, interleave_with_mask};
pub use portable::{
	packed::PackedPrimitiveType, packed_1, packed_2, packed_4, packed_8, packed_16, packed_32,
	packed_64, packed_aes_8, packed_aes_16, packed_aes_32, packed_aes_64,
};
pub use strategies::*;
