---
title: "Rayメモ"
publishedAt: 2023-10-09
description: "分散処理"
slug: "content13"
isPublish: true
tags: ["Ray","Python"]
---


# Rayとは

[Ray](https://www.ray.io/)とは機械学習を使ったpythonアプリケーションをスケーリングするためOSSフレームワーク。

以下の機能を提供する

- データ前処理、分散トレーニング、調整、強化学習、モデル提供など一般的な機械学習タスクに必要なこと
- Pythonアプリケーションの分散処理
- Kubernetesや各種クラウド製品を使ったスケーリング

## Rayのフレームワーク

以下の３つのレイヤで構成されている

- RayAIRuntime:機械学習アプリケーションにて使うツールキット。
次の５つの分野がサポート揃っている。
  - Data:データの読み込み・変換・チューニング・予測
  - Train:学習
  - Tune:チューニング
  - Serve:モデルの提供
  - RLib:他のtensorflow,torchなどのRayAIRuntime
- RayCore:分散処理できるようにする部分
- RayCluster:スケールする部分

# 試してみる

「RayCluster」と「RayCore」を試してみる

## 「RayCluster」

前述通りPythonアプリケーションをスケールする部分。
`ray.init`を呼び出せばRayClusterを組まなくてもRayは動作するが、スケールしたい場合はClusterを組む必要がある。

### どこで組めるのか
RayClusterはAWS,GCP,Azureなどクラウド製品からKubernetes上でも組めるようにサポートされている。

### RayClusterの構成
RayClusterは一つのHeadnodeといくつかのWorkernodeにて構成されている。

![](https://storage.googleapis.com/zenn-user-upload/4d23f904893d-20230923.png)
> 公式画像引用：https://docs.ray.io/en/latest/cluster/key-concepts.html

Headnodeは受け取ったワークロードリソース要求がクラスタの現在の容量を超えてるとWorkernodeを自動で増やし調整する。逆にWorkernodeがidle状態になるとWorkernodeを自動で削除する。

### Kubernetesを利用したRayClusterの構成
Kubernetesを利用してRayClusterを構築しジョブを実行するには以下の３つの提供されているCRDを構成する必要がある。

- RayCluster: Cluster本体
- RayJob: Clusterジョブ
- RayService: Clusterサービス

kind:RayJobを使わずRayServiceの指定のPortに`ray job submit`をして、ジョブを実行することもできる。前者の場合kind:RayJobをDeployすると都度RayClusterが作られるが、後者はユーザ側でRayClusterを作り直さない限り一つのRayClusterを使い続けることになる。なので使い分けとしては常にRayJobと1:1でRayClusterをDeployしたい場合はkind:RayJobを使い、そうではない場合はRayServiceの指定のPortに都度`ray job submit`を使うという形で良さそう。

### Kubernetesを利用してRayClusterを構築してみる

各種Operatorのインストールをする。

```
$ kubectl apply -k "github.com/ray-project/kuberay/ray-operator/config/default"
```

namespace`ray-system`上で正常に動いていることを確認できる。

```shell
$ kubectl get all -n ray-system
NAME                                    READY   STATUS    RESTARTS   AGE
pod/kuberay-operator-79dd8d67db-skm94   1/1     Running   0          13s

NAME                       TYPE        CLUSTER-IP     EXTERNAL-IP   PORT(S)    AGE
service/kuberay-operator   ClusterIP   10.43.81.120   <none>        8080/TCP   13s

NAME                               READY   UP-TO-DATE   AVAILABLE   AGE
deployment.apps/kuberay-operator   1/1     1            0           13s

NAME                                          DESIRED   CURRENT   READY   AGE
replicaset.apps/kuberay-operator-79dd8d67db   1         1         0       13s
```

そして手順通りrayjobをapplyしたが

```
$ curl -LO https://raw.githubusercontent.com/ray-project/kuberay/master/ray-operator/config/samples/ray_v1alpha1_rayjob.yaml

$ kubectl apply -f ray_v1alpha1_rayjob.yaml
```

RayClusterが動かない。。。HeadnodeでClusterをstartするところでkillされており何度が試したが動かず、切り分けがつかないので一旦放置とする(TODO)

```
$ kubectl logs raycluster-heterogeneous-head-rn2nl 
>
<jemalloc>: MADV_DONTNEED does not work (memset will be used instead)
<jemalloc>: (This is the expected behaviour if you are running under QEMU)
/bin/bash: line 1:    11 Killed                  ray start --head --dashboard-host=0.0.0.0 --metrics-export-port=8080 --block --num-cpus=1
```

### ローカルでRayを動かしてみる

やむを得ずローカルで動かす。

`pip install -U ray`をし以下を実行すると動く。

```
ray start --head
```

`http://127.0.0.1:8265/`から[DashBoard](https://docs.ray.io/en/latest/ray-observability/getting-started.html)を確認できる。

## 「RayCore」

前述通り分散処理の中核の部分。

以下を実行するとRay/RayClusterと接続できる。

```python
import ray
ray.init()
```

### Concept

RayCoreは以下の概念を導入することで柔軟性を保っている

- Tasks: TaskWorkerへ非同期に頼む単位(関数)
- Actors: TaskWorkerへ非同期に頼む単位(クラス)
- Objects: Task/Actor間の値の受け渡しなどに使うRemoteObject。ObjectRayClusterのWorkerNodeごとにObjectStoreがあり他のWorkerNodeと共有し使われる。
- PlacementGroup: Tasks/Actorsにおいて必要なCPU/Memoryなどのリソースを予約する仕組み。またリソースを近くに詰め込んだり(PACK)・分散させたり(SPREAD)することができ、リソースの局所性を実現している。
- Environment Dependencies

公式の次の図が参考になる。

![](https://storage.googleapis.com/zenn-user-upload/ef72dccaf4f9-20231009.png)

Tasks,Actors,Objectsを試してみる。

### Tasks

```python
import time
import ray

@ray.remote
def example_function():
    print("example_function was called.")
    return 1

result = []
for _ in range(4):
    result.append(ray.get(example_function.remote()))
    time.sleep(10)
print("result: ", result)
```

実行結果

```
$ python3 test.py 
・・・
(example_function pid=20478) example_function was called.
(example_function pid=20478) example_function was called.
(example_function pid=20478) example_function was called.
(example_function pid=20478) example_function was called.
result:  [1, 1, 1, 1]
```

`ray list tasks`にて実行されたtasksが確認できる

```
$ ray list tasks
・・・
Table:
------------------------------
    TASK_ID                                             ATTEMPT_NUMBER  NAME              STATE       JOB_ID  ACTOR_ID    TYPE         FUNC_OR_CLASS_NAME    PARENT_TASK_ID                                    NODE_ID                                                   WORKER_ID                                                   WORKER_PID  ERROR_TYPE
 0  16310a0f0a45af5cffffffffffffffffffffffff01000000                 0  example_function  FINISHED  01000000              NORMAL_TASK  example_function      ffffffffffffffffffffffffffffffffffffffff01000000  6dd489e2b26105214e05f22d97fc21b86c7a83149000287fb08faafa  25a072a9dfdc35b368e7b6a985dcae76cba2ea3c4dedaa7d9fd782c8         20478
 1  32d950ec0ccf9d2affffffffffffffffffffffff01000000                 0  example_function  FINISHED  01000000              NORMAL_TASK  example_function      ffffffffffffffffffffffffffffffffffffffff01000000  6dd489e2b26105214e05f22d97fc21b86c7a83149000287fb08faafa  25a072a9dfdc35b368e7b6a985dcae76cba2ea3c4dedaa7d9fd782c8         20478
 2  c2668a65bda616c1ffffffffffffffffffffffff01000000                 0  example_function  FINISHED  01000000              NORMAL_TASK  example_function      ffffffffffffffffffffffffffffffffffffffff01000000  6dd489e2b26105214e05f22d97fc21b86c7a83149000287fb08faafa  25a072a9dfdc35b368e7b6a985dcae76cba2ea3c4dedaa7d9fd782c8         20478
 3  c8ef45ccd0112571ffffffffffffffffffffffff01000000                 0  example_function  FINISHED  01000000              NORMAL_TASK  example_function      ffffffffffffffffffffffffffffffffffffffff01000000  6dd489e2b26105214e05f22d97fc21b86c7a83149000287fb08faafa  25a072a9dfdc35b368e7b6a985dcae76cba2ea3c4dedaa7d9fd782c8         20478
```

### Actors

```python
import ray

@ray.remote
class Counter:
    def __init__(self):
        self.value = 0
    def increment(self):
        self.value += 1
        return self.value
    def get_counter(self):
        return self.value

counters = [Counter.remote() for _ in range(10)]

results = ray.get([c.increment.remote() for c in counters])
print(results)

results = ray.get([counters[0].increment.remote() for _ in range(5)])
print(results)

results = ray.get([c.get_counter.remote() for c in counters])
print(results)
```

実行結果

```
$ python3 test.py 
[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
[2, 3, 4, 5, 6]
[6, 1, 1, 1, 1, 1, 1, 1, 1, 1]
```

`ray list actors`にて実行されたactorsが確認できる。

```
$ ray list actors

======== List: 2023-10-09 23:43:37.461942 ========
Stats:
------------------------------
Total: 10

Table:
------------------------------
    ACTOR_ID                          CLASS_NAME    STATE      JOB_ID  NAME    NODE_ID                                                     PID  RAY_NAMESPACE
 0  5188a8aa5c5f6b88b65bda5b04000000  Counter       DEAD     04000000          6dd489e2b26105214e05f22d97fc21b86c7a83149000287fb08faafa  25026  91a3cf6f-5f5c-4a16-b5a8-903e986b33e8
 1  70980acaf65af3f118fbc76b04000000  Counter       DEAD     04000000          6dd489e2b26105214e05f22d97fc21b86c7a83149000287fb08faafa  25029  91a3cf6f-5f5c-4a16-b5a8-903e986b33e8
 2  7150eb8db35ad4a017de536c04000000  Counter       DEAD     04000000          6dd489e2b26105214e05f22d97fc21b86c7a83149000287fb08faafa  25022  91a3cf6f-5f5c-4a16-b5a8-903e986b33e8
 3  bcc950d7f0e882348a1bde3b04000000  Counter       DEAD     04000000          6dd489e2b26105214e05f22d97fc21b86c7a83149000287fb08faafa  25021  91a3cf6f-5f5c-4a16-b5a8-903e986b33e8
 4  c7e7d617ee74bf591e2415fe04000000  Counter       DEAD     04000000          6dd489e2b26105214e05f22d97fc21b86c7a83149000287fb08faafa  25023  91a3cf6f-5f5c-4a16-b5a8-903e986b33e8
 5  d80010af34bce917989a629c04000000  Counter       DEAD     04000000          6dd489e2b26105214e05f22d97fc21b86c7a83149000287fb08faafa  25028  91a3cf6f-5f5c-4a16-b5a8-903e986b33e8
 6  e87daa89d8fb38e23e6422ee04000000  Counter       DEAD     04000000          6dd489e2b26105214e05f22d97fc21b86c7a83149000287fb08faafa  25025  91a3cf6f-5f5c-4a16-b5a8-903e986b33e8
 7  ec9e7cb7c1a304adec6bc99804000000  Counter       DEAD     04000000          6dd489e2b26105214e05f22d97fc21b86c7a83149000287fb08faafa  25024  91a3cf6f-5f5c-4a16-b5a8-903e986b33e8
 8  f07ce46668b3459f68b89d3404000000  Counter       DEAD     04000000          6dd489e2b26105214e05f22d97fc21b86c7a83149000287fb08faafa  25020  91a3cf6f-5f5c-4a16-b5a8-903e986b33e8
 9  f152588e1d9c00309b46013f04000000  Counter       DEAD     04000000          6dd489e2b26105214e05f22d97fc21b86c7a83149000287fb08faafa  25027  91a3cf6f-5f5c-4a16-b5a8-903e986b33e8
```

### Objects

```python
import time
import ray

@ray.remote
def echo_and_get(x_list): 
    print("args:", x_list)
    print("values:", ray.get(x_list))

a, b, c = ray.put(1), ray.put(2), ray.put(3)
# 確認のためsleep
time.sleep(100)

echo_and_get.remote([a, b, c])
```

実行結果

```
$ python3 test.py 
(echo_and_get pid=20469) args: [ObjectRef(00ffffffffffffffffffffffffffffffffffffff1100000001e1f505), ObjectRef(00ffffffffffffffffffffffffffffffffffffff1100000002e1f505), ObjectRef(00ffffffffffffffffffffffffffffffffffffff1100000003e1f505)]
(echo_and_get pid=20469) values: [1, 2, 3]
```

sleep中に`ray list objects`を使い現在putされているobjectsを確認することができる。

```
$ ray list objects
・・・
Table:
------------------------------
    OBJECT_ID                                                 OBJECT_SIZE    TASK_STATUS    REFERENCE_TYPE    CALL_SITE    TYPE      PID  IP
 0  00ffffffffffffffffffffffffffffffffffffff0e00000001e1f505  15.000 B       FINISHED       LOCAL_REFERENCE   disabled     DRIVER  43355  127.0.0.1
 1  00ffffffffffffffffffffffffffffffffffffff0e00000002e1f505  15.000 B       FINISHED       LOCAL_REFERENCE   disabled     DRIVER  43355  127.0.0.1
 2  00ffffffffffffffffffffffffffffffffffffff0e00000003e1f505  15.000 B       FINISHED       LOCAL_REFERENCE   disabled     DRIVER  43355  127.0.0.1
```

## 所感

k8sなどでスケーリングしやすいのと、Actorという概念が良いなと思いました。
また、「PlacementGroup」を使いResourceをWaitすることができる部分が
機械学習系のWorkflowでは便利そう。