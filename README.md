# wasm-engine
wasm-engine

1.1 Introduction

WebAssembly (abbreviated Wasm) is a safe, portable, 
low-level code format designed for efficient execution and compact representation. 
WebAssembly（略してWasm）は安全、ポータブルで低レイヤーに関するコードフォーマットです。それは効率的な実行とコンパクトな表現を目指して設計されています。

Its main goal is to enable high performance applications on the Web, 
but it does not make any Web-specific assumptions or provide Web-specific features, 
so it can be employed in other environments as well.
その主要な目的は、Webでのハイパフォーマンスなアプリケーションを可能にすることです。
しかしそれはWeb特有の過程を置いたり、Web特有の特徴を提供したりしません。
したがってそれは他の環境でも同様に援用されることができます。

WebAssembly is an open standard developed by a W3C Community Group.
WebAssemblyはオープンな標準であり、W3C Community Groupによって開発されています。

This document describes version 1.0 of the core WebAssembly standard. 
It is intended that it will be superseded by new incremental releases with additional features in the future.
このドキュメントは、the core WebAssembly standardのバージョン1.0について記述しています。
それは将来、追加機能が入った新しいincrementalなリリースによって置き換えられるでしょう。

1.1.1 Design Goals

The design goals of WebAssembly are the following:
WebAssemblyの設計目標は以下です。

• Fast, safe, and portable semantics:
高速、安全、そしてポータブルなセマンティクス

- Fast:
executes with near native code performance, taking advantage of capabilities common to all contemporary hardware.
- 高速:
全ての現代的なハードウェアに共通なcapabilitiyを利用した、ネイティブに近いパフォーマンスでの実行

- Safe:
code is validated and executes in a memory-safe, sandboxed environment preventing data corruption or security breaches.
- 安全:
コードは検証され、memory-safeでサンドボックスな環境の中で、データ破壊やセキュリティ脅威を防ぎつつ、実行される。

- Well-defined:
fully and precisely defines valid programs and their behavior in a way that is easy to reason about informally and formally.
- Well-defined:
妥当なプログラムとそれらの振る舞いが、完全で正確に、形式的にも非形式的にも理屈づけるのが容易な方法で定義される。

- Hardware-independent: 
can be compiled on all modern architectures, desktop or mobile devices and embedded systems alike.
- ハードウェア非依存:
デスクトップ・モバイル機器や組込システムなど、全てのモダンなアーキテクチャ上で、同じようにコンパイル可能である。

- Language-independent:
does not privilege any particular language, programming model, or object model.
- 言語非依存:
特定の言語、プログラミングモデルやオブジェクトモデルを特別扱いしない。

- Platform-independent:
can be embedded in browsers, run as a stand-alone VM, or integrated in other environments.
- プラットフォーム非依存:
ブラウザに埋め込め、スタンドアロンなVMとして実行、また他の環境と統合することもできる。

- Open:
programs can interoperate with their environment in a simple and universal manner.
- オープン:
プログラムはそれが属する環境と、単純で普遍的な方法で相互運用できる。

• Efficient and portable representation:
効率的でポータブルな表現:

- Compact:
has a binary format that is fast to transmit by being smaller than typical text or native code formats.
- コンパクト
典型的なテキストやネイティブコードフォーマットよりも小さく、高速で転送できるバイナリフォーマットを持つ。

- Modular:
programs can be split up in smaller parts that can be transmitted, cached, and consumed separately.
- モジュラー
プログラムは、別々に転送したりキャッシュしたり消費したりできるような小さなパーツに分割可能である。

- Efficient:
can be decoded, validated, and compiled in a fast single pass, equally with either just-in-time (JIT) or ahead-of-time (AOT) compilation.
- 効率的:
JITやAOTコンパイルと同様、高速なシングルパスでデコード、検証、コンパイルが可能。

- Streamable:
allows decoding, validation, and compilation to begin as soon as possible, before all data has been seen.
- ストリーム化可能
全てのデータをみる前に、できるだけ早くデコード、検証、コンパイルを開始できる。

- Parallelizable:
allows decoding, validation, and compilation to be split into many independent parallel tasks.
- パラメータ化可能:
デコード、検証、コンパイルを多くの独立した並行処理に分割することを許す。

- Portable:
makes no architectural assumptions that are not broadly supported across modern hardware.
- ポータブル
モダンなハードウェア間でそれほどサポートされていないような、アーキテクチャ上の仮定は設けない。

WebAssembly code is also intended to be easy to inspect and debug, especially in environments like web browsers,
but such features are beyond the scope of this specification.
WebAssemblyのコードは、特にWebブラウザでの調査とデバッグを容易にするように意図されているが、
それらの特徴はこの仕様の範囲外である。

1.1.2 Scope

