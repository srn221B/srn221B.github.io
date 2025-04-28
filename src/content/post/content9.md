---
title: "Hadoop周辺の設定メモ"
publishedAt: 2023-02-11
description: "雑記録"
slug: "content9"
isPublish: true
tags: ["Hadoop"]
---


## hive関連
- [https://cwiki.apache.org/confluence/display/hive/configuration+properties](https://cwiki.apache.org/confluence/display/hive/configuration+properties)参考
    - hive*から始まるプロパティはHiveのシステムプロパティとみなされる。なお「hive.conf.validation」でいじれる。

|key|default value|about|
|---|---|---|
|hive.tez.container.size|-1|tezのコンテナを使うメモリサイズ|
|hive.tez.auto.reducer.parallelism|false|reducerの並列有効化|
|hive.exec.reducers.bytes.per.reducer|256,000,000|1つのreducer辺りの処理サイズ|
|hive.exec.dynamic.partition.mode|stric|dynamic partitionを使用するときのモード。stricの場合partitionをselectで明示的に指定しなければいけない。|
|hive.exec.compress.output|false|queryの最終結果を圧縮するかどうかを決める。圧縮方式は「mapred.output.compress*」から取得|

## mapreduce関連
- https://software.fujitsu.com/jp/manual/manualfiles/m150005/j2ul1563/04z200/j1563-03-17-05-01.html
- mapred*はMRv1(org.apache.hadoop.mapred)
- mapreduceはMRv2(org.apache.hadoop.mapreduce)

|key|default value|about|
|---|---|---|
|mapred.reduce.tasks|-1|Hadoopジョブで使用するReduceタスク数|
|mapred.output.compression.codec||Hadoopジョブの主力するファイルを圧縮するときのCodecのClass|
|mapreduce.output.fileoutputformat.compress.codec|||

- 圧縮CodecのClass
    - DefaultCodec: DEFLATEアルゴリズムを使用したzlib形式により圧縮・伸長。圧縮ファイルを分散して処理することはできない
    - GzipCodec: DEFLATEアルゴリズムを使用したgzip形式により圧縮・伸長。分散して処理することはできないがgzipコマンドを使用して参照することはできる。
    - Bzip2Codec: bzip2アルゴリズムを使用したbzip2形式により圧縮・伸長。分散して処理できるが、圧縮・伸長性能は劣る。bzipコマンドを使用して参照することができる。
    - SnappyCodec: Snappyアルゴリズムを使用したSnappy形式により圧縮・伸長。分散して処理することはできないが、圧縮・伸長性能はたかい。Snappyネイティブライブラリをインストールする必要がある。
    - https://hadoop.apache.org/docs/r2.7.2/api/org/apache/hadoop/io/compress/package-summary.html

## hdfs(NameNode)関連

|key|default value|about|
|---|---|---|
|io.compression.codecs||使用できる圧縮するときのCodecのclass|

## tez関連
- https://docs.cloudera.com/HDPDocuments/HDP2/HDP-2.6.3/bk_command-line-installation/content/set_up_tez_for_ui.html

|key|default value|about|
|---|---|---|
|tez.queue.name||tezのqueue名|
|tez.tez-ui-history-url.base|http://<webserver-host:9999/tez-ui/|TezUIのホスト|
|tez.am.view-acls||View権限を与えるacl|
|||

