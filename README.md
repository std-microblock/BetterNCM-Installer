<div align="center"><image width="140em" src="https://github.com/MicroCBer/BetterUniverse-Installer/assets/66859419/919b7908-16b1-4a92-8468-07f02ab0f21d" /></div>
<h1 align="center">BetterUniverse Installer</h1>
<h3 align="center">一键安装 Better 系软件</h3>



目前支持：
- [BetterNCM](https://github.com/MicroCBer/BetterNCM)

计划支持：
- [BetterUniversal for QQNT/Electron](https://github.com/koishi-nt/BetterQQNT)

-------
![Installer](https://user-images.githubusercontent.com/66859419/210129835-11ceea16-f5dd-43b7-ba83-625a3c4d920e.png)

![bqqnt](https://user-images.githubusercontent.com/66859419/243067741-4b166ea9-d8fb-4b0d-8d1e-ef754f0d1eda.png)

![image](https://user-images.githubusercontent.com/66859419/204120743-a528b624-d016-4f6f-a0d7-e769cdd2dd74.png)




# 构建
```bash
cargo +nightly build --release -Z build-std=core,alloc,std,panic_abort -Z build-std-features=panic_immediate_abort --target i686-pc-windows-msvc
```
