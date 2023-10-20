# 実行
次のコマンドを打つことで、アセンブリが出力されます。<br>
`cargo run "[入力ファイルパス] [出力パスファイル]"`

# テストコードの実行
`test/test.sh` にデバッグ用のスクリプトがあります。`assert 0 "3-3"`のように、第一引数に想定される値、第二引数にコンパイルする文字列を入力して利用します。<br>
`bash test/test.sh`

# 参考
- https://qiita.com/AtsukiTak/items/0819ee57af2639891ecf
- https://www.sigbus.info/compilerbook
