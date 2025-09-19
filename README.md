# ELF parser

LinuxなどのUNIXにおいて実行やライブラリの形式として採用されているELF(_Executable and Linkable Format_)ファイルをパースし、記述されているデータを取得することを目指します。
`readelf`のようなコマンド形式で提供するというよりは、ローダなどを作成する際に解析に使える形を主に考えています。そのため、ライブラリcrateとして公開するかもしれません。

This crate targets to parse and get information of ELF(_Executable and Linkable Format_); executable and library file format used in some UNIX OS such as Linux.
I do not think providing with parse command like `readelf` but mainly think providing with library for who wants to create loader program or something, so I may publish this as library crate.
