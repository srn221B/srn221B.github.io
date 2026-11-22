---
title: "Great Expectationsについて調べてみた"
publishedAt: 2022-12-11
description: "調べたこと・試したこと"
slug: "content6"
isPublish: true
tags: ["GreatExpectations","Python"]
---


この記事は[Qiita x Code Polaris共催！女性ITエンジニアが作るアドベントカレンダー Advent Calendar 2022](https://qiita.com/advent-calendar/2022/qiita-code-polaris)の15日目の記事です。Qiitaのアドベントカレンダーに参加するの初めてです（wktk）
## 概要
[Great Expectations](https://docs.greatexpectations.io/docs/)(NOT V2)についてドキュメントを読みながらわかったこと、試したことをまとめる。
## Great Expectationsとは
データのバリデーション(検証)、ドキュメント化、プロファイリング(概要定義)をしてくれるPythonのライブラリ。Superconductive社が開発しているOSS。GXと略されるらしい。
### できること
- **データの期待の定義(Expectation)**
    - oO(xxxのカラムはNULLがないはずだ！)
    - oO(yyyのカラムはAAもしくはBBしか値が入らないはずだ！)
    - といったような期待の定義。
- **データの自動的なプロファイリング(Profiling)**
    - 「xxxのカラムはNULLがないです」
    - 「yyyのカラムはAAもしくはBBしか値が入らないです」
    - みたいなデータの概要の自動的な定義
- **データのバリデーション(Validation)**
    - 定義したExpectationが期待通りかの検証。
- **データのドキュメント化**
    - 定義したExpectationをValidationした結果の確認。
- **さまざまなデータソースやデータストアからのロード**
    - PandasやSparkなどのDataframeや、SQLDatabase及びファイルからのデータロード

### 何が嬉しいのか

データの品質管理ができる ->　バグ,過去分データの修正の早期対応。

### 似ているもの

- [dbt](https://www.getdbt.com/)のテスト機能。

- [kubeflow](https://www.kubeflow.org/)のValidation機能。

### できないこと
- **Pipelineの実行**
    - あくまでPythonのライブラリ。
    - 例えばデータ検証をしたいとなった時、GXの機能を活かしてデータ検証のPipeline（流れ）を作っていくわけだが、作ったPipelineのステップ毎の実行はできるが、全体を通した実行は単体でできない。なので、Airflow,dbt,Prefect,Dagster,KedroのようなDAGを実行するツールと統合すると良い。　
- **データのバージョン管理**
    - データのバージョン管理するならDVCやQuikltなどを使うと良い。
- **Pythonの環境以外で最適な状態で動くこと**
    - Pythonでできているので
    - 違う言語やエコシステムから呼び出すとき、CLIから呼び出しなんとかするということもできるが、言語やエコシステムに沿ったものを選択する方が良い、（R->assertR, TensorFlow->TFDV）

### データ検証作成の流れ

では、GreatExpectationsを使用して例えばデータ検証を作成したいとなったら何をしたら良いの、となるのだが以下の画像が参考になる。

![](https://storage.googleapis.com/zenn-user-upload/d392dd1323a2-20221205.png)

> https://docs.greatexpectations.io/assets/images/data_context_does_for_you-df2eca32d0152ead16cccd5d3d226abb.png

補足すると

- Setup
    - 「DataContext」の作成
    - 必要あれば）拡張機能(Plugins)の設定やValidation結果などのメタデータを保存する場所の設定
- Connect to Data
    - 「DataSources」の作成
    - 「BatchRequest」/「RuntimeBatchRequest」の作成
- Create Expectations
    - Profilingするなどをして「ExpectationSuite」の作成
- Validate Data
    - Validatorを使用して、作成した「Expectation Suite」と「BatchRequest」のペアの検証
    - CheckPointを使用して、作成した「Expectation Suite」と「BatchRequest」のペアを複数まとめ、検証


| 用語 | 何 |
|---|---|
|DataContext|プロジェクトのようなもの|
|DataSources|検証したいデータのConnector|
|BatchRequest|DataSources内のどのデータかの定義(TBL指定)|
|RuntimeBatchRequest|DataSources内のどのデータかの定義(Query指定)|
|Expectation|データに対する期待|
|ExpectationSuite|Expectationの集まり|

1についてはCLIから行うことができる
2~4についてはCLIやPython,JupyterNotebookから行うことができる

## 試してみる

上記データ検証作成をなぞってみる。

- サンプルデータについて
    - SQLDB：PostgreSQL

- DML

```
CREATE TABLE sample_poyo (
  int_column INTEGER, 
  varchar_column VARCHAR(10),
  varchar_column2 VARCHAR(10));
```

- DDL

```
INSERT INTO sample_poyo VALUES (0, 'xxxx', 'zzzz');
INSERT INTO sample_poyo VALUES (1, 'xxxx', 'zzzz');
INSERT INTO sample_poyo VALUES (2, 'xxxx', 'zzzz');
INSERT INTO sample_poyo VALUES (3, 'xxxx', 'zzzz');
INSERT INTO sample_poyo VALUES (4, 'yyyy', 'zzzz');
INSERT INTO sample_poyo VALUES (5, 'yyyy', 'zzzz');
INSERT INTO sample_poyo VALUES (6, 'yyyy', 'zzzz');
INSERT INTO sample_poyo VALUES (7, 'yyyy', 'zzzz');
```
-  データ内容
```
 int_column | varchar_column | varchar_column2 
------------+----------------+-----------------
         0 | xxxx           | zzzz
         1 | xxxx           | zzzz
         2 | xxxx           | zzzz
         3 | xxxx           | zzzz
         4 | yyyy           | zzzz
         5 | yyyy           | zzzz
         6 | yyyy           | zzzz
         7 | yyyy           | zzzz
```
JupyterNotebookとCLIを使用してインタラクティブに上記データ検証作成をすることもできますが、やりたいことの都合上ここにおいては**基本的にPythonで完結するやり方でかいていきます。**
手っ取り早くコードを見たい方は1.Setupを行い、[https://github.com/srn221B/sample_gx](https://github.com/srn221B/sample_gx)こちらを参考にどうぞ。


### 1.SetUp
#### インストール
FYI: https://docs.greatexpectations.io/docs/guides/setup/installation/local/
```bash
pip install great_expectations
```
#### DataContextの作成
プロジェクト(DataContext)を作成する。
```bash
great_expectations init
```
```
Using v3 (Batch Request) API

  ___              _     ___                  _        _   _
 / __|_ _ ___ __ _| |_  | __|_ ___ __  ___ __| |_ __ _| |_(_)___ _ _  ___
| (_ | '_/ -_) _` |  _| | _|\ \ / '_ \/ -_) _|  _/ _` |  _| / _ \ ' \(_-<
 \___|_| \___\__,_|\__| |___/_\_\ .__/\___\__|\__\__,_|\__|_\___/_||_/__/
                                |_|
             ~ Always know what to expect from your data ~

Let's create a new Data Context to hold your project configuration.

Great Expectations will create a new directory with the following structure:

    great_expectations
    |-- great_expectations.yml
    |-- expectations
    |-- checkpoints
    |-- plugins
    |-- .gitignore
    |-- uncommitted
        |-- config_variables.yml
        |-- data_docs
        |-- validations

OK to proceed? [Y/n]: <press Enter>
```
Enterするとgreat_expectationsといいうディレクトリができ、構成ファイルができる。
```
.
├── checkpoints
├── expectations
├── great_expectations.yml
├── plugins
│   └── custom_data_docs
│       ├── renderers
│       ├── styles
│       │   └── data_docs_custom_styles.css
│       └── views
├── profilers
└── uncommitted # バージョン管理すべきではないコードが入っている
    ├── config_variables.yml
    ├── data_docs
    └── validations
```
### 2.Connect to Data
#### DataSourceを作成する。
![](https://storage.googleapis.com/zenn-user-upload/c42b612fc635-20221205.png)
> https://docs.greatexpectations.io/assets/images/datasource_works_for_you-4a7ec1df1f7383cbb399dcac296f8895.png

DataSourceは主に[Execution Engine](https://github.com/great-expectations/great_expectations/blob/e5249f4b445a22e015cbcb07f6e45ecf516e698f/great_expectations/execution_engine/execution_engine.py#L197)と[DataConnector](https://github.com/great-expectations/great_expectations/blob/e5249f4b445a22e015cbcb07f6e45ecf516e698f/great_expectations/datasource/data_connector/data_connector.py#L20)から成っている。今回はPostgreSQLに接続するためのDataSourceを作成するので[こちらのページ](https://docs.greatexpectations.io/docs/guides/connecting_to_your_data/database/postgres)を参考にしながら以下のように作成する。
```python
import great_expectations as ge
context = ge.get_context()

ps_user_name=''
ps_password=''
ps_host=''
ps_port=''
ps_db=''

datasource_config = {
    "name": "my_datasource",
    "class_name": "Datasource",
    "execution_engine": {
        "class_name": "SqlAlchemyExecutionEngine",
        "connection_string": \
		f"postgresql+psycopg2://{ps_user_name}:{ps_password}@{ps_host}:{ps_port}/{ps_db}",
    },
    "data_connectors": {
        "default_runtime_data_connector_name": {
            "class_name": "RuntimeDataConnector",
            "batch_identifiers": ["default_identifier_name"],
        },
        "default_inferred_data_connector_name": {
            "class_name": "InferredAssetSqlDataConnector",
            "include_schema_name": True,
        },
    },
}
```
DataSourceをテストしたいときはyamlをdumpしてtest_yaml_configに渡せばテストできる。
```python
context.test_yaml_config(yaml.dump(datasource_config))
```
#### 作成したDataSourceを追加する。
```python
context.add_datasource(**datasource_config)
```
追加すると「great_expectations.yml」の`datasources`にも`my_datasource`の内容が反映されていることが確認できる。
```
datasources:
  my_datasource:
・・・
```
#### BatchRequestを作成する。
Datasource内のどのデータを検証するかを[BatchRequest](https://github.com/great-expectations/great_expectations/blob/e5249f4b445a22e015cbcb07f6e45ecf516e698f/great_expectations/core/batch.py#L390)として作成する。今回はテーブル内の全データなので以下のようにかく。
```python
from great_expectations.core.batch import BatchRequest
tbl_name="public.sample_poyo"
batch_request = BatchRequest(
    datasource_name="my_datasource",
    data_connector_name="default_inferred_data_connector_name",
    data_asset_name=tbl_name,
)
```

### 3.Create Expectations
いきなりExpectationを作成しても良いが、データを理解するためにProfilingをしてみる。
#### Profiling)ExpectationSuite/Validatorの作成
Expectationを追加するためのExpectationSuiteと、ExpectationSuiteとBatchRequestのペアを[Validator](https://github.com/great-expectations/great_expectations/blob/e5249f4b445a22e015cbcb07f6e45ecf516e698f/great_expectations/validator/validator.py#L162)に定義する。
```python
suite_name = "profiling_suite"
context.create_expectation_suite(
    suite_name, overwrite_existing=True
)
validator = context.get_validator(
    batch_request=batch_request,
    expectation_suite_name=suite_name
)
```
#### Profiling)Profilerを作成しProfilingしてみる。
[UseConfigurableProfiler](https://github.com/great-expectations/great_expectations/blob/e5249f4b445a22e015cbcb07f6e45ecf516e698f/great_expectations/profile/user_configurable_profiler.py#L39)へValidatorを渡すとProfilerが作成できる。また、Profilerには以下が設定できる。
```
- excluded_expectations: List[str]
	- 　除外するExpectation
- ignored_columns: List[str]
	- 除外するcolumn
- not_null_only: bool
	- デフォルトでNULLかどうかを評価するExpectationがNULL率によって入るわけだが,こちらをTrueにするとNULL率に関係なくNOT NULLがmustなExpectationが追加される。
- primary_or_compound_key: List[str]
	- 主キーや複合キーを指定すると、uniqueかどうか評価するExpectationが追加される。
- table_expectations_only
	- こちらをTrueにすると、tableレベルのExpectationのみを作成する。
- そのほか：https://docs.greatexpectations.io/docs/guides/expectations/how_to_create_and_edit_expectations_with_a_profiler#optional-parameters
```
今回は、ignored_columnsでcolumn「varchar_column2」の除外を行う。
```python
from great_expectations.profile.user_configurable_profiler \
    import UserConfigurableProfiler

profiler = UserConfigurableProfiler(
	profile_dataset=validator,
	ignored_columns=['varchar_column2']
	)
expectation_suit = profiler.build_suite()
expectations = expectation_suit['expectations']
for expectation in expectations:
    print('---')
    print('expectation_type: ' + expectation.expectation_type)
    print('kwargs: ' + str(expectation.kwargs))
```
実行すると以下のようなProfiling結果を見ることができる
```
expectation_type: expect_column_values_to_not_be_null
kwargs: {'column': 'int_column'}
---
expectation_type: expect_column_values_to_not_be_null
kwargs: {'column': 'varchar_column'}
---
expectation_type: expect_column_values_to_be_in_set
kwargs: {'value_set': ['xxxx', 'yyyy'], 'column': 'varchar_column'}
```
- 1つ目と2つ目はcolumn「int_column」「varchar_column」にはNULLが入らない。
- 3つ目はcolumn「varchar_column」には'xxxx'か'yyyy'のvalueが入る。

という意味だ。
各expectation_typeやそれぞれのkwargsについては[こちらのページ](https://greatexpectations.io/expectations/?filterType=Backend%20support&gotoPage=1&showFilters=false&viewType=Summary)から調べることができる。
#### Expectationの作成
データについて理解できたので、Expectationを作成してみる。具体的にはProfiling結果を参考に以下のExpectationをここでは定義する。

- 1.column「int_column」はNULLが入らない。
- 2.column「int_column」は0~7のvalueである。
- 3.column「varchar_column」はNULLが入らない。
- 4.column「varchar_column」は'xxxx'か'yyyy'のvalueが入る。

[ExpectationConfiguration](https://github.com/great-expectations/great_expectations/blob/9fed397a318f5fa773f274f11db2116b65b1dcc9/great_expectations/core/expectation_configuration.py#L96)に渡す、expectation_typeとkwargsについてはProfilingした時と同じ[こちらのページ](https://greatexpectations.io/expectations/?filterType=Backend%20support&gotoPage=1&showFilters=false&viewType=Summary)を参考に行う。
```python
# 新規suiteとvalidatorの作成
suite_name = "ex_suite"
context.create_expectation_suite(
    suite_name, overwrite_existing=True
)
validator = context.get_validator(
    batch_request=batch_request,
    expectation_suite_name=suite_name
)

# Expectationの作成
from great_expectations.core.expectation_configuration import ExpectationConfiguration
e1 = ExpectationConfiguration( # column「int_column」はNULLが入らない。
   expectation_type="expect_column_values_to_not_be_null",
   kwargs={
      "column": "int_column"}
)
e2 = ExpectationConfiguration(　# column「int_column」は0~7のvalueである。
   expectation_type="expect_column_values_to_be_between",
   kwargs={
      "column": "int_column",
      "min_value": 0,
      "max_value": 7}
)
e3 = ExpectationConfiguration( # column「varchar_column」はNULLが入らない。
   expectation_type="expect_column_values_to_not_be_null",
   kwargs={
      "column": "varchar_column"}
)
e4 = ExpectationConfiguration( # column「varchar_column」は'xxxx'か'yyyy'のvalueが入る。
   expectation_type="expect_column_values_to_be_in_set",
   kwargs={
      "column": "varchar_column",
      "value_set": ['xxxx', 'yyyy']}
)
```
#### ExpectationSuiteへExpectationの追加
expectationをまとめて渡すexpectation_configurationsは、 version0.15.*から使えるみたい。
```python
suite.add_expectation_configurations(
    expectation_configurations=[e1,e2,e3,e4])
context.save_expectation_suite(suite, "test_suite2")
```


#### Validatorを作成しValidateしてみる。
batch_requestとexpectation_suiteの準備ができたので、Validatorを作成してValidateしてみる。
```python
res = validator.validate()
```

#### 結果を表示してみる。
```python
for r in res['results']:
    print('---')
    print(f'expectation_type: {r.expectation_config.expectation_type}')
    print(f'kwargs: {r.expectation_config.kwargs}')
    print(f'success: {r.success}')
    print(f'result: {r.result}')
```
上記を実行すると以下のような実行結果が見える。
```
---
expectation_type: expect_column_values_to_not_be_null
kwargs: {'column': 'int_column', 'batch_id': 'b431836ba46465a1106559d8e5ff2656'}
success: True
result: {'element_count': 8, 'unexpected_count': 0, 'unexpected_percent': 0.0, 'partial_unexpected_list': []}
---
expectation_type: expect_column_values_to_be_between
kwargs: {'column': 'int_column', 'max_value': 7, 'min_value': 0, 'batch_id': 'b431836ba46465a1106559d8e5ff2656'}
success: True
result: {'element_count': 8, 'unexpected_count': 0, 'unexpected_percent': 0.0, 'partial_unexpected_list': [], 'missing_count': 0, 'missing_percent': 0.0, 'unexpected_percent_total': 0.0, 'unexpected_percent_nonmissing': 0.0}
---
expectation_type: expect_column_values_to_not_be_null
kwargs: {'column': 'varchar_column', 'batch_id': 'b431836ba46465a1106559d8e5ff2656'}
success: True
result: {'element_count': 8, 'unexpected_count': 0, 'unexpected_percent': 0.0, 'partial_unexpected_list': []}
---
expectation_type: expect_column_values_to_be_in_set
kwargs: {'column': 'varchar_column', 'value_set': ['xxxx', 'yyyy'], 'batch_id': 'b431836ba46465a1106559d8e5ff2656'}
success: True
result: {'element_count': 8, 'unexpected_count': 0, 'unexpected_percent': 0.0, 'partial_unexpected_list': [], 'missing_count': 0, 'missing_percent': 0.0, 'unexpected_percent_total': 0.0, 'unexpected_percent_nonmissing': 0.0}
```
試しに以下の行を追加して、『2. column「int_column」は0~7のvalueである。』に反するようにしてみる。
- DDL
```sql
INSERT INTO sample_poyo VALUES (9, 'xxxx', 'zzzz');
```
- データ内容
```
 int_column | varchar_column | varchar_column2 
------------+----------------+-----------------
         0 | xxxx           | zzzz
         1 | xxxx           | zzzz
         2 | xxxx           | zzzz
         3 | xxxx           | zzzz
         4 | yyyy           | zzzz
         5 | yyyy           | zzzz
         6 | yyyy           | zzzz
         7 | yyyy           | zzzz
         9 | yyyy           | zzzz
```
そうすると次のように結果が変わる。検証対象９行(element_count)のうち1行(unexpected_count)が期待通りではない、という意味だ。
```
expectation_type: expect_column_values_to_be_between
kwargs: {'column': 'int_column', 'max_value': 7, 'min_value': 0, 'batch_id': 'b431836ba46465a1106559d8e5ff2656'}
success: False
result: {'element_count': 9, 'unexpected_count': 1, 'unexpected_percent': 11.11111111111111, 'partial_unexpected_list': [9], 'missing_count': 0, 'missing_percent': 0.0, 'unexpected_percent_total': 11.11111111111111, 'unexpected_percent_nonmissing': 11.11111111111111}
```

## つまづいたポイント
今回指定したように、SqlAlchemyを活用したDataSourceを作成したい場合(execution_engine.class_nameに`SqlAlchemyExecutionEngine`を指定したDataSourceの場合)、create_engineする時の引数は「connection_string」しか現在渡せないようです。
[https://github.com/great-expectations/great_expectations/issues/6226](https://github.com/great-expectations/great_expectations/issues/6226)

## 終わりに
GreatExpectationsとは何か、およびPythonを使った使い方について説明していきました。
今回は使いませんでしたがRuntimeBatchRequestを使えば検証したいデータを都度柔軟に指定できるところ、Queryを書かずにExpectationを定義できるところ、及びコードベースでそれらが管理できるところが個人的に良いかなと思っています。

