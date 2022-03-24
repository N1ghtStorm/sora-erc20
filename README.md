# 1. Erc20 Pallet Substrate Node
Is a test node with pallet_erc20 - that imitates [ERC20](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/ERC20.sol) contract

# 2. You can check exctrinsic [here](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/extrinsics)

# 3. Technical documentation

### 3.1 Build

```bash
make build
```

### 3.2 Run node in temp in memory storage in dev mode

```bash
make run
```

### 3.3 Run node in persistent storage in dev mode

```bash
make run-local
```

### 3.4 Run tests

```bash
make test
```

### 3.5 Run clippy linter

```bash
make lint
```
