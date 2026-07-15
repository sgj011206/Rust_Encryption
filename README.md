[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) 

[![Coverage Status](https://coveralls.io/repos/github/sgj011206/Rust_Encryption/badge.svg?branch=main)](https://coveralls.io/github/sgj011206/Rust_Encryption)

![build](https://github.com/sgj011206/Rust_Encryption/actions/workflows/build.yml/badge.svg)
# Rust_Encryption
Rustで書かれたシンプルなファイル暗号化・復号ツール。

## Description
このプロジェクトは、Rustの学習とファイルI/O処理の実践を目的とした制作物です。
強力な暗号化アルゴリズムAES-256-GCMを使用して、個人のファイルを簡単に保護できます。
複雑な設定なしで初心者でもすぐに使えるソフトウェアを目指しました。

## Usage
以下のコマンドを実行してヘルプを表示します：

help

ファイルの暗号化：

encrypt file_path

ファイルの復号化：

decrypt file_path

新しいランダムキーまたはキーファイルを生成します：

keygen

バージョン情報を表示します：

version

## Security Warning

鍵は必ず安全に保管してください。鍵を紛失した場合、データは復元できません。


## Development environment

Mac OS,Visual Studio Code

## license

このプロジェクトはMITライセンスで公開されています。

## author

sgj011206
