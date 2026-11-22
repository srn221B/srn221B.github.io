---
title: "AirflowのPluginsについて"
publishedAt: 2023-01-04
description: "カスタマイズ機能についてです"
slug: "content8"
isPublish: true
tags: ["Airflow","Python"]
---


## AirflowPluginsとは
Airflowをカスタマイズするために追加できるPlugin。
### どんな時にAirflowPluginsを使うのか
例えば以下のようなときにAirflowPluginsを使う。
- AirflowのメタデータのDBのコンテンツを表示するViewを作成したいとき。
- 指定された時間内において任意数のDAGが失敗時にAirflowの機能を監視しカスタムアラートを送信するアプリケーションを作成したいとき。
- 外部データツールのファイルやログに動的にリンクする詳細ビューをUIに表示したいとき。

## 新規Plugin作成方法
「plugins_folder」へPythonファイルを用意し、Pythonファイル内で[airflow.plugins_manager.AirflowPlugin](https://github.com/apache/airflow/blob/a9493c13173f6108c02c42d2f4f60b82b5ccc71a/airflow/plugins_manager.py#L134)を拡張するクラスを定義すれば新規Pluginを作成できる。
- Pythonファイルの例
	- 定義したAirflowPluginクラスには以下を実装する。
        - `name:property`にPlugin名
	    - `on_load:function`にPluginのロード時に実行してほしい処理
    	- そのほかPluginに追加したいComponent(Pluginの機能)について列挙
```
from airflow.plugins_manager import AirflowPlugin

class MyAirflowPlugin(AirflowPlugin):
    name = "empty"
    def on_load(cls, *args, **kwargs):
        pass
    # そのほかPluginに追加したいComponentについて列挙していく。
```
上記場合、Plugin名が「empty」であるPluginを作成している。
- 「plugins_folder」の確認の仕方
	- ちなみに「plugins_folder」はどこで確認できるのかというのだが、「コマンドを利用する」か「cfgファイルを見る」の2通りで確認できる。
    - 基本的にデフォルトでは$AIRFLOW_HOME/pluginsになっているはず。
```
# コマンドを利用する
$ airflow info  | grep plugins_folder
plugins_folder         | xxx               
# cfgファイルを見る
$ cat airflow.cfg | grep plugins_folder
plugins_folder = xxxx
```
## Plugin確認方法
現在追加されているPluginは何か、Pluginに定義されてあるComponentは何かを確認する方法だが、「UIから確認する方法」と「CLIを利用して確認する方法」がある。WebServerを再起動しなくてもすぐに反映されるのは後者の方法。
- 「UIから確認する方法」
	- UIのAdmin->Pluginsを使用する。

![](https://storage.googleapis.com/zenn-user-upload/47201792a899-20230105.png)

- 「CLIを利用して確認する方法」
	- airflow pluginsを実行する。
```
$ airflow plugins
name            | source                      | appbuilder_views                                                                            | macros               
================+=============================+=============================================================================================+======================
my_plugins_poyo | $PLUGINS_FOLDER/__init__.py | {'name': 'Test View', 'category': 'Test Plugin', 'view': '__init__.TestAppBuilderBaseView'} | __init__.plugin_macro
```

## どんなComponentをPluginに追加することができるのか
例えば以下のようなComponentが現在ある。種類としてUIへの機能追加からScheduling、DAG作成する上での機能追加などなどがある。詳細については[ドキュメント](https://airflow.apache.org/docs/apache-airflow/stable/plugins.html)を確認すると良い。
|No|Component|About|
|---|---|---|
|1|hooks|独自のHookを作成する|
|2|macros|カスタムmacroを作成する|
|3|flask_blueprints|AirflowのFlaskにflask.BluePrint製のページを作成する|
|4|appbuilder_views|AirflowのFlaskにflask_appbuilder.BaseView製のページを作成する|
|5|appbuilder_menu_items|Airflow UIのメニューにセクションとリンクを作成する|
|6|global_operator_extra_links|**全ての種類の**OperatorにLinkを作成する|
|7|operator_extra_links|**任意の種類の**OperatorにLinkを作成する|
|8|timetables|CRONでは表現できないスケジュールを作成する|

そのほか「executorを使用してCustomExecutorを作成する」「listenerを使用してイベント検知を作成する」などできそう。

今回は上記のComponentのみ順に試してみます。

---
### 1.hooksを使用して独自のHookを作成する。
#### AirflowのHookとは
外部システムとのインタフェースの役割をしてくれるようなもの。例えば以下のように書いて一貫したメソッドを提供することを意図としてある。
```
pg_hook = PostgresHook.get_hook(conn_id)
pg_hook.copy_exper()
```
詳細：[https://airflow.apache.org/docs/apache-airflow/stable/concepts/connections.html](https://airflow.apache.org/docs/apache-airflow/stable/concepts/connections.html)
#### plugins_folder配下のファイル
今回は次のようなファイル構造で新規独自のHookを作成していきます。
```
.
├── __init__.py　# pluginについて定義
├── hooks
│   ├── __init__.py
│   └── airflow_hook_template.py # Hookについて定義
```
- `__init__.py`
```
from __future__ import division, absolute_import, print_function
import hooks
from airflow.plugins_manager import AirflowPlugin


class MyPlugins(AirflowPlugin):
    name = "my_plugins_poyo"
    hooks = [hooks.PluginHook]
```
- `hooks/__init__.py`
```
from hooks.airflow_hook_template import PluginHook

__all__ = ['PluginHook']
```
- `hooks/airflow_hook_template.py`
    - airflow.hooks.base.BaseHookを拡張して書く。
```
from airflow.hooks.base import BaseHook


class PluginHook(BaseHook):
    pass
```
上記の場合、特に何もしないHook。
#### 作成を反映させる。
webserverの再起動が必要。
#### dagのファイルからimportしてみる
plugins_folderは環境変数`PYTHONPATH`として自動的にbuildされている(https://airflow.apache.org/docs/apache-airflow/stable/modules_management.html#built-in-pythonpath-entries-in-airflow)ので,直接plugins_folder配下のディレクトリを指定する形でimportできる。UIからDAGを確認するなどをしてimportErrorが出ていなければ正常に動作している。
- dagのファイル
```
from hooks.airflow_hook_template import PluginHook
```
---

### 2.macrosを使用してカスタムmacroを作成する。
#### macroとは
JinjaTemplateを使用してTask内で利用できる変数のようなもの。
```
print('実行日は {{execution_date}}')
```

詳細：[https://airflow.apache.org/docs/apache-airflow/stable/templates-ref.html](https://airflow.apache.org/docs/apache-airflow/stable/templates-ref.html)
#### plugins_folder配下のファイル
plugins_folder配下に`__init__.py`を作成してそこに記載する形で書いていく。
- `__init__.py`
    - 関数に作成するmacroについて定義していく。
        - 関数名がmacro名
        - 関数の戻り値がmacroのvalueとなる
```
from airflow.plugins_manager import AirflowPlugin

def plugin_macro(x: int): 
    return x + 1 

class MyPlugins(AirflowPlugin):
    name = "my_plugins_poyo"
    macros = [plugin_macro]
```
上記の場合、`plugin_macro`というmacroを定義しており、そのmacroは引数`x:int`を`x + 1`した形で返すということをしている。

#### 作成を反映させる。
webserverの再起動が必要。
#### dagのファイルから利用してみる
- dagのファイル
    - 作成したmacroは`macros.<plugin_name>.<macro_name>`で取得できる。
```
from airflow.decorators import task, dag
from airflow.utils.dates import days_ago

@task
def test_output(**kwargs):
    macros = kwargs['macros']
    input = 1
    print(f'input: {input}')
    print(f'output: {macros.my_plugins_poyo.plugin_macro(input)}')

@dag(start_date=days_ago(2))
def test_plugins():
    test_output()
    
test_plugins()
```
- log
    - 上記DAGを実行してみたLogだが、引数`1`に対しoutputが`2`となっているのでmacroが効いていることを確認できる。
```
[2023-01-04, 18:31:16 JST] {logging_mixin.py:137} INFO - input: 1
[2023-01-04, 18:31:16 JST] {logging_mixin.py:137} INFO - output: 2
```
---

### 3.flask_blueprintsを使用してAirflowのFlaskにflask.BluePrint製のページを作成する。
#### flask.BluePrintとは
Flask内でアプリケーションの機能を複数のファイルに分割する方法。ソースコードが長すぎる場合に、機能ごとにファイルを分割し管理しやすくするために利用する。
#### plugins_folder配下のファイル
plugins_folder配下に`__init__.py`を作成してそこに記載する形で書いていく。
- `__init__.py`
    - airflow.plugins_manager.AirflowPluginを拡張してかく。
    - `@bp.route`に渡したパスが追加したいページの`http://<airflow webserver IP>:<port>/`直下のSubpathとなる。
```
from flask import Blueprint
from airflow.plugins_manager import AirflowPlugin


bp = Blueprint('app2', __name__)

@bp.route('/bp_test')
def bp_test():
    """ 今日の天気を取得して表示する
    """
    import requests
    import json
    res = requests.get('https://weather.tsukumijima.net/api/forecast/city/220030')
    today_weather = json.loads(res.text)['forecasts'][0]
    return today_weather['date'] + ' ' + today_weather['detail']['weather']

class MyPlugins(AirflowPlugin):
    name = "my_plugins_poyo"
    flask_blueprints = [bp]
```
上記の場合、Subpathが`/bp_test`であるページを追加しており、ページを開くとweatherAPIを使用して任意の市の今日の日付と天気を取得し表示するようなことをしている。
#### 作成を反映させる。
webserverの再起動が必要。
#### 作成したページにアクセスしてみる。
- `http://<airflow webserver IP>:<port>/bp_test`へアクセスすると以下のようなページが表示される。

![](https://storage.googleapis.com/zenn-user-upload/abade4eefcce-20230104.png)

---

### 4.appbuilder_viewsを使用してAirflowのFlaskにflask_appbuilder.BaseView製のページを作成する。
#### flask_appbuilder.BaseViewとは
FlaskのViewのうち、よりきめ細かくセキュリティ(メソッド,アクセス)を設定できるView。
#### plugins_folder配下のファイル
plugins_folder配下に`__init__.py`を作成してそこに記載する形で書いていく。
- `__init__.py`
    - AirflowのメニューバーのセクションにViewのLinkを設定する形で書いていく。
        - `category:str`に渡した値がメニューバー名
        - `name:str`に渡した値がセクション名
```
from airflow.plugins_manager import AirflowPlugin
from flask_appbuilder import expose, BaseView as AppBuilderBaseView


class TestAppBuilderBaseView(AppBuilderBaseView):
    default_view = "test"
    @expose('/')
    def test(self):
        """ hello test_pageを表示する
        """
        return 'hello test_page'

v_appbuilder_view = TestAppBuilderBaseView()
v_appbuilder_package = {
    "name": "Test View",
    "category": "Test Plugin",
    "view": v_appbuilder_view,
}

class MyPlugins(AirflowPlugin):
    name = "my_plugins_poyo"
    appbuilder_views = [v_appbuilder_package]
```
上記の場合、メニューバー名`Test Plugin`セクション名`Test View`にViewへのLinkを設定しており、Linkをアクセスすると'hello test_page'と表示されることを期待している。
#### 作成を反映させる。
webserverの再起動が必要。
#### 作成したページにアクセスしてみる。
- 期待通りのメニューバーとセクションができている。

![](https://storage.googleapis.com/zenn-user-upload/7e0c9ebbbf00-20230105.png)

- Viewへアクセスしてみるとページが正常に確認できる。

![](https://storage.googleapis.com/zenn-user-upload/c86b0a906497-20230105.png)

---

### 5.appbuilder_menu_itemsをしてAirflow UIのメニューにセクションとリンクを作成する。
#### plugins_folder配下のファイル
plugins_folder配下に`__init__.py`を作成してそこに記載する形で書いていく。
- `__init__.py`
    - 4.appbuilder_viewsの説明と同様
         - `category:str`に渡した値がメニューバー名
        - `name:str`に渡した値がセクション名
```
from airflow.plugins_manager import AirflowPlugin

appbuilder_mitem = {
    "name": "467 notes",
    "href": "https://467tn.com",
    "category": "MyPages",
}

class MyPlugins(AirflowPlugin):
    name = "my_plugins_poyo"
    appbuilder_menu_items = [appbuilder_mitem]
```
上記の場合、メニューバー名`MyPages`セクション名`467 notes`にViewへのLinkを設定しており、Linkをアクセスすると`href`先のページへ遷移されることを期待している。
#### 作成を反映させる。
webserverの再起動が必要。
#### 作成したページにアクセスしてみる。
- 期待通りのメニューバーとセクションができている。また、セクション先をアクセスすると`https://467tn.com`へ遷移される。

![](https://storage.googleapis.com/zenn-user-upload/b45c75284248-20230105.png)

---

### 6.global_operator_extra_linksを使用して**全ての種類の**OperatorにLinkを作成する
#### AirflowのOperatorLinkとは
Taskの詳細画面につけるボタンLink(うまい表現がわかりません)。基底が[BaseOperatorLink](https://airflow.apache.org/docs/apache-airflow/stable/_api/airflow/models/baseoperator/index.html)となっており、ボタンLinkをClickすると当クラスの`get_link:function`が実行されるわけだが、`get_link:function`には引数として当Taskのoperator(BaseOperator)とti_key(airflow.models.taskinstance.TaskInstanceKey)が渡される。なのでこのOperatorLinkを使えば、Operatorが外部インタフェースと通信するようなOperatorだった場合に外部インタフェース先にあるLogなどを`ti_key`をもとに表示するようなことができます。
例えば標準でprovideされているOperatorLinkのうち、BigQueryConsoleLinkは、BigQueryOperatorを使用してBigQueryに投げたLogをGoogleCloudConsoleから当job_idを指定して確認することができるような仕様をしているOperatorLinkです。

詳細：[https://airflow.apache.org/docs/apache-airflow/stable/howto/define_extra_link.html](https://airflow.apache.org/docs/apache-airflow/stable/howto/define_extra_link.html)


#### plugins_folder配下のファイル
plugins_folder配下に`__init__.py`を作成してそこに記載する形で書いていく。
- `__init__.py`
    - [BaseOperatorLink](https://airflow.apache.org/docs/apache-airflow/stable/_api/airflow/models/baseoperator/index.html)を拡張して書く。
        - `name:property`に渡した値がボタンLink名
        - `get_link:function`に定義した処理はボタンLinkをClickした時に動作する挙動。
            - 戻り値としてstrでLinkPathを記載。
```
from airflow.plugins_manager import AirflowPlugin
from airflow.models.baseoperator import BaseOperatorLink

class GlobalLink(BaseOperatorLink):
    name = "Airflow docs"

    def get_link(self, operator, *, ti_key=None):
        return "https://airflow.apache.org/"


class MyPlugins(AirflowPlugin):
    name = "my_plugins_poyo"
    global_operator_extra_links = [
        GlobalLink(),
    ]
```
上記の場合、`Airflow docs`というボタンLinkを作成しており、ボタンLinkをClickすると`https://airflow.apache.org/`へ遷移する。
#### 作成を反映させる。
webserverの再起動が必要。
#### 作成したLinkを確認してみる。
- 任意のTaskの詳細を開くとOperatorの種類に限らず期待通りのボタンLinkが作られていることを確認できる。ボタンLinkをクリックすると`https://airflow.apache.org/`へ遷移する。

![](https://storage.googleapis.com/zenn-user-upload/904f888118ac-20230105.png)

---

### 7.operator_extra_linksを使用して**任意の種類の**OperatorにLinkを作成する。
#### OperatorLinkとは
「6.global_operator_extra_links」での説明と同様なので割愛。
#### plugins_folder配下のファイル
plugins_folder配下に`__init__.py`を作成してそこに記載する形で書いていく。
- `__init__.py`
    - [BaseOperatorLink](https://airflow.apache.org/docs/apache-airflow/stable/_api/airflow/models/baseoperator/index.html)を拡張して書く。
        - `name:property`と`get_link:function`の仕様については「6.global_operator_extra_links」での説明と同様。
        - `operators:List[BaseOperator]`にLinkを反映したいOperatorを列挙する。
```
from airflow.plugins_manager import AirflowPlugin
from airflow.models.baseoperator import BaseOperatorLink
from airflow.providers.cncf.kubernetes.operators.kubernetes_pod import (
    KubernetesPodOperator,
)

class MyLink(BaseOperatorLink):
    name = "Kubernetes docs"
    operators = [KubernetesPodOperator]
    def get_link(self, operator, *, ti_key=None):
        return "https://kubernetes.io/"

class MyPlugins(AirflowPlugin):
    name = "my_plugins_poyo"
    operator_extra_links = [
        MyLink(),
    ]
```
上記の場合、`Kubernetes docs`というボタンLinkを作成しており、ボタンLinkをClickすると`https://kubernetes.io/`へ遷移する。またこのLinkは`KubernetesPodOperator`を使用したTaskにしか反映されないことを意図としている。

#### 作成を反映させる。
webserverの再起動が必要。
#### 作成したLinkを確認してみる。
KubernetesPodOperatorのあるDAGを開いて確認してみる。
- KubernetesPodOperatorのTaskに期待通りのボタンLinkが作られていることを確認できる。ボタンLinkをクリックすると`https://kubernetes.io/`へ遷移する。

![](https://storage.googleapis.com/zenn-user-upload/2888bb451e42-20230105.png)

- PythonOperatorのTaskにはボタンLinkが作られていない。

![](https://storage.googleapis.com/zenn-user-upload/5b4d15b5bba1-20230105.png)

---

### 8.timetablesを使用してCRONでは表現できないスケジュールを作成する
個人的な本題だったので少し他よりも丁寧に書く。
#### Timetableとは
AirflowのDAG内での「Cronで表すことができないようなスケジュール」を決めるためのTimeTable。DAGに渡す`timetable`引数で指定できる。
- 「Cronで表すことができないようなスケジュール」とは
    - 例えば以下のようなスケジュール。
    - 実行間隔が不規則
        - 2022.12.31 10:00/2023.01.03 10:00のみ実行するなど
    - 毎日異なる時間に実行する
        - 水曜日は10:00、木曜日は03:00に実行するなど
    - グレゴリオ暦に従わないスケジュール
- 標準でprovideされているTimeTable
    - EventsTimetable
    - CronTriggerTimetable

詳細：[https://airflow.apache.org/docs/apache-airflow/stable/concepts/timetable.html](https://airflow.apache.org/docs/apache-airflow/stable/concepts/timetable.html)

#### 独自のTimetable作成方法について
Airflow2.2から独自のTimeTableを作成することができるようになった。[airflow.timetables.base.Timetable](https://github.com/apache/airflow/blob/0be5c90639dcaa9c6542434160f1f1caf51202a0/airflow/timetables/base.py#L108)を拡張すれば良いのだが、以下の関数を含める必要がある。

|関数名|about|
|---|---|
|next_dagrun_info|スケジュール実行時のデータ間隔を返す|
|infer_manual_data_interval|手動実行時のデータ間隔で返す|

#### 独自のTimetableに指定する関数を作成する。
今回は以下のような仕様のTimeTableを作ってみます。
- 1日に2回,06:00(①)と16:30(②)にジョブを実行する。実行時に取得するデータの間隔は以下。
	- 06:00(①)実行時・・・前日の16:30~当日の06:00
	- 16:30(②)実行時・・・当日の06:00~当日の16:30
##### next_dagrun_info関数
- データの間隔をDagRunInfo(`run_after:DateTime`, `data_interval:DataInterval`)で返す
- parameterに入ってくる`restriction:TimeRestriction`とはDAGをスケジュールできる期間に関する制限(`earliest:DateTime`,`latest:DateTime`,`catchup:bool`)を持つクラス。
```python
def next_dagrun_info(
    self,
    *,
    last_automated_data_interval: Optional[DataInterval],
    restriction: TimeRestriction,
) -> Optional[DagRunInfo]:
    if last_automated_data_interval is not None:  # a
        last_start = last_automated_data_interval.start
        delta = timedelta(days=1)
        if last_start.hour == 6: # b
            next_start = last_start.set(hour=16, minute=30).replace(tzinfo=UTC)
            next_end = (last_start+delta).replace(tzinfo=UTC)
        else: # c
            next_start = (last_start+delta).set(hour=6, minute=0).replace(tzinfo=UTC)
            next_end = (last_start+delta).replace(tzinfo=UTC)
　　else:  # d
        next_start = restriction.earliest
        if next_start is None:  
            return None
        if not restriction.catchup:
            next_start = max(next_start, DateTime.combine(Date.today(), Time.min).replace(tzinfo=UTC))
        next_start = next_start.set(hour=6, minute=0).replace(tzinfo=UTC)
        next_end = next_start.set(hour=16, minute=30).replace(tzinfo=UTC)
    if restriction.latest is not None and next_start > restriction.latest: # e
        return None 
    return DagRunInfo.interval(start=next_start, end=next_end)
```
長い行になりましたが補足すると以下
- 初回実行ではない場合(a)
    - 前回実行のデータ間隔のStartが06:00の場合(b)
        - 今回の実行は①に該当する
        - next_startは前回の当日の16:30
        - next_endは前回の翌日の06:00
    - 前回実行のデータ間隔のStartが16:30の場合(c)
        - 今回の実行は②に該当する
        - next_startは前回の翌日の06:00
        - next_endは前回の翌日の16:30
- 初回実行の場合(d)
    - DAGをスケジュールできる期間のうちの最早い日を実行日(next_start)として（catchup=Falseの場合、現在の日付を実行日(next_start)とする）
        - next_startは実行日の06:00
        - next_endは実行日の16:30
- またこの時DAGの終了日(restriction.latest)がある場合、その日を越したnext_endを実行しないことを期待している。(e)
        

#### infer_manual_data_interval関数
- スケジュール実行時のデータの間隔をDataInterval(`start: DateTime`, `end:Datetime`)で返す。
```python
def infer_manual_data_interval(self, run_after: DateTime) -> DataInterval:
    delta = timedelta(days=1)
    # a
    if run_after >= run_after.set(hour=6, minute=0) and run_after <= run_after.set(hour=16, minute=30):
        start = (run_after-delta).set(hour=16, minute=30, second=0).replace(tzinfo=UTC) 
        end = run_after.set(hour=6, minute=0, second=0).replace(tzinfo=UTC) 
    # b
    elif run_after >= run_after.set(hour=16, minute=30) and run_after.hour <= 23:
        start = run_after.set(hour=6, minute=0, second=0).replace(tzinfo=UTC)
        end = run_after.set(hour=16, minute=30, second=0).replace(tzinfo=UTC)
    # c
    else:
        start = (run_after-delta).set(hour=6, minute=0).replace(tzinfo=UTC)
        end = (run_after-delta).set(hour=16, minute=30).replace(tzinfo=UTC)
    return DataInterval(start=start, end=end)
```
補足すると
- 現在時刻が06:00~16:30の間(a)
    - ①の実行とされる。
    - startは前日の16:30、endは当日の06:00
- 現在の時刻が16:30~23:00の間(b)
    - ②の実行とされる。
    - startは当日の06:00、endは当日の16:30
- それ以外、深夜の00:00~05:59の間(c)
     - ②の実行とされる。
    - startは前日の06:00、endは前日の16:30

#### plugins_folder配下のファイル
- ファイル構造
```
├── __init__.py
└── timetables
    ├── __init__.py
    └── airflow_timetable_template.py
```
- `__init__.py`
```
from airflow.plugins_manager import AirflowPlugin
import timetables

class MyPlugins(AirflowPlugin):
    name = "my_plugins_poyo"
    timetables = [timetables.MyTimeTable]
```
- `timetables/__init__.py`
```
from timetables.airflow_timetable_template import MyTimeTable

__all__ = ['MyTimeTable']
```
- `timetables/airflow_timetable_template.py`
上記で書いた関数をそのまま移植していきます。TimeTable名は`MyTimeTable`
```
from datetime import timedelta
from typing import Optional
from pendulum import Date, DateTime, Time, timezone

from airflow.plugins_manager import AirflowPlugin
from airflow.timetables.base import DagRunInfo, DataInterval, TimeRestriction, Timetable

UTC = timezone("UTC")

class MyTimeTable(Timetable):

    def infer_manual_data_interval(self, run_after: DateTime) -> DataInterval:
        ・・・

    def next_dagrun_info(
        self,
        *,
        last_automated_data_interval: Optional[DataInterval],
        restriction: TimeRestriction,
    ) -> Optional[DagRunInfo]:
        ・・・
```
#### 作成を反映させる。
webserverとschedulerの再起動が必要。(TimeTableはschedulerも読み込む必要があるためschedulerも再起動します。再起動しないと`TimetableNotRegistered`Errorが出ます。)
#### 作成したTimeTableを試してみる。
- dagのファイル
適当なDAGの`timetable`引数に作成した`MyTimeTable:TimeTable`を渡す。importErrorが出てなければ正常に動作しているはず、
```
from timetables.airflow_timetable_template import MyTimeTable


@dag(start_date=days_ago(2), timetable=MyTimeTable())
...
```
- UIから作成したDAGの確認
	- Scheduleが`MyTimeTable`になっている
	- 現在時刻が`01/05.14:08`の場合
		- 次の実行は`01/05.16:30`
		- 次の次の実行は`01/06.06:00`。その時のデータ間隔は`01/05.16:30~01/06.06:00`
![](https://storage.googleapis.com/zenn-user-upload/d0837ec8e048-20230105.png)

---

## 終わり
以上AirflowのPluginについて見ていきました。TimeTableについては、AirflowのScheduleについて再度理解する機会になってよかったです。また時間があるときにlistenerやexecutorのカスタムも試してみたいです。



## 参考
 - https://docs.astronomer.io/learn/scheduling-in-airflow#timetables
 - https://docs.astronomer.io/learn/using-airflow-plugins
 - https://airflow.apache.org/docs/apache-airflow/stable/plugins.html