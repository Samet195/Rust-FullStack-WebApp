# Rust FullStack Web App
Cross-Platform WebAssembly Application

> [!NOTE]
> This project is a prototype and a draft for application development with **Web Assembly** and **Rust**. So  it's not a real application. Also is <u>not ready for production applications yet</u>.

## Building
> [!IMPORTANT]
> The application was developed and tested on **Ubuntu 22.04 LTS** and not adapted yet for develop on other operating systems.

### Requirements
Technology | Link
-|-
Rust | https://www.rust-lang.org/tools/install
Cargo Make | https://crates.io/crates/cargo-make#installation
Android Studio | https://developer.android.com/studio
Android SDK API 34 | https://developer.android.com/about/versions/11/setup-sdk
Android NDK | https://developer.android.com/studio/projects/install-ndk
---

### Prepairing Build Environment
#### Installing Rust
```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Installing `cargo-make`
```sh
$ cargo install --force cargo-make
```

#### Generating standalone NDK toolchain
```sh
$ $NDK_HOME/build/tools/make_standalone_toolchain.py --arch arm64 --api 34 --install-dir backend/lib/ndk
```
---

### Compiling

#### Run on pc as server
```sh
$ cargo make -p production run
```

or

#### As part of Android app
```sh
$ cargo make -p production build-android
```
Then start you Android Studio, open `backend/android` as project and build the Android app.
