# PTT BBS in Rust

這個 Project 的目標在於使用 [Rust][3] 這個全新的 System Language 來重新撰寫 PTT。

## Why Rust ?

[PTT BBS 站][1] 已經運行了數十年之久，程式碼最初採用 C 語言來撰寫，並[開源於 Github][2] 上。

Rust 語言具有不會有 Segmentation Falut、不需自己管理記憶體 (但是也不用 Garbage Collection 等方式降低執行效能)、近似於 C 效能等特性。同時該語言融合了許多近期新興語言的特色，像是 Pattern Matching、Closures、Generics 等等，並且在於 Concurrency 的方面具有許多 Native Support。我認為 Rust 很適合用來開發大量用戶同時連線的 PTT，而且相較於 C 更容易維護。

Rust 語言目前由 [Mozilla 公司][4]開發維護，並且正以 Rust 與開發 [Firefox][5] 的經驗，重新撰寫新的[瀏覽器核心][6]。現在也有其它專案正在重新以 Rust 替換掉原本以 C 撰寫的程式。

[1]: https://www.ptt.cc/index.html
[2]: https://github.com/ptt/pttbbs
[3]: https://www.rust-lang.org/en-US/
[4]: http://mozilla.com.tw/
[5]: https://www.mozilla.org/zh-TW/firefox/new/
[6]: https://github.com/servo/servo
