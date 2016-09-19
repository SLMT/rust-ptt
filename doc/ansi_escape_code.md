# ANSI 跳脫代碼 (ANSI Escape Code)

ANSI 跳脫代碼是一種用來控制文字終端機上的色彩、格式等各種顯示方式的一種方法。每一段跳脫代碼由多個字元組成。其中都以 `ESC` (ASCII 十進位為 `27`、十六進位為 `0x1B`) 字元為開頭。第二個字元之後則為 ASCII `64` 到 `95` 之間的任一字元。

代碼可以再以第二個字元是否為 `[` (ASCII 十進位為 `91`、十六進位為 `0x5B`) 來進行分類。第二個字元為 `[` 的代碼稱為 CSI (Control Sequence Introducer) 代碼，為其它字元者則稱為 Non-CSI 代碼。

以上都假設環境為 7-bit，也就是每個字元只能用 0 ~ 127 的數字表示。若在 8-bit 環境之下，每個字元可以有 256 種表示方式。某兩個跳脫代碼的字元可以被代換為 `0x80` 到 `0x9F` 的某個字元。不過 7-bit 仍比較常用，而且 PTT 似乎只支援 7-bit 環境。

## 參考資料

[Wiki - ANSI Escape Code](https://en.wikipedia.org/wiki/ANSI_escape_code)
