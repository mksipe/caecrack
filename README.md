# caecrack

caecrack is a small project I developed to get more familiar with rust. It is an automatic ROT solver. It uses a pre-defined wordlist to start cracking against a specific string or word.

This program is intended to tell you the number of rotations rather than tell you the plaintext string. To do this, it takes the first word of a sentence and solves that before everything else.

---

## Installation

## Usage

### Standalone

These set of instructions are to run this program once with cargo, then can be safely removed. 

#### Installation

1. `git clone https://github.com/mksipe/caecrack`
2. `cd caecrack`
3. `cargo run -- -h`
> after usage
4. `cd ..`
5. `rm -r caecrack`


### Built-in

These set of instructions are to run this program periodically through the command line.

#### Installation

> You must be an administrator to be able to add public executables on Linux.

1. `git clone https://github.com/mksipe/caecrack`
2. `cd caecrack`
3. `sudo chmod 700 Install.sh` 
4. `sudo ./install.sh`
5. `cd .. `
6. `rm caecrack -r  `

