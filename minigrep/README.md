## 运行命令

### 测试

```shell
cargo test
```

- 指定运行测试单个文件
```shell
cargo test --test search_test 
```

### 正常运行

- 不忽略大小写
```shell
cargo run you poem.txt
```

- 忽略大小写运行
```shell
IGNORE_CASE=1 cargo run you poem.txt
```
