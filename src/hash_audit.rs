/// # Hash Audit
/// ### Evaluating a hash file using bitwise comparison
/// as an exercise in low level programming and multi-threading and parallelism
/// 
/// ### Questions
/// 1. Trade offs when deciding between performance optimisation
/// 2. Specific immplementation
/// 
use encryption_types::HashingAlgs;
use encryption_ops::{hash_data, Payload, CipherPayload};
use bitvec::prelude::*;
use sha2::{Sha256, Sha512, Digest};
use std::convert::TryInto; // For converting slices to arrays


