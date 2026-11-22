---
title: "Airflowのschedule関連について"
publishedAt: 2022-01-11
description: "Airflowのschedule関連について"
slug: "content2"
isPublish: true
tags: ["Airflow"]
---

## 概要
Airflowのscheule関連についてしっかりと理解できていなかったので、DataPipelines with ApacheAirflowのCAPTER3「Scheduling in Airflow」を参考にしつつ雑にまとめてみた。

## TL;DR
- DAGは「schedule_interval」に設定した間隔で実行される。
- 「schedule_interval」はcronとtimedeltaインスタンスで書くことができる。
- 「interval is started」「the end of the interval」
- 「execution date」は実際の実行時間ではない。
- JinjaTemplateを使用して動的に設定することによってデータを段階的に処理できる。
- DAGはbackfillを使うと過去分実行ができる。

## schedule_intervalについて
- DAGの定期的な実行間隔は　airflow.dagの「scheudle_interval」引数に指定する。
### 「None」の場合、UIもしくはAPIからの実行のみがトリガーとなる。
```python
dag = DAG(
    dag_id="unscheduled",
    start_date=dt.datetime(2019, 1, 1),
    schedule_interval=None
)
```

### 「Cron」を使用することができる。
- 0 * * * *
- 0 0 * * MON-FRI
    - 値をrangeで渡すこともできる（例は平日の0時に実行）
- 0, 0,12 * * *
    - 値をリストで渡すこともできる（例は0,12時に実行）

```python
dag = DAG(
    dag_id="use_cron",
    start_date=dt.datetime(2019, 1, 1),
    schedule_interval="0 0 * * *",
)
```

### 「@daily」などのairflowが提供しているmacroも使用することができる。
- https://airflow.apache.org/docs/apache-airflow/1.10.4/scheduler.html
```python
dag = DAG(
    dag_id="use_macro",
    start_date=dt.datetime(2019, 1, 1),
    schedule_interval="@daily"
)
```
- 上記の場合、初回実行時間は「2019, 1, 2の00:00」

「start_date」から「schedule_interval」を足した時間(start + interval)が初回実行になるため。

![スクリーンショット 2022-08-07 15.58.34.png](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/289561/cbe483c0-42e4-3dd3-c180-a22a7393be5f.png)


### 「timedeltaインスタンス」を渡して使用することもできる。
これによってCronなどで実現できない「n日間隔で実行する」などのschedulingができる。
```python
import datetime at dt

dag = DAG(
    dag_id="use_timedelta",
    schedule_interval=dt.timedelta(day=3),
    start_date=dt.datetime(year=2019, month=1, day=1),
)
```
- 上記の場合、初回実行時間は「2019,1,4の00:00」

「start_date=2019,1,1からschedule_interval=３日後(2019,1,3)分の実行となる」

### 「end_date」を使用して最終実行時間を指定することもできる。
```python
dag = DAG(
    dag_id="use_enddate",
    start_date=dt.datetime(2019, 1, 1),
    schedule_interval="@daily",
    end_date=dt.datetime(2019, 1, 5)
)
```
- 上記の場合、最終実行時間は「2019, 1, 5の00:00」
![スクリーンショット 2022-08-07 16.00.18.png](https://qiita-image-store.s3.ap-northeast-1.amazonaws.com/0/289561/dd523315-030d-dd1a-233e-004f68861307.png)



## 日付関連のJinja Templateについて
- 実行日時を取得したい場合「{{ execution_date }}」を使用してtimestamp型の実行日時を取得することができる。
    - YYYY-MM-DDのみ取得したい場合は「{{ ds }}」。
    - YYYYMMDDのみ取得したい場合は「{{ ds_nodash }}」。

### Operatorに直接書くことができる。
```python
BashOperator(
    task_id="fetch_events",
    bash_command={
        "mkdir -p /data/events && "
        "start_date={{ds}}&end_date={{next_ds}}"
    }
)
```
### 引数に渡すこともできる。
```python
def _function_test(**context):
    """function test"""
    print(f"{context['templates_dict']['start_date']}"
          f"{context['templates_dict']['end_date']}")

fetch_events = PythonOperator(
    task_id="fetch_events",
    python_callable=_function_test,
    templates_dict={
        "start_date": "start_date={{ds}}",
        "end_date": "end_date={{next_ds}}"
    }
)
```
### interval-basedな日付の取得の仕方をするので注意
- 例えば、start_dateが2019,1,1でschedule_intervalが@dailyのDAGが初回実行される日付は「2019,1,2」だが、「2019,1,2」のジョブで取得される「{{ds}}」は「2019,1,1(前日)」である。
    - そのため、「previous_execution_date」や「next_execution_date」をジョブ内で使用するようなDAGをかく場合は、「start_date」~「end_date」外の値を取得しようとしていないかなど注意が必要。

## Backfillについて
- AirflowにはDAGをactiveにした時に、start_dateからactive時までのスケジュール間隔内で実行されていない過去ジョブがあった場合、自動で実行してくれるBackfillという機能がある。
    - ３０日以内のみ有効らしい

### defaultで有効になっているが無効にしたい場合は「catchup」をFalseに
- dag内に書くこともできるし
```python
dag = DAG(
    dag_id="catchup_test",
    schedule_interval="@daily",
    start_date=dt.datetime(year=2019, month=1, day=1),
    catchup=False
)
```
- airflow.cfgの「catchup_by_default」をFalseにすることでもできる。
```python
catchup_by_default = False
```