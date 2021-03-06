# Rust学习日志,repo为:https://github.com/gystar/HelloRust

## 7.4

- 使用官网下载的rustup-init.exe安装win64-gnu版本的Rust，使用rustup uninstall 卸载Rust
- 配置vscode的rust运行环境
- 阅读《Rust编程之道》的第一章，读书笔记在doc目录中

## 7.5

- 阅读《Rust编程之道》的第二章，全面梳理语法，读书笔记在doc目录

## 7.6 

- 阅读《Rust编程之道》的第三章，读书笔记在doc目录

## 7.7 

- 阅读《Rust编程之道》的第五章，了解所有权机制，读书笔记在doc目录
- 完成第十章的完整示例代码，结果在csv_challenge目录
- 开始做一部分的小练习

## 7.8

- 阅读《Rust编程之道》的第十章，了解unsafe模块，读书笔记在doc目录

- 完成所有的rustlings小练习，结果在rustlings目录

## 7.9
- 开始做hard way的编程练习题目，完成以下编程练习：
  - ex6
  - ex7
  - ex8
  - ex9
  - ex31 单步调试Rust

## 7.10
- 完成以下编程练习：
  - ex15
  - ex16
  - ex17 小型数据库
  - ex18 函数指针
  - ex24 控制台输入
  - ex25  控制台输入2
  - ex45  socket连接

7.11 

- 完成以下编程小练习
  - ex32 实现一个双向链表
  - ex33 链表算法：冒泡排序和归并排序
- 开始看risvc书籍和清华的OS课程


# rCoreTutorial
os tutorial summer of code
用Rust实现rCore的所有实验

https://rcore-os.github.io/rCore-Tutorial-deploy/docs/lab-0/guide/intro.html

## 7.12  

- 开始lab0，遇到不少问题
  
  - rust的版本控制rust-toolchain文件不能有UTF8的BOOM头，否则rust相关的工具无法运行
- 在windows下进行lab0编译，和实验指导的效果不同，出现各种奇怪点，开始用wsl+vscode开发
  - wsl完全卸载：
    - Get-AppxPackage -allusers | Select Name, PackageFullName
    - get-appxpackage CanonicalGroupLimited.Ubuntu16.04onWindows | remove-Appxpackage    
  - 在wsl中安装rust：
    - 安装wls2  https://docs.microsoft.com/zh-cn/windows/wsl/install-win10

    - 更换软件源为阿里源

    - 新建/etc/wsl.conf，并配置一些类容，否则无法联网：

      ```
      [network]
      generateHosts = true
      generateResolvConf = true
      ```

    - 再apt update和apt upgrade

    - 用官网的方式安装rust，期间可能需要安装一些依赖的包。


## 7.13  

- 终于搞定了wls2的编译环境
-  完成了lab0的代码

## 7.14 

- 完成了lab1的代码
- 完成lab1的结果在wsl2中运行
- win10下也可以编译通过了，需要先cargo clean
- 直接用win10下的vscode来下编写代码。wsl对windows系统的文件进行了挂载，在wsl中编译并执运行结果。

## 7.15

- 完成了lab2的代码

## 7.16

- 研究线段树算法

## 7.16

- 完成了线段树分配单个物理页的算法

## 7.18

- 完成了lan3的页表和页表项的设计

## 7.19

- 完成了页表映射的代码
- 参加第二次交流会
- 改进线段树分配算法：每个物理页只占用3bit，原来每个占用18B+3bit

## 7.20

- 完成实验指导4，一步步的进行的线程、进程的封装，最后完成多线程运行和正常退出。
- 使用的是lock版本的线程

## 7.21

- 完成了实验指导5，加载设备树和文件系统
- 初步完成了伙伴分配算法。

## 7.22

- 完成实验指导6，加载用户程序并运行notebook程序
- 为每个lab创建本地和远程分支。
- 实现实验1的内容：将物理页的分配信息放在物理页中

## 7.22
- 完成伙伴分配算法
- 将伙伴分配算法应用于分支lab2，在master分支上运行还有问题

## 7.23
- 完成实验4（上）
- 基本完成实验4（下）中的stride算法，还未解决溢出问题

## 7.24
- 解决了stride算法的溢出问题，并应用到分支lab4

## 7.25
- 进行试验六，完成了sys_getid和sys_fork，打包文不了解做法

## 7.26
- 补充了7个rust的pat练习题目，在rust的repo上完成了。
- 参加第三次交流会
- 提交调查问卷

## 7.27
- 在分支lab3+上完成了时钟页面置换算法
- 完成rCoreTutorial的实验报告

