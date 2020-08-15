cpuid_rs
===

これなに
---
- 実行したら x86 の CPUID 命令を叩いて vendor, processor signature を出力するだけのプログラム
- New asm! を使うので nightly じゃないとビルドできません
- asm! 書いてみたかっただけです

参考文献
---
- インテル® プロセッサの識別とCPUID命令 https://www.intel.co.jp/content/dam/www/public/ijkk/jp/ja/documents/developer/Processor_Identification_071405_i.pdf
- New inline assembly syntax available in nightly https://blog.rust-lang.org/inside-rust/2020/06/08/new-inline-asm.html
- Inline assembly (RFC 2873) https://github.com/rust-lang/rfcs/pull/2873
    - Tracking Issue for inline assembly (`asm!`) https://github.com/rust-lang/rust/issues/72016
