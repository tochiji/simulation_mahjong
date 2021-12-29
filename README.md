# 麻雀の半荘繰り返しシミュレーション

麻雀の半荘を繰り返し、最終的なポイントの分布を確認するだけのシミュレーションツールです。

1着 +60pt、2着 +15pt、3着 -15pt、4着 -60ptとして、完全にランダムな着順で1000半荘実施することを1000回繰り返します。

プレイヤーは14人です。

1000半荘実施後、ポイントの大きさ順に1位〜14位が決まります。
このプログラムでは、各回1位の人間のポイントを小さい順にただ表示します。

麻雀が完全な運のゲームだと仮定した時に、どの程度ポイントが分散するのかを確認するためのプログラムです。

## ローカルでの立ち上げ

```console
$ cargo run > result.txt
```

これで、1000回分の最大ポイントが昇順に表示されます。

```console
$ cat result.txt
555
555
570
600
615
630
630
660
660
675
675
675
690
...
```

## ヒストグラム表示
Rを使用します。

```console
$ R
R version 4.1.2 (2021-11-01) -- "Bird Hippie"
Copyright (C) 2021 The R Foundation for Statistical Computing
Platform: aarch64-apple-darwin20.6.0 (64-bit)

R は、自由なソフトウェアであり、「完全に無保証」です。 
一定の条件に従えば、自由にこれを再配布することができます。 
配布条件の詳細に関しては、'license()' あるいは 'licence()' と入力してください。 

R は多くの貢献者による共同プロジェクトです。 
詳しくは 'contributors()' と入力してください。 
また、R や R のパッケージを出版物で引用する際の形式については 
'citation()' と入力してください。 

'demo()' と入力すればデモをみることができます。 
'help()' とすればオンラインヘルプが出ます。 
'help.start()' で HTML ブラウザによるヘルプがみられます。 
'q()' と入力すれば R を終了します。

> A=scan("results.txt")
Read 1000 items
> hist(A,  main="ランダム1000半荘を1000回実施した時の総合1位ポイント分布", xlab="総合1位のポイント", ylab="出現回数")
```