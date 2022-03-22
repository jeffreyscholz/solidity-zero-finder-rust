## Install
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

## Build
`cargo build --release`

## Run
`./target/release/eth-zero-finder "myFunctionName?(uint,address,...)"`

The question mark will be substituted with a hex number found by searching. Your function should follow the format for solidity function selectors. Eg: No arguments: "myFunction()". One address argument: "myFunction(address)". Two arguments: "anotherFunction(uint256,address)" etc.
