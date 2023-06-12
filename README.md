<div align="center"><image width="140em" src="https://user-images.githubusercontent.com/66859419/183120498-1dede5b4-0666-4891-b95f-c3a812b3f12f.png" /></div>
<h1 align="center">BetterNCM Installer II</h1>
<h3 align="center">PC版网易云客户端插件管理器</h3>

一键安装 [BetterNCM V2](https://github.com/MicroCBer/BetterNCM)

**网易云版本必须 `>=2.10.2`**

![image](https://user-images.githubusercontent.com/66859419/204120743-a528b624-d016-4f6f-a0d7-e769cdd2dd74.png)

![Installer](https://user-images.githubusercontent.com/66859419/210129835-11ceea16-f5dd-43b7-ba83-625a3c4d920e.png)

# 手动安装流程
1. 从 BetterNCM 仓库下载最新版 `BetterNCMII.dll`
2. 打开网易云音乐安装目录，将上一步下载的 `BetterNCMII.dll` 复制进去并改名为 `msimg32.dll`

# 插件库
已在 BetterNCM 内置

# 构建
```bash
cargo +nightly build --release -Z build-std=core,alloc,std,panic_abort -Z build-std-features=panic_immediate_abort --target i686-pc-windows-msvc
```
