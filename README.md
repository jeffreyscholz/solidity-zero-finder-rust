Functions are more efficient in solidity when their function selector has mostly zeros.

For example

`deposit278591A(uint256)` has a function selector of `00000070`
but `deposit(uint256)` has a function selector of `b6b55f25`

The function selector is computed as the first 8 bytes of the keccak256 of the function signature. The function signature excludes the variable names. For example, `sendValue(uint256 amount)` is not correct but `sendValue(uin256)` is.

The gas cost of a function name is 4 times the number of zero bytes, and 12 times the number of nonzero bytes. So in the worst case, the gas cost is 96 gas, and in the best case 40 gas. An all zero function selector won't compile because that conflicts with the fallback function. Thus, `mint_22F5A30(uint256)` is more efficient than `mint(uint256)`

## Install Rust
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

## Build
`cargo build --release`

## Run
`./target/release/eth-zero-finder "myFunctionName?(uint,address,...)"`

The question mark will be substituted with a hex number found by searching. Your function should follow the format for solidity function selectors. Eg: No arguments: "myFunction()". One address argument: "myFunction(address)". Two arguments: "anotherFunction(uint256,address)" etc.

The search usually completes in less than 10 minutes on a macbook pro.
