# MS Audio Dock Teams 键重映射器

[English](README.md)

将 Microsoft Audio Dock 上的专用 Teams 键变成真正符合你使用习惯的快捷键：
启动任意已注册应用、运行自定义程序，或者打开链接。

## 它解决了什么问题

Microsoft Audio Dock 配备了一颗专用 Teams 键，但 Windows 没有提供将这颗按键
重新分配给其他应用的通用设置。如果 Teams 不是你主要使用的会议工具，这颗实体按键
通常无法发挥作用。

MS Audio Dock Teams 键重映射器让这颗按键可以启动你真正需要的程序，同时无需修改
Dock 固件、安装自定义驱动或获取管理员权限。

## 它可以做什么

- 启动 Windows 中注册的任意桌面应用或 Microsoft Store 应用。
- 运行自定义可执行文件、打开 URL，或调用其他由系统 Shell 支持的目标。
- 按名称快速搜索已安装应用，并显示应用的 Windows 原生图标。
- 在保存前测试当前动作。
- 动作成功触发后播放可选的确认提示音。
- 登录 Windows 时自动启动。
- 关闭设置窗口后继续在系统托盘中监听。
- 使用中文或英文界面。

## 核心原理

应用通过 Windows Raw Input API 只读监听 Audio Dock，并识别 Teams 键产生的 HID
报文。它不会向 Dock 写入数据，也不会替换设备驱动。

检测到按键后，应用会把所选动作交给独立工作线程执行。从列表中选择的程序通过
Windows Shell 启动，程序列表来自 `shell:AppsFolder`，因此传统桌面程序和
Microsoft Store 打包应用都可以作为目标，同时不会阻塞设备监听。

## 系统要求

- Windows 10 或 Windows 11，x64
- Microsoft Audio Dock

内置设备配置针对标准 Microsoft Audio Dock 的 HID 标识和 Teams 键报文。

## 下载

请从仓库的 [Releases 页面](../../releases/latest)下载最新版本。每个版本包含：

| 文件 | 适用场景 |
| --- | --- |
| `*-windows-x64-installer.exe` | 需要常规的当前用户安装、开始菜单快捷方式、可选桌面快捷方式和卸载支持。 |
| `*-windows-x64-portable.zip` | 希望解压后直接运行，不进行安装。 |
| `SHA256SUMS-*.txt` | 验证下载文件是否完整。 |

安装器不需要管理员权限。Portable 压缩包中包含应用程序、中英文说明和许可证。

## 使用方法

1. 将 Microsoft Audio Dock 连接到电脑。
2. 安装应用，或解压 Portable 压缩包，然后启动 **MS Audio Dock Remapper**。
3. 展开 **按下 Teams 键时执行** 下方的动作列表。
4. 搜索并选择任意已在 Windows 中注册的应用。
5. 也可以选择列表顶部的 **自定义程序**，填写命令、可执行文件路径、URL 和可选参数。
6. 点击 **测试动作**，确认目标能够正确打开。
7. 点击 **保存**。
8. 按下 Dock 上的 Teams 键。

状态区域会显示是否检测到 Dock、已注册的匹配输入集合数量，以及最近一次触发信息。

## 后台运行

关闭设置窗口只会将应用隐藏到系统托盘，不会停止按键监听。双击托盘图标可以重新打开
窗口；需要完全停止应用时，请使用应用菜单中的 **退出**。

还可以启用以下选项：

- **登录 Windows 时自动启动**
- **启动时最小化到托盘**
- **触发时播放确认提示音**
- **启用按键重映射**

同一时间只能运行一个应用实例。

## 配置文件

所有设置均以可读 JSON 格式保存在本地：

```text
%APPDATA%\ms-audio-dock-remapper\config.json
```

应用不需要管理员权限，也不会修改 Audio Dock 的固件或驱动。

## 界面语言

首次启动时，应用会跟随 Windows 界面语言。也可以通过应用菜单直接选择中文或英文，
选择结果会被自动保存。

## 参与开发

开发环境、开发构建、测试、项目规则和发布打包方式请参阅
[CONTRIBUTING.md](CONTRIBUTING.md)。

## 许可证

本项目采用 [MIT License](LICENSE)。
