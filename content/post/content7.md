---
title: "AirflowのTaskFlowAPIについて"
publishedAt: 2023-01-01
description: "今更ですがしっかり調べてみました"
slug: "content7"
isPublish: true
tags: ["Airflow","Python"]
---


## TaskFlowAPIとは
Airflow上でのWorkflowの書き方の一つ。従来Workflowの書き方は「DAGclassを定義し、Operator同士を繋げる書き方(Operatorを使用した書き方)」しかなかったが、Airflow2.0から新しい書き方が導入された。

## 基本的な書き方

### DAGの書き方
`@dag`でWorkflowを定義する関数をdecorateする。グローバル変数にDAGを登録する必要がなくなった。
```
@dag(
    schedule=None,
    start_date=pendulum.datetime(2021, 1, 1, tz="UTC"),
    catchup=False,
    tags=["example"],
)
def tutorial_taskflow_api():
   """ DAG Docs Example
   """
```
- `tutorial_taskflow_api`がdag_id
- 関数配下に書いてある__doc__がDAGのDoc

### Taskの書き方(PythonOperatorの例)
TaskFlowDecoratorでTaskの関数をdecorateする。
```
@task()
def extract():
    """ extract doc_md Example
    """
    data_string = '{"1001": 301.27, "1002": 433.21, "1003": 502.22}'

    order_data_dict = json.loads(data_string)
    return order_data_dict
```
- `extract`がtask_id
- 関数配下に書いてある__doc__がTaskのDoc


### 対応しているOperator
上記PythonOperator以外にも以下のOperatorがTaskFlowDecoratorに対応している。主に独自の実行環境でPythonを実行する必要があるようなOperatorが現在は対応しているイメージ。
|Operator|TaskFlowDecorator|
|---|---|
|PythonOperator|@task(@task.python)|
|PythonVirtualenvOperator|@task.virtualenv|
|BranchPythonOperator|@task.branch|
|DockerOperator|@task.docker|
|KubernetesOperator|@task.kubernetes|
|ExternalPythonOperato|@task.external_python|
|SensorOperator|@task.sensor|


### Flowの書き方
Operatorを使用した書き方では、`>>`を使用してFlowを記述していたが、Pythonライクに関数を呼び出す形でFlow(依存関係)を書くことができるようになった。

例えばデータを抽出(extract)して、抽出したデータを変換(transform)して、変換したデータから`total_order_value`を読み込む(load)ような例は以下のように明示的に定義できる。
```
order_data = extract() # extract
order_summary = transform(order_data) # transform
load(order_summary["total_order_value"]) # load
```


