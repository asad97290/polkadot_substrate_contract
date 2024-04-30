## install cargo contract
```bash
 cargo install --force --locked cargo-contract
```

## bootstrap hello world project
```bash
cargo contract new flipper
```

## Test Contract
```bash
cargo test  
```

## Build contract
```bash
cargo build 
```
and
```bash
cargo contract build 
```

## install substrate-contracts-node
```bash
cargo install contracts-node
```

## Run a local blockchain
```bash
substrate-contracts-node --dev
```



## Deploy contract to local blockchain
- go to this site 

[Contract deployment UI](https://contracts-ui.substrate.io/)

- deploy contract by selecting the flipper.contract in the 
target/ink directory 
