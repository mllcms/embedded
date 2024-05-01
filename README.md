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

## 温湿度

- dht11 监控温度湿度

```shell
cargo r -rp dht11
```

![接线图](/dht11/接线图.jpg)

## 蜂鸣器

- buzzer 循环发声
- buzzer-switch 触发发声

```shell
cargo r -rp buzzer
cargo r -rp buzzer-switch
```

![接线图](/buzzer/接线图.jpg)