## 何が異なるのか(TaskFlowAPIのメリット)
Operatorを使用した書き方と比べ、個人的に以下の４点がメリットとしてあるかなと思っております。
- 「Xcomが抽象化されTask間のデータの受け渡しが容易になった」
- 「Task全体が見やすくなった」
- 「Taskを再利用できる」
- 「multiple_outputsが使える」
### Xcomが抽象化されTask間のデータの受け渡しが容易になった
以前の書き方ではTask間のデータの受け渡しは[task_instances](https://airflow.apache.org/docs/apache-airflow/stable/_api/airflow/models/taskinstance/index.html#module-airflow.models.taskinstance)のxcom_pullやxcom_pushなどを記述して行っていたが、Xcomが抽象化されたためXcomの記述をしなくても良くなり、データの受け渡しが容易になった。
- NOT TaskFlowAPI利用
```
def extract(**kwargs):
    ti = kwargs["ti"]
    data_string = '{"1001": 301.27, "1002": 433.21, "1003": 502.22}'
    ti.xcom_push("order_data", data_string)
def transform(**kwargs):
    ti = kwargs["ti"]
    extract_data_string = ti.xcom_pull(task_ids="extract", key="order_data")
    print(extract_data_string)
extract_task = PythonOperator(
    task_id="extract",
    python_callable=extract,
)
transform_task = PythonOperator(
    task_id="transform",
    python_callable=transform,
)
extract_task >> transform_task
```
- TaskFlowAPI利用
```
@task()
def extract():
    data_string = '{"1001": 301.27, "1002": 433.21, "1003": 502.22}'
    order_data_dict = json.loads(data_string)
    return order_data_dict
@task()
def transform(order_data_dict: dict):
    print(order_data_dict)
order_data = extract()
transform(order_data)
```
### Task全体が見やすくなった
DAGやTaskのDocがdecorateした関数の__doc__を自動的に参照してくれるようになったのでTask全体が見やすくなった。Task間の依存関係もわかりやすくなった。
- NOT TaskFlowAPI利用
```
def extract(**kwargs):
　　　pass
extract_task = PythonOperator(
  task_id="extract",
  python_callable=extract,
)
extract_task.doc_md = dedent('extract_taskのdoc')
```
- TaskFlowAPI利用
```
@task()
def extract():
  """ extract_taskのdoc
  """
```

### Taskを再利用できる
[override](https://github.com/apache/airflow/blob/81cd6c74788bc3182397dd28a4e7db3f21ff2d69/airflow/decorators/base.py#L447)という関数を使用して再利用できる。また再利用時にtask_id,queue,poolを指定できる。
```
from airflow.decorators import task, dag
from datetime import datetime


@task
def add_task(x, y):
    print(f"Task args: x={x}, y={y}")
    return x + y

@dag(start_date=datetime(2022,1,1))
def mydag():
    """ start -> [add_start_0, add_start_1, add_start_2,]
    """
    start = add_task.override(task_id='start')(1,2)
    for i in range(3):
        start >> add_task.override(task_id=f'add_start_{i}')(start, i)


@dag(start_date=datetime(2022,1,1))
def mydag2():
    """ add_task >> ['new_add_task_0', 'new_add_task_1', 'new_add_task_2']
    """
    start = add_task(1,2)
    for i in range(3):
        start >> add_task.override(task_id=f'new_add_task_{i}')(start, i)

first_dag = mydag()
second_dag = mydag2()
```

### multiple_outputsが使える
1TaskでXcom複数のkey/valueをPushしたい時にこれまでkeyの分だけxcom_pushをしていたが、multiple_outputsを使えば1行で複数key/valueをPushすることができる。Taskの関数の戻り値にDictを指定しているとTaskFlowDecoratorに渡す`multiple_outputs`が自動的にTrueになり、Dictを展開した上でXcomにkey/valueをPushしてくれる。
- NOT TaskFlowAPI利用
```
def identity_dict(**kwargs):
    ti = kwargs['ti']
    ti.xcom_push('x', kwargs['x'])
    ti.xcom_push('y', kwargs['y'])
```
- TaskFlowAPI利用
```
@task(multiple_outputs=True)
def identity_dict(x: int, y: int):
    return {"x": x, "y": y}
# 以下の書き方でも同じ挙動
@task
def identity_dict(x: int, y: int) -> dict[str, int]:
    return {"x": x, "y": y}
```
上記の場合は次のkey/valueがXcomにPushされる。
- return_value: {"x": x, "y": y}
- x: 1
- y: 3


## Operatorとの互換性について
### TaskFlowAPIとOperatorの併用は可能
```
@task()
def extract_from_file():
   order_data_file = "/tmp/order_data.csv"
   order_data_df = pd.read_csv(order_data_file)
file_task = FileSensor(task_id='check_file', filepath='/tmp/order_data.csv')
order_data = extract_from_file()
file_task >> order_data
```
- Operator->TaskFlowAPIへのXcomの受け渡しもOperatorの`output`プロパティを利用すればできる
```
@task
def task_output(msg):
    print(f"Output: {msg}")

@dag(start_date=datetime(2022,1,1))
def mydag2():
    bash_task = BashOperator(
        task_id="bash_task",
        bash_command='echo "Here is the message"')
    bash_task_output = bash_task.output
    task_output(bash_task_output)
```
### TaskFlowAPIでもContextを取得することができる
Taskにレンダリングされる[Context](https://github.com/apache/airflow/blob/8a15557f6fe73feab0e49f97b295160820ad7cfd/airflow/utils/context.py#L158)は明示的に引数を渡すか(①)、kwargsでとるか(②)、get_current_contextを使用(③)して取得することができる。
```
#　明示的に引数を渡す
@task
def my_python_callable(ti=None, next_ds=None):
    pass
# kwargsでとる
@task
def my_python_callable(**kwargs):
    ti = kwargs["ti"]
    next_ds = kwargs["next_ds"]

from airflow.operators.python import get_current_context
def some_function_in_your_library():
    context = get_current_context()
    ti = context["ti"]
```

## @task.kubernetesを試してみる
Zennにおいて以前書いた記事[https://zenn.dev/467/articles/ca76be579ccf97](https://zenn.dev/467/articles/ca76be579ccf97)に書いてあるOperator(KubernetesPodOperator)を使ったやり方をTaskFlowDecorator(@task.kubernetes)に対応しつつ試してみます。@task.kubernetesへ渡せるParameterは[`decorators/__init__.pyl`](https://github.com/apache/airflow/blob/5246c009c557b4f6bdf1cd62bf9b89a2da63f630/airflow/decorators/__init__.pyi#L300)から確認できます。

### 簡易的なPodを作ってみる
- dag
```
from airflow.decorators import task, dag
from airflow.utils.dates import days_ago

@task.kubernetes(
    namespace="default", name="hello-pod-work",
    image="python:3.8-slim-buster", labels={"foo": "bar"}, do_xcom_push=False, in_cluster=False)
def dry_run_demo():
    print('dry_run_test')

@dag(
    default_args={'owner': '467', 'depends_on_past': False}, start_date=days_ago(2),
    description='KubernetesPodOperatorを試す', tags=["work"])
def test_kubernetes_pod_operator_work():
    dry_run_demo()
```
logとmanifestを確認するとわかるが、環境変数` __PYTHON_SCRIPT`へdry_run_demo関数のコードをexportし、`/tmp/script.py`という一時ファイルへ書き込み、実行ということをしている。
- manifest
```
    env:
    - name: __PYTHON_SCRIPT
      value: "import pickle\nimport sys\n\n\nif sys.version_info >= (3,6):\n    try:\n
        \       from airflow.plugins_manager import integrate_macros_plugins\n        integrate_macros_plugins()\n
        \   except ImportError:\n        \n        pass\n\n\narg_dict = {\"args\":
        [], \"kwargs\": {}}\n\n\n# Script\ndef dry_run_demo():\n    print('dry_run_test')\n\nres = dry_run_demo(*arg_dict[\"args\"],
        **arg_dict[\"kwargs\"])"
```
- log
```
[2023-01-03, 13:41:48 JST] {pod_manager.py:237} INFO - + python -c 'import base64, os;x = os.environ["__PYTHON_SCRIPT"];f = open("/tmp/script.py", "w"); f.write(x); f.close()'
[2023-01-03, 13:41:48 JST] {pod_manager.py:237} INFO - + python /tmp/script.py
[2023-01-03, 13:41:48 JST] {pod_manager.py:237} INFO - dry_run_test
```
### Secretをmountしてみる
- dag
```
from works import config_storage_apis

@task.kubernetes(
    namespace="default", name="hello-pod-work", image="python:3.8-slim-buster",
    labels={"foo": "bar"}, do_xcom_push=False, in_cluster=False,
    secrets=[
        config_storage_apis.secret_file(),
        config_storage_apis.secret_env(),
        config_storage_apis.secret_all_keys()],)
def dry_run_demo():
    import os
    print(f"secret_files: {os.listdir(path='/etc/sql_conn')}")
    print(f"secret_env: {os.environ.get('SQL_CONN')}")
    print(f"secret_all_keys: {os.environ.get('sql_alchemy_conn2')}")

@dag(
    default_args={'owner': '467', 'depends_on_past': False}, start_date=days_ago(2),
    description='KubernetesPodOperatorを試す', tags=["work"])
def test_kubernetes_pod_operator_work():
    dry_run_demo()
```
- works.config_storage_apis
```
from airflow.kubernetes.secret import Secret

def secret_file():
    return Secret(
        deploy_type='volume', deploy_target='/etc/sql_conn',
        secret='airflow-secrets'
        )

def secret_env():
    return Secret(
        deploy_type='env', deploy_target='SQL_CONN',
        secret='airflow-secrets', key='sql_alchemy_conn'
	)

def secret_all_keys():
    return Secret(
        deploy_type='env', deploy_target=None,
        secret='airflow-secrets-2'
	)
```
- log
```
[2023-01-03, 14:20:37 JST] {pod_manager.py:237} INFO - secret_files: ['sql_alchemy_conn', '..data', '..2023_01_03_05_20_34.2294851822']
[2023-01-03, 14:20:37 JST] {pod_manager.py:237} INFO - secret_env: hoge
[2023-01-03, 14:20:37 JST] {pod_manager.py:237} INFO - secret_all_keys: hoge2
```

### PersistentVolumeClaim(PVC)をmountしてみる
- dag
```
from works import config_storage_apis

@task.kubernetes(
    namespace="default", name="hello-pod-work", image="python:3.8-slim-buster",
    labels={"foo": "bar"}, do_xcom_push=False, in_cluster=False,
    volumes=[config_storage_apis.volume()],
    volume_mounts=[config_storage_apis.volume_mount()],)
def dry_run_demo():
    import os
    print(f"volume_mount: {os.listdir(path='/root/mount_file')}")

@dag(
    default_args={'owner': '467', 'depends_on_past': False}, start_date=days_ago(2),
    description='KubernetesPodOperatorを試す', tags=["work"])
def test_kubernetes_pod_operator_work():
    dry_run_demo()
```
- works.config_storage_apis
```
from kubernetes.client import models as k8s

def volume_mount():
    return k8s.V1VolumeMount(
        mount_path='/root/mount_file', name='test-volume',
        read_only=True, sub_path=None, sub_path_expr=None,
        mount_propagation=None
        )

def volume():
    return k8s.V1Volume(
        name='test-volume',
        persistent_volume_claim=k8s.V1PersistentVolumeClaimVolumeSource(
            claim_name='my-pvc'),
        )
```
- log
```
[2023-01-03, 16:55:47 JST] {pod_manager.py:237} INFO - volume_mount: ['pv-delay-bind']
```

### Configmapをmountしてみる
- dag
```
from works import config_storage_apis

@task.kubernetes(
    namespace="default", name="hello-pod-work", image="python:3.8-slim-buster",
    labels={"foo": "bar"}, do_xcom_push=False, in_cluster=False,
    env_from=config_storage_apis.configmaps(),)
def dry_run_demo():
    import os
    print(f"key1: {os.environ['key1']}")
    print(f"key2: {os.environ['key2']}")

@dag(
    default_args={'owner': '467', 'depends_on_past': False}, start_date=days_ago(2),
    description='KubernetesPodOperatorを試す', tags=["work"])
def test_kubernetes_pod_operator_work():
    dry_run_demo()
```
- works.config_storage_apis
```

def configmaps():
    return [
        k8s.V1EnvFromSource(
		config_map_ref=k8s.V1ConfigMapEnvSource(
			name='test-configmap-1')
		),
        k8s.V1EnvFromSource(
		config_map_ref=k8s.V1ConfigMapEnvSource(
			name='test-configmap-2')
		),
        ]
```
- log
```
[2023-01-03, 17:08:32 JST] {pod_manager.py:237} INFO - key1: value1
[2023-01-03, 17:08:32 JST] {pod_manager.py:237} INFO - key2: value2
```

### Pod内に複数コンテナを作る
Operatorを使用した書き方では複数コンテナを作る方法として`full_pod_spec`パラメータを使用する方法と`pod_template_file`パラメータを使用する方法がありましたが、TaskFlowDecoratorでは`full_pod_spec`パラメータを使用する方法は現在サポートされていないようです。なので`pod_template_file`パラメータを使用する方法で試してみます。
- test.yml
```
apiVersion: v1
kind: Pod
metadata:
  labels:
    run: share-pod
  name: share-pod
spec:
  containers:
  - image: python:3.7-slim-buster
    name: container1
  - image: python:3.8-slim-buster
    name: container2
  restartPolicy: Always
```
- dag
```
pod_template_filepath='test.yml'

@task.kubernetes(
    namespace="default", pod_template_file=pod_template_filepath,
    do_xcom_push=False, in_cluster=False,)
def dry_run_demo():
    print('hello container!')

@dag(
    default_args={'owner': '467', 'depends_on_past': False}, start_date=days_ago(2),
    description='KubernetesPodOperatorを試す', tags=["work"])
def test_kubernetes_pod_operator_work():
    dry_run_demo()
```
name:share-podへ、name:container1,name:container2というコンテナを作成しているが、dry_run_demo関数内のコード(print文)は一つのコンテナ上でしか実行されない。上記の例だとspec.containersの1要素目のコンテナ(container1)が優先されnameが`base`に変換された形で実行される。
- manifestの一部
```
apiVersion: v1
kind: Pod
metadata:
  name: k8s-airflow-pod-3b69a29a430d46-eddb86a134094187bba374454c6d83f1
  namespace: default
spec:
  containers:
  - args:
    - -cx
    - python -c "import base64, os;x = os.environ[\"__PYTHON_SCRIPT\"];f = open(\"/tmp/script.py\",
      \"w\"); f.write(x); f.close()" && python /tmp/script.py
    command:
    - bash
    env:
    - name: __PYTHON_SCRIPT
      value: "import pickle\nimport sys\n\n\nif sys.version_info >= (3,6):\n    try:\n
        \       from airflow.plugins_manager import integrate_macros_plugins\n        integrate_macros_plugins()\n
        \   except ImportError:\n        \n        pass\n\n\narg_dict = {\"args\":
        [], \"kwargs\": {}}\n\n\n# Script\ndef dry_run_demo():\n    print('hello container!')\n\nres = dry_run_demo(*arg_dict[\"args\"],
        **arg_dict[\"kwargs\"])"
    image: python:3.7-slim-buster
    name: base
  - image: python:3.8-slim-buster
    name: container2
```
- log(airlfow)
airflowのlogにはbase(container1)のログしか残っていない
```
[2023-01-03, 18:42:40 JST] {pod_manager.py:237} INFO - hello container!
```
- log(k8s)
k8s上では正しく動作している。
```
$ k logs k8s-airflow-pod-3b69a29a430d46-eddb86a134094187bba374454c6d83f1  -c base
+ python -c 'import base64, os;x = os.environ["__PYTHON_SCRIPT"];f = open("/tmp/script.py", "w"); f.write(x); f.close()'
+ python /tmp/script.py
$ k logs k8s-airflow-pod-3b69a29a430d46-eddb86a134094187bba374454c6d83f1  -c container2
```

### Xcomを扱う
- dag
```
@task.kubernetes(
    namespace="default", name="hello-pod-work", image="python:3.8-slim-buster",
    labels={"foo": "bar"}, do_xcom_push=True, in_cluster=False,)
def dry_run_demo():
    f = open('/airflow/xcom/return.json', 'w')
    f.write('[1,2,3,4]')
    f.close()

@task
def xcom_output(**kwargs):
    ti = kwargs["ti"]
    print(f"output: {ti.xcom_pull(task_ids='dry_run_demo')}")

@dag(
    default_args={'owner': '467', 'depends_on_past': False}, start_date=days_ago(2),
    description='KubernetesPodOperatorを試す', tags=["work"])
def test_kubernetes_pod_operator_work():
    dry_run_demo() >> xcom_output()
```
- log(airflow xcom_output)
```
[2023-01-03, 19:14:54 JST] {logging_mixin.py:137} INFO - output: [1, 2, 3, 4]
```

## 参考
- https://airflow.apache.org/docs/apache-airflow/stable/tutorial/taskflow.html