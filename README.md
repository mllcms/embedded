# embedded

## 模板生成

```shell
cargo generate esp-rs/esp-template
cargo r -rp clean --target x86_64-pc-windows-msvc
```

## 亮灯

- blinky 同步亮灯
- blinky-async 异步亮灯

```shell
cargo r -rp blinky
cargo r -rp blinky-async
```

