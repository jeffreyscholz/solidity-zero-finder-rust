Functions are more efficient in solidity when their function selector has mostly zeros.

For example

`deposit278591A(uint256)` has a function selector of `00000070`
but `deposit(uint256)` has a function selector of `b6b55f25`

The function selector is computed as the first 4 bytes of the keccak256 of the function signature. The function signature excludes the variable names. For example, `sendValue(uint256 amount)` is not correct but `sendValue(uin256)` is.

The gas cost of a function name is 4 times the number of zero bytes, and 16 times the number of nonzero bytes. So in the worst case, the gas cost is 64 gas (4 non-zero bytes), and in the best case 28 gas (3 zero and 1 non-zero). An all zero function selector won't compile because that conflicts with the fallback function. Thus, `mint_22F5A30(uint256)` is more efficient than `mint(uint256)`

If the zeros are leading zeros, this will make the contract smaller and save gas on deployment.

## Install Rust
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

## Build
`cargo build --release`

## Run
`./target/release/eth-zero-finder "myFunctionName?(uint,address,...)"<num threads> (true|false)`

The question mark will be substituted with a hex number found by searching. Your function should follow the format for solidity function selectors. Eg: No arguments: "myFunction()". One address argument: "myFunction(address)". Two arguments: "anotherFunction(uint256,address)" etc.

Num threads is the number of threads to use. The final true false is if you want to force the zeros to be leading zeros. Leading zeros lead to a lower gas cost on deployment. 

The search usually finds a solution in less than 30 seconds on a macbook pro with 8 threads.

Example run:

`./target/release/eth-zero-finder "withdraw_?()" 8 true`
