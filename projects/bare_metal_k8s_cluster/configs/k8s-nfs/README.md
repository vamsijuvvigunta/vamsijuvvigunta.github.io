# External NFS Provisionerr

<!-- TOC -->

- [External NFS Provisionerr](#external-nfs-provisionerr)
- [Source](#source)
- [Modifications](#modifications)
    - [deployment.yaml](#deploymentyaml)
    - [other yaml files](#other-yaml-files)
- [Setup](#setup)
- [Status test](#status-test)
- [Functionality test](#functionality-test)
    - [Create a claim](#create-a-claim)
    - [Create a POD using the claim](#create-a-pod-using-the-claim)
    - [Cleanup](#cleanup)

<!-- /TOC -->

# Source

 * `cd github`
 * `git clone https://github.com/kubernetes-sigs/nfs-subdir-external-provisioner.git`
 * Use the files under `nfs-subdir-external-provisioner/deploy` per https://github.com/kubernetes-sigs/nfs-subdir-external-provisioner
 * Complete tips on how to use the files at https://www.ibm.com/support/pages/how-do-i-create-storage-class-nfs-dynamic-storage-provisioning-openshift-environment

# Modifications

## deployment.yaml

* Update NFS server to the NAS IP
* Update path to the export `/data/K8s`

```diff
-            server: 10.3.243.101
+            server: 192.168.23.190
-            value: /ifs/kubernetes
-            value: /data/K8s
```

## other yaml files

 * rbac.yaml
 * class.yaml


# Setup

 * `export ENABLE_LEADER_ELECTION=false`
 * `kubectl create -f rbac.yaml`
 * `kubectl create -f class.yaml`

```console
vamsi@MAAS:~/github/nfs-subdir-external-provisioner/deploy$ kubectl get sc
NAME                  PROVISIONER                                   RECLAIMPOLICY   VOLUMEBINDINGMODE   ALLOWVOLUMEEXPANSION   AGE
managed-nfs-storage   k8s-sigs.io/nfs-subdir-external-provisioner   Delete          Immediate           false                  7s
```


 * `kubectl create -f deployment.yaml`

```console
vamsi@MAAS:~/bitbucket/infrastructure/k8s/nfs$ kubectl get all
NAME                                          READY   STATUS              RESTARTS   AGE
pod/nfs-client-provisioner-864f8b7f4d-4qkts   0/1     ContainerCreating   0          8s

NAME                 TYPE        CLUSTER-IP     EXTERNAL-IP   PORT(S)   AGE
service/kubernetes   ClusterIP   10.152.183.1   <none>        443/TCP   75m

NAME                                     READY   UP-TO-DATE   AVAILABLE   AGE
deployment.apps/nfs-client-provisioner   0/1     1            0           8s

NAME                                                DESIRED   CURRENT   READY   AGE
replicaset.apps/nfs-client-provisioner-864f8b7f4d   1         1         0       8s
```
# Status test

Check the status of the provisioner pod
> Compared to the status of the service provisioned via the juju nfs relation, this one actually shows events and the containers! Yay!
```console
vamsi@MAAS:~/bitbucket/infrastructure/k8s/nfs$ kubectl describe pod nfs-client-provisioner-864f8b7f4d
Name:         nfs-client-provisioner-864f8b7f4d-4qkts
Namespace:    default
Priority:     0
Node:         tiny2/192.168.23.119
Start Time:   Thu, 26 Aug 2021 20:16:53 -0700
Labels:       app=nfs-client-provisioner
              pod-template-hash=864f8b7f4d
Annotations:  kubernetes.io/psp: privileged
Status:       Running
IP:           10.1.73.10
IPs:
  IP:           10.1.73.10
Controlled By:  ReplicaSet/nfs-client-provisioner-864f8b7f4d
Containers:
  nfs-client-provisioner:
    Container ID:   containerd://36cc249c54d0c931dff3db961f54627df50db99cde7078fbf05fd21fa2919a0e
    Image:          k8s.gcr.io/sig-storage/nfs-subdir-external-provisioner:v4.0.2
    Image ID:       k8s.gcr.io/sig-storage/nfs-subdir-external-provisioner@sha256:63d5e04551ec8b5aae83b6f35938ca5ddc50a88d85492d9731810c31591fa4c9
    Port:           <none>
    Host Port:      <none>
    State:          Running
      Started:      Thu, 26 Aug 2021 20:17:07 -0700
    Ready:          True
    Restart Count:  0
    Environment:
      PROVISIONER_NAME:  k8s-sigs.io/nfs-subdir-external-provisioner
      NFS_SERVER:        192.168.23.190
      NFS_PATH:          /data/K8s
    Mounts:
      /persistentvolumes from nfs-client-root (rw)
      /var/run/secrets/kubernetes.io/serviceaccount from kube-api-access-89t2j (ro)
Conditions:
  Type              Status
  Initialized       True 
  Ready             True 
  ContainersReady   True 
  PodScheduled      True 
Volumes:
  nfs-client-root:
    Type:      NFS (an NFS mount that lasts the lifetime of a pod)
    Server:    192.168.23.190
    Path:      /data/K8s
    ReadOnly:  false
  kube-api-access-89t2j:
    Type:                    Projected (a volume that contains injected data from multiple sources)
    TokenExpirationSeconds:  3607
    ConfigMapName:           kube-root-ca.crt
    ConfigMapOptional:       <nil>
    DownwardAPI:             true
QoS Class:                   BestEffort
Node-Selectors:              <none>
Tolerations:                 node.kubernetes.io/not-ready:NoExecute op=Exists for 300s
                             node.kubernetes.io/unreachable:NoExecute op=Exists for 300s
Events:
  Type    Reason     Age   From               Message
  ----    ------     ----  ----               -------
  Normal  Scheduled  44s   default-scheduler  Successfully assigned default/nfs-client-provisioner-864f8b7f4d-4qkts to tiny2
  Normal  Pulling    33s   kubelet            Pulling image "k8s.gcr.io/sig-storage/nfs-subdir-external-provisioner:v4.0.2"
  Normal  Pulled     31s   kubelet            Successfully pulled image "k8s.gcr.io/sig-storage/nfs-subdir-external-provisioner:v4.0.2" in 2.795733834s
  Normal  Created    30s   kubelet            Created container nfs-client-provisioner
  Normal  Started    30s   kubelet            Started container nfs-client-provisioner

```

# Functionality test

## Create a claim

```yaml
kind: PersistentVolumeClaim
apiVersion: v1
metadata:
  name: test-claim
spec:
  storageClassName: managed-nfs-storage
  accessModes:
    - ReadWriteMany
  resources:
    requests:
      storage: 1Mi
```

`kubectl create -f test-claim.yaml`
```console
vamsi@MAAS:~/bitbucket/infrastructure/k8s/nfs$ kubectl create -f test-claim.yaml 
persistentvolumeclaim/test-claim created
vamsi@MAAS:~/bitbucket/infrastructure/k8s/nfs$ kubectl get pvc
NAME         STATUS   VOLUME                                     CAPACITY   ACCESS MODES   STORAGECLASS          AGE
test-claim   Bound    pvc-171be648-44d3-445a-a9e4-d6d4ce84d0d4   1Mi        RWX            managed-nfs-storage   6s
```console

Check the NFS dir and I see a default-test-claim... that corresponds to the PVC
```console
vamsi@MAAS:/mnt/K8s$ ls -l
total 0
drwxrwxrwx 1 root  root   0 Nov 28  2020 default-pvc1-pvc-fc82dddc-8fd3-4411-9e12-e5e957ac2bee
drwxrwxrwx 1 root  root   0 Aug 26 20:41 default-test-claim-pvc-171be648-44d3-445a-a9e4-d6d4ce84d0d4
-rw-rw-rw- 1 vamsi vamsi  0 Nov 28  2020 hello.txt
drwxrwxrwx 1 root  root  40 Aug 26 11:02 kubernetes-worker
drwxrwxrwx 1    98    98  0 Aug 26 10:01 test
```

## Create a POD using the claim

```yaml
kind: Pod
apiVersion: v1
metadata:
  name: test-pod
spec:
  containers:
  - name: test-pod
    image: gcr.io/google_containers/busybox:1.24
    command:
      - "/bin/sh"
    args:
      - "-c"
      - "touch /mnt/SUCCESS && exit 0 || exit 1"
    volumeMounts:
      - name: nfs-pvc
        mountPath: "/mnt"
  restartPolicy: "Never"
  volumes:
    - name: nfs-pvc
      persistentVolumeClaim:
        claimName: test-claim
```

`kubectl creat -f test-pod.yaml`
```console
```

 * Uses the claim name: `test-claim`
 * What is this volumes name ?
 * Simple script that uses the busybox
 * And yes, the NFS dir has this SUCCESS file.

 ## Cleanup

 `kubectl delete -f test-claim.yaml -f test-pod.yaml`