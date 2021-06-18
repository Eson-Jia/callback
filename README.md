# Rust 函数作为 C 代码的回调

有的时候依赖库需要回调函数来更新其内部状态,或者将数据传给调用方.可以传入一个 Rust 函数到依赖库来实现此功能. 有一点是必须的:回调函数标记位`extern`且调用约定正确才能正确被 C 代码调用. 回调函数通过一个注册调用传递给 C
库,然后就可以正常被调用了.

## linux 编译与链接静态库

gcc 链接阶段使用一下命令指定链接参数:

- `-Ldir`:

  Add directory dir to the list of directories to be searched for -l.

- `-llibrary` or `-l library`:

Search the library named library when linking.  (The second alternative with the library as a separate argument is only
for POSIX compliance and is not recommended.)
It makes a difference where in the command you write this option; the linker searches and processes libraries and object
files in the order they are specified. Thus, foo.o -lz bar.o searches library z after file foo.o but before bar.o. If
bar.o refers to functions in z, those functions may not be loaded.

The linker searches a standard list of directories for the library, which is actually a file named liblibrary.a. The
linker then uses this file as if it had been specified precisely by name.

The directories searched include several standard system directories plus any that you specify with -L.

Normally the files found this way are library files---archive files whose members are object files. The linker handles
an archive file by scanning through it for members which define symbols that have so far been referenced but not
defined. But if the file that is found is an ordinary object file, it is linked in the usual fashion. The only
difference between using an -l option and specifying a file name is that -l surrounds library with lib and .a and
searches several directories.
通常以这种方式找到的文件是库文件——其成员是目标文件的归档文件。链接器通过扫描归档文件中的成员来处理归档文件，这些成员定义了迄今为止已被引用但尚未定义的符号。但是如果找到的文件是一个普通的目标文件，它会以通常的方式链接。唯一使用的差异-l选项并指定一个文件名是-l围绕着图书馆与lib和.a
和搜索几个目录。

## cargo rustc

```shell
-L [KIND=]PATH      Add a directory to the library search path. The optional KIND can be one of dependency, crate, native, framework, or all (the default).
-l [KIND=]NAME      Link the generated crate(s) to the specified native library NAME. The optional KIND can be one of static, framework, or dylib (the default).
```

## 链接第三方库

链接库的流程,将所有代码都编译成目标文件之后,需要把这些文件链接起来生成可执行文件.目标文件有未定义的符号(例如:`undefined reference to 'register_callback'`),需要第三方库(通过`-l`命令传入的库)中查找.

在源码中加`#[link(name = "extlib")]`与在编译脚本中加`println!("cargo:rustc-link-lib=extlib");`的效果是一样的,都是告诉`cargo`去链接本地库`extlib`.
使用`cc::Build`去编译生成一个第三方库的同时,会动态的调用`println!("cargo:xxxxxxxx");`产生需要的信息传给`cargo`编译器.

注意当我们没有依赖库的源码只有库文件的时候,我们可以将链接库的设置硬编码到[源码中](./src/main.rs)

### 无源码第三方库

### 有源码第三方库

参考:

- [cargo:rustc-link-lib=[KIND=]NAME](https://doc.rust-lang.org/cargo/reference/build-scripts.html#cargorustc-link-libkindname)
- [-l: link the generated crate to a native library](https://doc.rust-lang.org/rustc/command-line-arguments.html#-l-link-the-generated-crate-to-a-native-library)
- [the link attribute](https://doc.rust-lang.org/reference/items/external-blocks.html#the-link-attribute)

