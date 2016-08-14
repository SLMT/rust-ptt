# PTT BBS in Rust

[![Build Status](https://travis-ci.org/SLMT/rust-ptt.svg?branch=master)](https://travis-ci.org/SLMT/rust-ptt)

這個 Project 的目標在於使用 [Rust][3] 這個全新的 System Language 來重新撰寫 PTT。

## Why Rust ?

[PTT BBS 站][1] 已經運行了數十年之久，程式碼最初採用 C 語言來撰寫，並[開源於 Github][2] 上。

Rust 語言具有不會有 Segmentation Falut、不需自己管理記憶體 (但是也不用 Garbage Collection 等方式降低執行效能)、近似於 C 效能等特性。同時該語言融合了許多近期新興語言的特色，像是 Pattern Matching、Closures、Generics 等等，並且在於 Concurrency 的方面具有許多 Native Support。我認為 Rust 很適合用來開發大量用戶同時連線的 PTT，而且相較於 C 更容易維護。

Rust 語言目前由 [Mozilla 公司][4]開發維護，並且正以 Rust 與開發 [Firefox][5] 的經驗，重新撰寫新的[瀏覽器核心][6]。現在也有其它專案正在重新以 Rust 替換掉原本以 C 撰寫的程式。

## Current Status

現在這個程式具有一個基本的 TCP Server，聆聽 54321 port，可以接受外部連線。

接受到一筆連線後，會送出一些 telnet 的指令，並持續接收 client 的回覆。其中收到重設終端機大小的回覆之後，會將指定的大小印出來。(已經過 PCMan Client 測試)

## Try It !!

假設你的電腦已經安裝好 [Rust][3] 並透過 [Git][8] 將這個 repository 下載下來。

透過 shell 切換到 `rust-ptt` 資料夾下之後，輸入：

```bash
> cargo run
```

這個動作會透過 Rust 的套件管理工具 [Cargo][9] 來將程式碼進行 compile：

```
Compiling rust-ptt v0.1.0 (file:///some-places/rust-ptt)
```

然後自動啟動程式：

```
Running `target/debug/rust-ptt`
```

然後隨便找一個拿來連 PTT 的 client 或者 telnet client 來連上 server (網址：`localhost:54321`)，此時 server 這邊應該會顯示：

```
Start a connection to 127.0.0.1:54689
Width: 80, Height: 24
```

這個訊息代表 client 的 address，以及 client 請求的 terminal 大小。注意 address 不一定會這個例子相同，而且顯示的長寬也會隨著你的 client 的設定而有所不同。Client 方面暫時不會顯示任何資訊。

[1]: https://www.ptt.cc/index.html
[2]: https://github.com/ptt/pttbbs
[3]: https://www.rust-lang.org/en-US/
[4]: http://mozilla.com.tw/
[5]: https://www.mozilla.org/zh-TW/firefox/new/
[6]: https://github.com/servo/servo
[7]: https://github.com/vanilladb/vanillacore
[8]: https://git-scm.com/
[9]: https://crates.io/
