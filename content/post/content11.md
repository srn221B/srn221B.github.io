---
title: "GreatExpectationsのActionについて"
publishedAt: 2023-05-07
description: "gx"
slug: "content11"
isPublish: true
tags: ["GreatExpectations","Python"]
---



## Actionとは
GreatExpectations(GX)で得たValidationResult(検証結果)を受け取った後に行う処理をする機能。

### Actionの種類
例えば次のようなActionがある。
|class名|できること|
|---|---|
|SlackNotificationAction|Slackへの通知|
|PagerdutyAlertAction|PagerDutyへのEvent通知|
|MicrosoftTeamsNotificationAction|Teamsへの通知|
|OpsgenieAlertAction|OpsgenieへのAlert通知|
|EmailAction|Emailの送付|
|StoreValidationResultAction|ValidationStoreへ検証結果の保存|
|StoreEvaluationParametersAction|ValidationStoreへ評価値の保存(Expectation SuiteのValidationResultsにあるEvaluationParameter)|
|StoreMetricsAction|MetricStoreへ検証結果のMetricの保存|
|UpdateDataDocsAction|DataDocの更新|
|SNSNotificationAction|AmazonSNSへの通知|

また、[ValidationAction](https://docs.greatexpectations.io/docs/reference/api/checkpoint/validationaction_class/)Classを継承し`_run` メソッドを上書きすればカスタマイズした独自のActionを追加することもできる。


### 使い方について
複数のBatchRequestを検証するCheckPointにActionは構成される。そのため、使用する際はdata_contextへ`add_checkpoint`を用いてCheckPointを追加するとき、もしくはdata_contextへ`run_checkpoint`を用いてCheckPointを実際に実行するときに、Actionを定義する。

例えば「data_contextへ`add_checkpoint`を用いてCheckPointを追加するとき」のActionの定義の仕方は次のように行う。
```python
yaml_config = """
・・・
validations:
  - expectation_suite_name: users.warning 
  - expectation_suite_name: users.error  
    action_list:
    - name: quarantine_failed_data
      action:
          class_name: CreateQuarantineData
    - name: advance_passed_data
      action:
          class_name: CreatePassedData
action_list:
    - name: store_validation_result
      action:
        class_name: StoreValidationResultAction
    - name: store_evaluation_params
      action:
        class_name: StoreEvaluationParametersAction
    - name: update_data_docs
      action:
        class_name: UpdateDataDocsAction
・・・ 
"""
context.add_checkpoint(**yaml.load(yaml_config))
```
「.validations.expectation_suite_name.action_list」に書かれている各Actionは、紐づかれているexpectation_suite(ここではuser.error)の検証結果に特化したActionの定義である。
「.action_list」に書かれている各Actionは、全expectation_suiteの検証結果に対するActionの定義である。

「data_contextへ`run_checkpoint`を用いてCheckPointを実際に実行するとき」のActionの定義の仕方は次のように行う。
```python
checkpoint_run_result: CheckpointResult = data_context.run_checkpoint(
    ・・・
    action_list=[
        {
            "name": "store_validation_result",
            "action": {"class_name": "StoreValidationResultAction"}
        },
        {
            "name": "store_evaluation_params",
            "action": {"class_name": "StoreEvaluationParametersAction"}
        }
    ]
    ・・・
)
```

## StoreMetricsActionを実際に試してみる
Actionのうち、「StoreMetricsAction」を試し、実際にMetricStoreへ検証結果のMetricが保存されることを確認する。
[公式のHow to](https://docs.greatexpectations.io/docs/guides/setup/configuring_metadata_stores/how_to_configure_a_metricsstore)を参考に試してみる。


### 1.事前準備
- [GreatExpectationのセットアップ](https://467tn.com/post/content6/#1setup)
- 適当なDataSourceとBatchRequest,ExpectationSuiteの準備
### 2.MetricStoreの追加
GreatExpectationsへMetricStoreの情報を追加する。参考と同じく、今回はMetricStoreをPostgresとする。
#### great_expectations/uncommitted/config_variables.ymlへPostgresの認証情報の追加

great_expectations.ymlへcredentialsを記載するための認証情報をconfig_variables.ymlへ書く。

```yml
my_postgres_db_yaml_creds:
  drivername: postgresql
  host: ${MY_DB_HOST}
  port: 5432
  username: postgres
  password: ${MY_DB_PW}
  database: metric_store_sample
```
#### `MY_DB_HOST`と`MY_DB_PW`のexport

上記config_variables.ymlで記載してstring.Templateがloadできるようにexportする。

```bash
export MY_DB_HOST=''
export MY_DB_PW=''
```
#### great_expectations/great_expectations.ymlへMetricStoreの追加

Store名「sample_metric_store」として追加する。
ちなみに.stores.metric_store.store_backendについては、現在「DatabaseStoreBackend」のsubclassのみ対応されているようだ。
```yml
stores:
・・・・
  sample_metric_store:
    class_name: MetricStore
    store_backend:
      class_name: DatabaseStoreBackend
      credentials: ${my_postgres_db_yaml_creds}
```

### 3.CheckPointの作成
今回は[こちらの検証結果](https://467tn.com/post/content6/#%E7%B5%90%E6%9E%9C%E3%82%92%E8%A1%A8%E7%A4%BA%E3%81%97%E3%81%A6%E3%81%BF%E3%82%8B)をもとに以下のようなCheckPointを作成してみる。
```python
## 事前準備
batch_request:BatchRequest
expectation_suite_name:str

## DataContextは初期化
import great_expectations as gx
context: AbstractDataContext = gx.get_context()

# checkpointをdata_contexthe追加
checkpoint_name = "my_checkpoint"
context.add_checkpoint(
    name=checkpoint_name,
    config_version=1,
    class_name="Checkpoint",
    validations=[
        {
            "expectation_suite_name": suite_name,
        }
    ],
    batch_request=batch_request,
    action_list=[
        {
            "name": "store_metric", # 任意のAction名
            "action": {
                "class_name": "StoreMetricsAction",
                "target_store_name": "sample_metric_store", # great_expectations.ymlへ追加したStore名
                "requested_metrics":{ # 保存するMetricsについて列挙
                    "*": [ # 全ExpectationSuite対象
                        "statistics.successful_expectations", # A
                        "statistics.unsuccessful_expectations" # B
                    ],
                    suite_name: [ # 任意のExpectationSuite対象
                        {"column": {
                            "int_column": ["expect_column_values_to_be_between.result.unexpected_count"] # C
                        }}
                    ]
                }
            }
        }
    ]
)
```
こちらのCheckPointを実行すれば次の3つのMetricがMetricStoreへ保存されるはずだ。
- A・・・全ExpectationSuiteの成功したExpectation数
- B・・・全ExpectationSuiteの成功しなかったExpectation数
- C・・・任意のExpectationSuiteのExpectation:expect_column_values_to_be_betweenのint_columnが期待通りではなかった数

そのほか「statistics」や各Expectationの「result」から以下のようなMetircの取得を指定できる。（ExpectationのTypeによっては取得できないものもあるかもしれません。）
|Metric|内容|
|---|---|
|statistic.evaluated_expectations|評価したExpectationの数|
|statistic.successful_expectations|成功したExpectationの数|
|statistic.unsuccessful_expectations|成功しなかったExpectationの数|
|statistic.success_percent|成功のパーセント表記|
|*.result.element_count|期待通りだったカラムの数|
|*.result.unexpected_count|期待通りではなかったカラムの数|
|*.result.unexpected_percent|期待通りではなかったパーセント表記|
|*.result.partial_unexpected_list|期待通りではなかったカラムの値のリスト|

### 4.CheckPointを実行してみる
```python
context.run_checkpoint(checkpoint_name=checkpoint_name)
```
をしてCheckPointを実行する。

実行すると「Calculating Metrics: 100%」というログとともに、MetricStore上に`ge_metrics`というTBLが自動的に作られ、取得を指定したA〜CまでのMetricが挿入されていることを確認できる。
```sql
# select * from ge_metrics;
 run_name |        run_time         |  data_asset_name   | expectation_suite_identifier |                        metric_name                         | metric_kwargs_id  |      value       
----------+-------------------------+--------------------+------------------------------+------------------------------------------------------------+-------------------+------------------
 __none__ | 20230507T162308.788052Z | public.sample_poyo | ex_suite                     | statistics.successful_expectations                         | __                | {"value": 3}
 __none__ | 20230507T162308.788052Z | public.sample_poyo | ex_suite                     | statistics.unsuccessful_expectations                       | __                | {"value": 1}
 __none__ | 20230507T162308.788052Z | public.sample_poyo | ex_suite                     | expect_column_values_to_be_between.result.unexpected_count | column=int_column | {"value": 2}
```

- 全ExpectationSuiteのうち、成功したExpectation数は3つ
- 全ExpectationSuiteのうち、成功しなかったExpectation数は1つ
- Expectation:expect_column_values_to_be_betweenのint_columnが期待通りではなかった数は2つ

という意味だ。

## 終わり
GreatExpectationsのActionの概要、およびActionのうちStoreMetricsActionについて簡単にですが紹介しました。Actionを自作できることや、StoreMetricsActionの場合保存したいMetricを柔軟に指定できる部分が良いなと思いました。