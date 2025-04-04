## IME switcher

> Q: 为什么从 **NO English Mode** 改为 **IME switcher**？  
> A: 因为 win11 24H2 26100.3624 下，设置默认默认为英文不起效，多次切换窗口后会重置为中文，对于我这种大部分时间敲代码的码农来说十分不友好，所以将输入法和模式作为配置项从环境变量中读取。

~~防止微软拼音输入法启动切换到英文模式~~

防止微软拼音输入法因为窗口切换而使输入法默认模式失效

和AutoHotkey脚本的区别是这个程序监听focus事件而不是一直循环，运行在后台几乎不消耗资源，用户完全无感知。

![Screenshot](assets/screenshot.png)

## 安装

下载以后直接运行exe即可。需要开机启动可以执行 `copy_to_startup.bat`，这个脚本会把exe复制到startup目录。
在运行的时候托盘会有一个图标，在图标上点击右键可以退出程序。

## 配置

可在环境变量或者可执行程序所在目录下的 `.env` 文件中配置，配置项(示例见 [.env.example](.env.example)):
- IME_SWITCHER_INPUT_METHOD: 配置需要监听的目标输入法，十六进制（`0x` 前导可省略）。取值见 [win11 input locales](https://learn.microsoft.com/en-us/windows-hardware/manufacture/desktop/default-input-locales-for-windows-language-packs?view=windows-11#input-locales)
- IME_SWITCHER_CMODE: 配置输入法的模式。取值见 [ime conversion mode values](https://learn.microsoft.com/en-us/windows/win32/intl/ime-conversion-mode-values)。对应 win11 设置的 *微软拼音输入法 > 常规* 下的 *默认模式*
  - 英文模式: `0`
  - 中文模式: `1025`
- IME_SWITCHER_SLEEP_MILLIS: 切换的等待时间，时间太短可能会被系统改回去，可以根据情况调整，建议值 20-50 之间

> windows 打开环境变量面板的命令: `rundll32 sysdm.cpl,EditEnvironmentVariables`

## 编译

代码用Rust编写，安装Rust环境以后执行下面的命令

```bash
cargo build [--release]
```
