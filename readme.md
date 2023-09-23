# 実行
次のコマンドを打つことで、`tmp/test.s`にアセンブリが生成されます。<br>
`cargo run "[コンパイルする文字列]"`

# テストコードの実行
`src/test.sh` にデバッグ用のスクリプトがあります。`assert 0 "3-3"`のように、第一引数に想定される値、第二引数にコンパイルする文字列を入力して利用します。<br>
`bash src/test.sh`

# 参考
- https://qiita.com/AtsukiTak/items/0819ee57af2639891ecf
- https://www.sigbus.info/compilerbook