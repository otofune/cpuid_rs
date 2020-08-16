cpuid_rs
===

これなに
---
- 実行したら x86 の CPUID 命令を叩いて VMware の vmx ファイルに適用できる設定列を生成します
    + `eax=0` のすべて
    + `eax=1` の processor signature
        * 注意: processor signature から機能の差異を再現するようで、それを切らないと Intel <-> AMD 間でこの値を変えることはできない
        * `featureCompat.enable = "FALSE"` で切れます
    + `eax=80000002f ~ 80000004f` の brand string
- New asm! を使うので nightly じゃないとビルドできません
- asm! 書いてみたかっただけです

参考文献
---
- インテル® プロセッサの識別とCPUID命令 https://www.intel.co.jp/content/dam/www/public/ijkk/jp/ja/documents/developer/Processor_Identification_071405_i.pdf
- New inline assembly syntax available in nightly https://blog.rust-lang.org/inside-rust/2020/06/08/new-inline-asm.html
- Inline assembly (RFC 2873) https://github.com/rust-lang/rfcs/pull/2873
    - Tracking Issue for inline assembly (`asm!`) https://github.com/rust-lang/rust/issues/72016
