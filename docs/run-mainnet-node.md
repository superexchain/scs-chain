
## **1. local**

### install
[install environment](./install-environment.md)

### build

```
cargo build --release --features scs
```

### run

```
./scs --database auto --pruning archive
```

### **2. docker**
```
docker run -id --name scs-node wjyask/scs-node:latest /usr/local/bin/scs --database auto
```