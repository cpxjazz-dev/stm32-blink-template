# STM32 Blue Pill Rust Blink 模板

给便宜 STM32 核心板的第一跑：Blue Pill（STM32F103C8T6）+ 蓝色 ST-Link V2 兼容版下载器，全 Rust（Embassy no_std），不用写一行 C。

## 这套是给谁的

- 板子：Blue Pill，芯片 STM32F103C8T6（Cortex-M3，72MHz，64KB Flash，20KB RAM）
- 下载器：蓝色 ST-Link V2 兼容版（就是你发图确认的那款）
- 效果：板载 LED（PC13）每 500ms 闪一次，RTT 打印一句 `Blue Pill blink started`

## 接线（只接 3 根）

| ST-Link V2 | Blue Pill |
|---|---|
| SWDIO | DIO（PA13） |
| SWCLK | CLK（PA14） |
| GND | GND |

- 板子自己用 Type-C 供电，**下载器那边的 3.3V 不用接**，别两边同时供电。
- 黄色跳线帽 BOOT0 留在默认的 0 那边就行，只有走 USB DFU 烧录才需要拨到 1。
- 板载 LED 在 **PC13，低电平亮**，代码里初始 `Level::High` 是灯灭，`toggle()` 翻转。灯常亮不闪，多半是逻辑反了，先别怀疑烧录。

## 跑起来

```bash
# 1. 装 Rust（没装的话）：https://rustup.rs
rustup target add thumbv7m-none-eabi

# 2. 装 probe-rs（烧录 + RTT 看日志的工具）
cargo install probe-rs-tools --locked

# 3. 烧录并运行，日志直接打在终端
cargo run --release
```

第一次 `cargo run` 会从 crates.io 下依赖，稍等一会儿。看到 LED 开始闪、终端打出 `Blue Pill blink started`，就是通了。

## 版本说明（2026-11-06 核对过 crates.io）

- `embassy-stm32 0.6.0`，feature 选 `stm32f103c8`，`memory-x` 让它自动生成 `memory.x`，不用自己写链接脚本
- `embassy-executor 0.10.0` + `embassy-time 0.5.1`，是 0.6.0 要求的那套
- 注意一个坑：executor 从 0.10 起，`arch-cortex-m` 这个 feature 改名叫 `platform-cortex-m` 了，照旧写法会报错；这份模板已经按新名字写了

## 出问题先看这三条

1. **probe-rs 认不出芯片**：套装里的芯片有可能是 GD32 / CKS32 换标的。先把报错贴出来；常见解法是换 `--chip`（比如 Flash 是 128KB 的那批写 `STM32F103CB`），或在 probe-rs 里手动指定目标芯片。
2. **LED 不闪但程序下了**：确认 BOOT0 回到 0、按一下复位键；再确认 LED 逻辑——PC13 是低电平亮。
3. **cargo 报错先看版本**：`Cargo.toml` 顶部的注释写了核对日期，抄代码前先对一下 crates.io 上的最新版，成套升级，别只升一个。

## 换 Black Pill（F411CE）要改的三处

- `Cargo.toml` 里 feature `stm32f103c8` 改成 `stm32f411ce`
- target 改成 `thumbv7em-none-eabihf`（M4F 带浮点）
- `.cargo/config.toml` 里 runner 的 `--chip` 改成 `STM32F411CE`
- 板载 LED 同样多在 PC13，以板子丝印为准

## 修改记录

- 2026-11-06：初版。Blue Pill F103C8 + ST-Link V2，Blink 500ms，版本按当日 crates.io 核对（embassy-stm32 0.6.0 / executor 0.10.0 / time 0.5.1）。
