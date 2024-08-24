/// # Sha-256 Bitwise
/// #### A 256 bit parallelization of the SHA-256 Algorithm
/// 1. Rayon is used for parallel along with the Mutex Data Structre
/// 
/// If you want to read more about hash functions in general, do so here. That said, in order to move forward let’s recap three of the main purposes of a hash function:
///## What are hashes for?
///- To scramble data deterministically
///- To accept input of any length and output a fixed-length result
///- To irreversibly manipulate data. The input can’t be derived from the output
///
///
use rayon::prelude::*;
use std::sync::Mutex;

use std::process::Command;
use regex::Regex;

// Follow the guide