At its core, WebAssembly is a virtual instruction set architecture (virtual ISA). 
As such, it has many use cases and can be embedded in many different environments. 
To encompass their variety and enable maximum reuse, the WebAssembly specification is split and layered into several documents.
コアとして、WebAssemblyは仮想的な命令セットアーキテクチャである。
それ自体、それは多くのユースケースをもち、多くの異なった環境に埋め込むことが可能である。
それらの多様性を包括するため、また最大限の再利用を可能にするため、WebAssemblyの仕様はいくつかのドキュメントに分割され、層を成している。

This document is concerned with the core ISA layer of WebAssembly. 
It defines the instruction set, binary encoding, validation, and execution semantics, as well as a textual representation. 
It does not, however, define how WebAssembly programs can interact with a specific environment they execute in, nor how they are invoked from such an environment.
このドキュメントは、WebAssembly ISA層に関わっている。
それは命令セット、バイナリエンコーディング、検証、実行のセマンティクス、そして同様にテキスト表現を定義している。
しかしながらそれは、WebAssemblyプログラムが、それが実行される特定の環境とどのように相互作用すべきか、またそれらがそのような環境からどのように呼び出されるのか、については定義していない。

Instead, this specification is complemented by additional documents defining interfaces to specific embedding environments such as the Web. 
These will each define a WebAssembly application programming interface (API) suitable for a given environment.
代わりにこの仕様は、（Webのような）特定の埋め込み環境とのインターフェースを定義する追加のドキュメントによって補完されている。
それらはそれぞれ、所定の環境に適したWebAssembly APIを定義しているだろう。

1.1.3 Dependencies

WebAssembly depends on two existing standards:
WebAssemblyは2つの既存の標準に依存している:

- IEEE 754-20084, for the representation of floating-point data and the semantics of respective numeric op-
erations.
- IEEE 754-20084。浮動小数点数を表すデータ表現と、それぞれの数値操作のセマンティクス。

- Unicode, for the representation of import/export names and the text format.
- Unicode。名前とそのテキストフォーマットのインポート/エクスポートに関する表現。

However, to make this specification self-contained, relevant aspects of the aforementioned standards are defined and formalized as part of this specification, such as the binary representation and rounding of floating-point values, and the value range and UTF-8 encoding of Unicode characters.
しかしながら、この仕様を自己充足的にするため、（バイナリ表現や浮動小数点数の丸め、Unicode文字の値の範囲やUTF-8でのエンコーディングのような）前述の標準で関係する部分はこの仕様の一部として定義、形式化されている。

Note:
The aforementioned standards are the authoritative source of all respective definitions. 
Formalizations given in this specification are intended to match these definitions. 
Any discrepancy in the syntax or semantics described is to be considered an error.
ノート:
前述の標準は、全てのそれぞれの定義の権威あるものである。
この仕様で与えられた形式化はそれらの定義と一致するように意図されている。
シンタックスやセマンティクスの記述内での不一致があれば、（それはこの仕様が）誤っていると考えるべきである。

1.2 Overview

1.2.1 Concepts

WebAssembly encodes a low-level, assembly-like programming language. 
This language is structured around the following concepts.
WebAssemblyは低レベルなアセンブリに似たプログラミング言語をエンコードする。
この言語は以下のような概念周辺で構造化されている。

Values
WebAssembly provides only four basic value types. 
These are integers and IEEE 754-20086 numbers, each in 32 and 64 bit width. 
32 bit integers also serve as Booleans and as memory addresses. 
The usual operations on these types are available, including the full matrix of conversions between them. 
There is no distinction between signed and unsigned integer types. 
Instead, integers are interpreted by respective operations as either unsigned or signed in two’s complement representation.

値
WebAssemblyは4種類の基本的な値型を提供するのみである。
それらは整数と、IEEE 754-20086での数値であり、それぞれ32ビット、64ビット幅を持つ。
32ビット整数は真偽値やメモリアドレスとしても提供される。
それらの型に対する通常の命令は、それらの間の変換に関する全ての組合せを含め、使用可能である。
符号あり、なしで整数型の違いはない。
代わりに整数は、それぞれの命令によって、2つの補完的な表現の中で、符号つきとも符号なしとしても、翻訳される。

Instructions
The computational model of WebAssembly is based on a stack machine. 
Code consists of sequences of instructions that are executed in order. 
Instructions manipulate values on an implicit operand stack and fall into two main categories. 
Simple instructions perform basic operations on data. 
They pop arguments from the operand stack and push results back to it. 
Control instructions alter control flow. 
Control flow is structured, meaning it is expressed with well-nested constructs such as blocks, loops, and conditionals. 
Branches can only target such constructs.

命令
WebAssemblyの計算モデルはスタックマシンに基づいている。
コードは（順番に実行される）命令の列からなる。
命令は、暗黙のオペランドスタック上の値を取り扱い、2つの主要なカテゴリに分類される。
単純な命令はデータに対する基本的な命令を行う。
それらはオペランドスタックから引数をpopして、結果をpushしてそれに戻す。
制御命令は、制御構造を変更する。
制御構造は構造化されている。つまり、ブロック、ループ、条件などのような十分にネストした構造物で表現されているということだ。
分岐はそれらの構造物のみをターゲットとすることができる。