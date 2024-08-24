# Nentrokey


         ____   _____   _____  ____   _   _
    / __ \ |  __ \ |_   _|/ __ \ | \ | |
   | |  | || |__) |  | | | |  | ||  \| |
   | |  | ||  _  /   | | | |  | || . ` |
   | |__| || | \ \  _| |_| |__| || |\  |
    \____/ |_|  \_\|_____|\____/ |_| \_|
     _____  _       _____
     | |     | |       | |
    / ____|| |     |_   _|
   | |     | |       | |
   | |____ | |____  _| |_
    \_____||______||_____| "

# :: Nentrokey Security ::
***A Lightweight concurrent toy encryption implementation of encryption and binary pair auditing for deepending knowledge***


### Features

- Symmetrical Encryption with RSA Support and in depth documentation based on reading
- Assymetrical Public/Private Key implementation of AES-X Bit
- Hashing Algorithm
- Bitwise binary file pairing for expansion of low level comprehension
  - Validation with MD5
- Decryption suite for all the functionality
- Simple CLI

### Roadmap
- Explorations of ```Ockam``` [Ockam Github]("https://github.com/build-trust/ockam") for multi-node secure data in transit.
- Audacious Attempt to implement an ```Elliptical Curve Algorithm``` (Much bigger mathematical, and development challenge without a library)
  



### Usage 
- import ```use::nentrokey::lib::main;``` into your own library after installing with ```cargo add nentrokey```
- Run the src/main.rs binary to execute using flags
- Use the src/lib.rs file to customize the output or types of encryption of the library.

```bash
- Use flag -e or --encryption to specify encryption type
- Use flag -f or --file to specify file path
- Use flag -t or --text to specify text
- Use flag -v or --verbose to enable verbose output
- Use flag -q or --quiet to enable quiet output
```
Binary:
- Run your ```src/main.rs``` function from the CLI
- More to come this is mostly an experimental project

### Example in program implementation: 
```rust
// Your main file
use nentrokey::main;

fn run_nentrokey() {
    nentrokey::main();
}
```

### Execution: 
```bash
cargo run nentrokey -t -e symmetric --verbose --quiet
```

### Contribute
1. Clone the repo
2. Create a feature branch for your additions
3. Pull Request to the Develop Branch

```bash
git clone https://www.github.com/nentropy/nentrokey/
git checkout -b feature_name
```

### Have the Fun! -- Nentropy.