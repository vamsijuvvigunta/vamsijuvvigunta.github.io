<!-- TOC -->

- [Status](#status)
- [Juju and k8s storage](#juju-and-k8s-storage)
- [Resources](#resources)
- [Setup storage](#setup-storage)
- [K8s concepts](#k8s-concepts)
    - [Static strorage](#static-strorage)
    - [Dynamic storage](#dynamic-storage)
- [History - Attempt Juju storage on external NFS](#history---attempt-juju-storage-on-external-nfs)
    - [Configure and test NAS share](#configure-and-test-nas-share)
    - [Test via NFS mount](#test-via-nfs-mount)
    - [Test juju charm nfs mounting](#test-juju-charm-nfs-mounting)
    - [Tying juju and k8s storage together](#tying-juju-and-k8s-storage-together)
        - [Test k8s storage via PVC](#test-k8s-storage-via-pvc)
    - [Plain K8s storage](#plain-k8s-storage)
        - [Add RBAC](#add-rbac)
        - [Create storage class](#create-storage-class)
        - [Deploy provisioner](#deploy-provisioner)
        - [Test provisioner](#test-provisioner)
    - [Make storage class default](#make-storage-class-default)
    - [Retest  nfs charm k8s storage](#retest--nfs-charm-k8s-storage)
        - [Why is juju not showing kubernetes pool](#why-is-juju-not-showing-kubernetes-pool)
    - [✅️ Official nfs-subir provisioner](#%EF%B8%8F-official-nfs-subir-provisioner)

<!-- /TOC -->

# Status

- ✅️ Kubernetes-sigs/nfs-subdir-external-provisioner
- My own charm works but I see no containers in the deployed pods. So ditching it

# Juju and k8s storage

Kubernetes has a complex storage setup: a layering of abstractions when your storage can be anywhere in the cloud. Looks like juju has similar ones too but can help initiailize k8s storage from it's own. I am just trying to figure out from various sources how I can get storage for 
- kubernetes itself
- kubernetes apps including kubeflow
- juju storage is a different beast that is not particularly useful for me now.

# Resources

- **Main**
  - ✅️ https://github.com/kubernetes-sigs/nfs-subdir-external-provisioner
  - https://discourse.charmhub.io/t/setting-up-static-kubernetes-storage-tutorial/1193
- Secondary:
  - https://medium.com/@knobby/nfs-default-storage-in-kubernetes-with-cdk-847336cc4a72
  - Kubernetes NFS provisioning
    - https://blog.exxactcorp.com/deploying-dynamic-nfs-provisioning-in-kubernetes/
    - https://app.cnvrg.io/docs/guides/on-prem-volumes.html#deploy-the-nfs-provisioner 
    - These directly provision nfs for kubernetes. Useful but it seems simpler to go with juju and setup a juju relationship

# Setup storage

 **Pre-Reqs**
 - NAS and using ext4 fs
 - NAS reachable from cluster switch (_MAAS provided DHCP in my case_)
 - K8s deployed on MAAS cluster

**Steps**
 * `cd bitbucket/infrastructure/configs/k8s-nfs`
 * `export ENABLE_LEADER_ELECTION=false`
 * `kubectl create -f rbac.yaml`
 * `kubectl create -f class.yaml`
 * `kubectl create -f deployment.yaml`
 * `kubectl patch storageclass managed-nfs-storage -p '{"metadata": {"annotations":{"storageclass.kubernetes.io/is-default-class":"true"}}}'`

**Test**
 * K8s checks
   * `kubectl get sc` _should show the new storage class marked as default_
 * Pod level checks
   * `kubectl get all` _locate the nfs-client-provisioner-xxx_
   * `kubectl describe pod nfs-client-provisioner-xxxx` _verify therer is an events section that shows image pull, container start etc_
 * functionality checks
   * `kubectl create -f test-claim.yaml`
   * `kubectl get all pv,pvc` _you should see a test-claim pvc_
   * `kubectl create -f test-pod.yaml` _you should see a SUCCESS file in the pv on the NAS_

# K8s concepts

The main goal here is that you can autoscale!

- You can have many pods of different types running
- Each could need it's own storage
- Since the scaling happens automatically, storage allocation also needs to happens automatically


## Static strorage

PV (*persistent volumes*)

These are manually specified in yaml files and deployed via `kubectl`. One can reuse the same PV for multiple pods I think. Pain mostly because you need manual intervention.

## Dynamic storage 

- Provisioners
- Persistent Volume Claims (PVCs)

The key is the `claim` which is satisfied by a `provisioner`. No manual intervention needed.


# History - Attempt Juju storage on external NFS

There seems to be no direct support for juju storage on external nfs. All the examples use `ceph`, `aws` etc. The abstractions juju uses seems to mirror the k8s ones.
 - juju storage class is the type of storage
 - juju storage pools are storage pools created for a certain storage-class
 - these seem to be a thin abstraction over the cloud-native concepts of `storageclass`, `provider` etc.

I am trying to see about providing storage for the kubernetes clouds that juju manages via NFS from my existing NAS. I think the way this works is to work at a kubernetes-level (which is a *cloud* for juju) and then make juju use the kubernetes provided storage classes.

## Configure and test NAS share

Added a share called `K8s` on vamsi-nas and turned on nfs access.

## Test via NFS mount

`sudo apt install nfs-common` was needed to get `showmount`

```console
vamsi@MAAS:~$ showmount -e vamsi-nas
Export list for vamsi-nas:
/data/Pub    *
/data/K8s    *
/data/Backup *
/data/Vamsi  127.0.0.1
/data/Kumar  127.0.0.1
/data/Hema   127.0.0.1
```

Now try to mount!

```console
vamsi@MAAS:~$ sudo mount vamsi-nas:/data/K8s /mnt/K8s
```

Success! Unmount it 

```console
vamsi@MAAS:~$ sudo umount /mnt/K8s
```

## Test juju charm nfs mounting

- Uses the `remote-nfs` charm (See [remote-nfs on charm store](https://jaas.ai/u/chris.macnaughton/remote-nfs/2))

`juju deploy -m k8s cs:~chris.macnaughton/remote-nfs-2 --config "nfs-server=vamsi-nas" --config "nfs-target=/data/K8s" --config "local-target=/mnt/cluster-io"`

```console
vamsi@MAAS:~$ juju deploy -m juju-k8s cs:~chris.macnaughton/remote-nfs-2 --config "nfs-server=vamsi-nas" --config "nfs-target=/data/K8s" --config "local-target=/mnt/cluster-io"
Located charm "cs:~chris.macnaughton/remote-nfs-2".
Deploying charm "cs:~chris.macnaughton/remote-nfs-2".
```

<pre><font color="#4E9A06"><b>vamsi@MAAS</b></font>:<font color="#3465A4"><b>~</b></font>$ juju status
Model  Controller       Cloud/Region     Version  SLA          Timestamp
juju-k8s    juju-controller  devmaas/default  2.8.6    unsupported  10:41:55-08:00

App                    Version  Status   Scale  Charm                  Store       Rev  OS      Notes
containerd             1.3.3    <font color="#4E9A06">active       </font>3  containerd             jujucharms   96  ubuntu  
easyrsa                3.0.1    <font color="#4E9A06">active       </font>1  easyrsa                jujucharms  338  ubuntu  
etcd                   3.4.5    <font color="#4E9A06">active       </font>1  etcd                   jujucharms  543  ubuntu  
flannel                0.11.0   <font color="#4E9A06">active       </font>3  flannel                jujucharms  512  ubuntu  
kubeapi-load-balancer  1.18.0   <font color="#4E9A06">active       </font>1  kubeapi-load-balancer  jujucharms  752  ubuntu  exposed
kubernetes-master      1.19.4   <font color="#4E9A06">active       </font>1  kubernetes-master      jujucharms  908  ubuntu  
kubernetes-worker      1.19.4   <font color="#4E9A06">active       </font>2  kubernetes-worker      jujucharms  711  ubuntu  exposed
remote-nfs                      <font color="#C4A000">unknown      </font>0  remote-nfs             jujucharms    2  ubuntu  

Unit                      Workload  Agent  Machine  Public address  Ports           Message
easyrsa/0*                <font color="#4E9A06">active    idle   </font>0        192.168.23.103                  Certificate Authority connected.
etcd/0*                   <font color="#4E9A06">active    idle   </font>0        192.168.23.103  2379/tcp        Healthy with 1 known peer
kubeapi-load-balancer/0*  <font color="#4E9A06">active    idle   </font>0        192.168.23.103  443/tcp         Loadbalancer ready.
kubernetes-master/0*      <font color="#4E9A06">active    idle   </font>0        192.168.23.103  6443/tcp        Kubernetes master running.
  containerd/2            <font color="#4E9A06">active    idle   </font>         192.168.23.103                  Container runtime available
  flannel/2               <font color="#4E9A06">active    idle   </font>         192.168.23.103                  Flannel subnet 10.1.6.1/24
kubernetes-worker/0*      <font color="#4E9A06">active    idle   </font>1        192.168.23.104  80/tcp,443/tcp  Kubernetes worker running.
  containerd/1            <font color="#4E9A06">active    idle   </font>         192.168.23.104                  Container runtime available
  flannel/1               <font color="#4E9A06">active    idle   </font>         192.168.23.104                  Flannel subnet 10.1.72.1/24
kubernetes-worker/1       <font color="#4E9A06">active    idle   </font>2        192.168.23.105  80/tcp,443/tcp  Kubernetes worker running.
  containerd/0*           <font color="#4E9A06">active    idle   </font>         192.168.23.105                  Container runtime available
  flannel/0*              <font color="#4E9A06">active    idle   </font>         192.168.23.105                  Flannel subnet 10.1.22.1/24

Machine  State    DNS             Inst id  Series  AZ       Message
0        <font color="#4E9A06">started  </font>192.168.23.103  tiny1    focal   default  Deployed
1        <font color="#4E9A06">started  </font>192.168.23.104  tiny2    focal   default  Deployed
2        <font color="#4E9A06">started  </font>192.168.23.105  big-boy  focal   default  Deployed
</pre>

> Note the `remote-nfs` name even though we installed `remote-nfs-2` charm

## Tying juju and k8s storage together

Example show a relation to `kubernetes-worker` and says that the relationship can work on either kubernetes-worker or kubernetes-master.

<pre><font color="#4E9A06"><b>vamsi@MAAS</b></font>:<font color="#3465A4"><b>~</b></font>$ juju add-relation kubernetes-worker remote-nfs
</pre>


- This failed badly
- Needed some fixes to deal with python 3.8. See [Problem log](./problemLog.md#remote-nfs-install-fails) for how this was eventually fixed
  - Repackage the charmhelpers python libs with a fix
  - install charm from local disk.

> The learning here is to examine the metadata.yaml files for all involved charms. You need to match `requires` and `provides` sections. I was expecting magic but there is no such. The relation only works when one charm provides and the other requires a `nfs` endpoint with a `mount` interface.

Now everything works and the status shows the nfs paths!

<pre><font color="#4E9A06"><b>vamsi@MAAS</b></font>:<font color="#3465A4"><b>~</b></font>$ juju status --color
Model  Controller       Cloud/Region     Version  SLA          Timestamp
k8s    juju-controller  devmaas/default  2.8.6    unsupported  08:21:23-08:00

App                    Version  Status  Scale  Charm                  Store       Rev  OS      Notes
containerd             1.3.3    <font color="#4E9A06">active      </font>3  containerd             jujucharms   96  ubuntu  
easyrsa                3.0.1    <font color="#4E9A06">active      </font>1  easyrsa                jujucharms  338  ubuntu  
etcd                   3.4.5    <font color="#4E9A06">active      </font>1  etcd                   jujucharms  543  ubuntu  
flannel                0.11.0   <font color="#4E9A06">active      </font>3  flannel                jujucharms  512  ubuntu  
kubeapi-load-balancer  1.18.0   <font color="#4E9A06">active      </font>1  kubeapi-load-balancer  jujucharms  752  ubuntu  exposed
kubernetes-master      1.19.4   <font color="#4E9A06">active      </font>1  kubernetes-master      jujucharms  908  ubuntu  
kubernetes-worker      1.19.4   <font color="#4E9A06">active      </font>2  kubernetes-worker      jujucharms  711  ubuntu  exposed
remote-nfs                      <font color="#4E9A06">active      </font>2  remote-nfs             local         1  ubuntu  

Unit                      Workload  Agent  Machine  Public address  Ports           Message
easyrsa/0*                <font color="#4E9A06">active    idle   </font>0        192.168.23.103                  Certificate Authority connected.
etcd/0*                   <font color="#4E9A06">active    idle   </font>0        192.168.23.103  2379/tcp        Healthy with 1 known peer
kubeapi-load-balancer/0*  <font color="#4E9A06">active    idle   </font>0        192.168.23.103  443/tcp         Loadbalancer ready.
kubernetes-master/0*      <font color="#4E9A06">active    idle   </font>0        192.168.23.103  6443/tcp        Kubernetes master running.
  containerd/2            <font color="#4E9A06">active    idle   </font>         192.168.23.103                  Container runtime available
  flannel/2               <font color="#4E9A06">active    idle   </font>         192.168.23.103                  Flannel subnet 10.1.6.1/24
kubernetes-worker/0*      <font color="#4E9A06">active    idle   </font>1        192.168.23.104  80/tcp,443/tcp  Kubernetes worker running.
  containerd/1            <font color="#4E9A06">active    idle   </font>         192.168.23.104                  Container runtime available
  flannel/1               <font color="#4E9A06">active    idle   </font>         192.168.23.104                  Flannel subnet 10.1.72.1/24
  remote-nfs/5            <font color="#4E9A06">active    idle   </font>         192.168.23.104                  vamsi-nas:/data/K8s -&gt; /mnt/cluster-io
kubernetes-worker/1       <font color="#4E9A06">active    idle   </font>2        192.168.23.105  80/tcp,443/tcp  Kubernetes worker running.
  containerd/0*           <font color="#4E9A06">active    idle   </font>         192.168.23.105                  Container runtime available
  flannel/0*              <font color="#4E9A06">active    idle   </font>         192.168.23.105                  Flannel subnet 10.1.22.1/24
  remote-nfs/4*           <font color="#4E9A06">active    idle   </font>         192.168.23.105                  vamsi-nas:/data/K8s -&gt; /mnt/cluster-io

Machine  State    DNS             Inst id  Series  AZ       Message
0        <font color="#4E9A06">started  </font>192.168.23.103  tiny1    focal   default  Deployed
1        <font color="#4E9A06">started  </font>192.168.23.104  tiny2    focal   default  Deployed
2        <font color="#4E9A06">started  </font>192.168.23.105  big-boy  focal   default  Deployed
</pre>

### Test k8s storage via PVC

Claim a PVC and see what happens (courtesy: https://medium.com/@knobby/nfs-default-storage-in-kubernetes-with-cdk-847336cc4a72)

`kubectl get sc` Fails!

```console
vamsi@MAAS:~$ kubectl get sc
No resources found
```

> Figured out much later after I created the [remote-nfs-mount charm](charms.md) and dug into the code for kubernetes-worker charm that it really needs the `nfs` relation: specifically one that implements the `mount` interface.

**Create a PVC**
```console
vamsi@MAAS:~ kubectl apply -f - <<EOF
kind: PersistentVolumeClaim
apiVersion: v1
metadata:
  name: test
spec:
  accessModes:
    - ReadWriteMany
  resources:
    requests:
      storage: 1Gi
EOF

persistentvolumeclaim/test created

vamsi@MAAS:~$ kubectl get pv
No resources found
```

Gah! Still broken!!

**Juju pools**

```console
vamsi@MAAS:~$ juju storage-pools
Name    Provider  Attrs
loop    loop      
maas    maas      
rootfs  rootfs    
tmpfs   tmpfs     
```

So looks like it is not even showing up as a storage-pool.

> From what I read, this needs the addition of a `k8s` cloud to juju and then it automatically adds a `kubernetes` pool. I thought I did it but still no luck. Dig more.

## Plain K8s storage

No juju involvement here. Follow instructions from https://blog.exxactcorp.com/deploying-dynamic-nfs-provisioning-in-kubernetes/

- Seems complete
- Examples for creating PVC and PVs
- Example pod that uses a PVC

### Add RBAC 

role based authorization for nfs ?

```console
vamsi@MAAS:~/bitbucket/nfs-provisioning$ kubectl create -f rbac.yaml 
serviceaccount/nfs-client-provisioner created
clusterrole.rbac.authorization.k8s.io/nfs-client-provisioner-runner created
clusterrolebinding.rbac.authorization.k8s.io/run-nfs-client-provisioner created
role.rbac.authorization.k8s.io/leader-locking-nfs-client-provisioner created
rolebinding.rbac.authorization.k8s.io/leader-locking-nfs-client-provisioner created
vamsi@MAAS:~/bitbucket/nfs-provisioning$ 
```

Verify

```console
amsi@MAAS:~/bitbucket/nfs-provisioning$ kubectl get clusterrole,clusterrolebinding,role,rolebinding | grep nfs
clusterrole.rbac.authorization.k8s.io/nfs-client-provisioner-runner                                          2020-11-28T18:06:30Z
clusterrolebinding.rbac.authorization.k8s.io/run-nfs-client-provisioner                                 ClusterRole/nfs-client-provisioner-runner                          2m47s
role.rbac.authorization.k8s.io/leader-locking-nfs-client-provisioner   2020-11-28T18:06:30Z
rolebinding.rbac.authorization.k8s.io/leader-locking-nfs-client-provisioner   Role/leader-locking-nfs-client-provisioner   2m47s
```

### Create storage class

- They retain a example.com/nfs as provider. This **is important**. Changing it to "vamsi-nas/K8s" and keeping it consistent across the other files fails PV provisioning later on.

```console
vamsi@MAAS:~/bitbucket/nfs-provisioning$ kubectl create -f class.yaml 
storageclass.storage.k8s.io/managed-nfs-storage created
vamsi@MAAS:~/bitbucket/nfs-provisioning$ kubectl get storageclass
NAME                  PROVISIONER       RECLAIMPOLICY   VOLUMEBINDINGMODE   ALLOWVOLUMEEXPANSION   AGE
managed-nfs-storage   example.com/nfs   Delete          Immediate           false                  9s
```

<pre><font color="#4E9A06"><b>vamsi@MAAS</b></font>:<font color="#3465A4"><b>~/bitbucket/nfs-provisioning</b></font>$ kubectl create -f class.yaml 
storageclass.storage.k8s.io/managed-nfs-storage created
<font color="#4E9A06"><b>vamsi@MAAS</b></font>:<font color="#3465A4"><b>~/bitbucket/nfs-provisioning</b></font>$ kubectl get storageclass
NAME                  PROVISIONER       RECLAIMPOLICY   VOLUMEBINDINGMODE   ALLOWVOLUMEEXPANSION   AGE
managed-nfs-storage   example.com/nfs   Delete          Immediate           false                  9s
</pre>

### Deploy provisioner

- Update the NFS server IPs in the file to vamsi-nas's IP of 192.168.1.228
- Update paths to /data/K8s

End up with the following:

```yaml
kind: Deployment
apiVersion: apps/v1
metadata:
  name: nfs-client-provisioner
spec:
  selector:
    matchLabels:
      app: nfs-client-provisioner
  replicas: 1
  strategy:
    type: Recreate
  template:
    metadata:
      labels:
        app: nfs-client-provisioner
    spec:
      serviceAccountName: nfs-client-provisioner
      containers:
        - name: nfs-client-provisioner
          image: quay.io/external_storage/nfs-client-provisioner:latest
          volumeMounts:
            - name: nfs-client-root
              mountPath: /persistentvolumes
          env:
            - name: PROVISIONER_NAME
              value: example.com/nfs
            - name: NFS_SERVER
              value: 192.168.1.228
            - name: NFS_PATH
              value: /data/K8s
      volumes:
        - name: nfs-client-root
          nfs:
            server: 192.168.1.228
            path: /data/K8s
```

```console
vamsi@MAAS:~/bitbucket/nfs-provisioning$ kubectl create -f deployment.yaml 
deployment.apps/nfs-client-provisioner created
```

Verify pods created

```console
vamsi@MAAS:~/bitbucket/nfs-provisioning$ kubectl get all
NAME                                          READY   STATUS    RESTARTS   AGE
pod/nfs-client-provisioner-57c66fdb77-5v7hz   1/1     Running   0          25s

NAME                 TYPE        CLUSTER-IP     EXTERNAL-IP   PORT(S)   AGE
service/kubernetes   ClusterIP   10.152.183.1   <none>        443/TCP   37h

NAME                                     READY   UP-TO-DATE   AVAILABLE   AGE
deployment.apps/nfs-client-provisioner   1/1     1            1           26s

NAME                                                DESIRED   CURRENT   READY   AGE
replicaset.apps/nfs-client-provisioner-57c66fdb77   1         1         1       26s
```

Get some details. Use the *provisioner* pod name listed above

```console
vamsi@MAAS:~$ kubectl describe pod nfs-client-provisioner-7f4dcddb7d-kr55w
Name:         nfs-client-provisioner-7f4dcddb7d-kr55w
Namespace:    default
Priority:     0
Node:         big-boy/192.168.23.105
Start Time:   Sat, 28 Nov 2020 11:28:46 -0800
Labels:       app=nfs-client-provisioner
              pod-template-hash=7f4dcddb7d
Annotations:  kubernetes.io/psp: privileged
Status:       Running
IP:           10.1.22.13
IPs:
  IP:           10.1.22.13
Controlled By:  ReplicaSet/nfs-client-provisioner-7f4dcddb7d
Containers:
  nfs-client-provisioner:
    Container ID:   containerd://858ff332e17608543532e77a79c10402a2c2ca4917da90e9841c32e92807b83c
    Image:          quay.io/external_storage/nfs-client-provisioner:latest
    Image ID:       sha256:fb50e11b84fec7ee64a2ae3d4f9d36edc358d151c95dbd796c03b9dd1dc1a446
    Port:           <none>
    Host Port:      <none>
    State:          Running
      Started:      Sat, 28 Nov 2020 11:28:48 -0800
    Ready:          True
    Restart Count:  0
    Environment:
      PROVISIONER_NAME:  example.com/nfs
      NFS_SERVER:        192.168.1.228
      NFS_PATH:          /data/K8s
    Mounts:
      /persistentvolumes from nfs-client-root (rw)
      /var/run/secrets/kubernetes.io/serviceaccount from nfs-client-provisioner-token-zmh7j (ro)
Conditions:
  Type              Status
  Initialized       True 
  Ready             True 
  ContainersReady   True 
  PodScheduled      True 
Volumes:
  nfs-client-root:
    Type:      NFS (an NFS mount that lasts the lifetime of a pod)
    Server:    192.168.1.228
    Path:      /data/K8s
    ReadOnly:  false
  nfs-client-provisioner-token-zmh7j:
    Type:        Secret (a volume populated by a Secret)
    SecretName:  nfs-client-provisioner-token-zmh7j
    Optional:    false
QoS Class:       BestEffort
Node-Selectors:  <none>
Tolerations:     node.kubernetes.io/not-ready:NoExecute op=Exists for 300s
                 node.kubernetes.io/unreachable:NoExecute op=Exists for 300s
Events:          <none>
```

### Test provisioner

```console
vamsi@MAAS:~/bitbucket/nfs-provisioning$ kubectl create -f 4-pvc-nfs.yaml 
persistentvolumeclaim/pvc1 created
```

```console
vamsi@MAAS:~$ kubectl get pv,pvc
NAME                                                        CAPACITY   ACCESS MODES   RECLAIM POLICY   STATUS   CLAIM          STORAGECLASS          REASON   AGE
persistentvolume/pvc-fc82dddc-8fd3-4411-9e12-e5e957ac2bee   500Mi      RWX            Delete           Bound    default/pvc1   managed-nfs-storage            5h8m

NAME                         STATUS   VOLUME                                     CAPACITY   ACCESS MODES   STORAGECLASS          AGE
persistentvolumeclaim/pvc1   Bound    pvc-fc82dddc-8fd3-4411-9e12-e5e957ac2bee   500Mi      RWX            managed-nfs-storage   5h8m
```

**Check the NFS dir**

```console
total 4
drwxrwxrwx+ 1    99    99  144 Nov 28 11:29 .
drwxr-xr-x  3 root  root  4096 Nov 27 10:25 ..
drwxrwxrwx+ 1 root  root     0 Nov 28 11:29 default-pvc1-pvc-fc82dddc-8fd3-4411-9e12-e5e957ac2bee
-rw-rw-rw-+ 1 vamsi vamsi    0 Nov 28 09:26 hello.txt
```

Works!!

## Make storage class default

> Kubeflow notebook etc want a default storage class to work well. If there isn't one, they expect the user to create PVs manually.

`kubeflow get storageclass` does not mark the `managed-nfs-storage` as default. Do it here (_the yaml files for creating the storage class itself could be modified but this will work just fine_)

`kubectl patch storageclass managed-nfs-storage -p '{"metadata": {"annotations":{"storageclass.kubernetes.io/is-default-class":"true"}}}'`

```console
vamsi@MAAS:~$ kubectl get storageclass
NAME                            PROVISIONER                                   RECLAIMPOLICY   VOLUMEBINDINGMODE   ALLOWVOLUMEEXPANSION   AGE
managed-nfs-storage (default)   k8s-sigs.io/nfs-subdir-external-provisioner   Delete          Immediate           false                  3d12h
```

## Retest  nfs charm k8s storage

[Mike wilson's article](https://medium.com/@knobby/nfs-default-storage-in-kubernetes-with-cdk-847336cc4a72) demonstrates that the `nfs` charm seems to do what the exxactcorp nfs provisioner example detailed above does. Much simpler if I can get that to work.

I learnt a lot by trying to build my own charm
- remote-nfs-charm` (see charms/remote-nfs-mount)
- reading the code for `kubernetes-worker` shows that it needs the `nfs` endpoing with a `mount` interface
- I copied code from the `nfs` charm and simplified it to take advantage of nfs4 exports but had to ensure that I was indeed providing the `nfs` endpoint.

Once this was done, I can se the manually provisioned storageclass as well as the one that the nfs relationship adds.

```console
juju add-relation remote-nfs-mount:nfs kubernetes-worker
```

```diff
vamsi@MAAS:~/charms/builds$ kubectl get sc
NAME                  PROVISIONER       RECLAIMPOLICY   VOLUMEBINDINGMODE   ALLOWVOLUMEEXPANSION   AGE
+default (default)     fuseim.pri/ifs    Delete          Immediate           false                  86s
managed-nfs-storage   example.com/nfs   Delete          Immediate           false                  78d
```

So. Are we all set ?

### Why is juju not showing kubernetes pool

I have added the `k8s` cloud to juju.

```
vamsi@MAAS:~/bitbucket/infrastructure/charms/remote-nfs-mount$ juju clouds
Only clouds with registered credentials are shown.
There are more clouds, use --all to see them.

Clouds available on the controller:
Cloud         Regions  Default  Type
devmaas       1        default  maas  
juju-cluster  1        default  k8s   

Clouds available on the client:
Cloud         Regions  Default    Type  Credentials  Source    Description
devmaas       1        default    maas  1            local     Metal As A Service
juju-cluster  0                   k8s   1            local     A Kubernetes Cluster
localhost     1        localhost  lxd   0            built-in  LXD Container Hypervisor
```

It was supposed to automatically enable a `kubernetes` storage pool. I see the `type:k8s` cloud but no associated storage-pool.
```
vamsi@MAAS:~/bitbucket/infrastructure/charms/remote-nfs-mount$ juju storage-pools
Name    Provider  Attrs
loop    loop      
maas    maas      
rootfs  rootfs    
tmpfs   tmpfs    
```

https://discourse.charmhub.io/t/setting-up-static-kubernetes-storage-tutorial/1193 hoowever says that we need a k8s-model. *Am assuming all kubernetes deployments are done to the k8s-model*. Since I created MAAS mostly for kubernetes, I sillily called my main model (which includes the juju k8s deployment) `k8s`. Will likely redo the reployment to get things correct. Messed up names
- `juju-cluster` could be `k8s-cloud` to make it clear
- `k8s` model can be `MAAS-model` instead

```
vamsi@MAAS:~/bitbucket/infrastructure/charms/remote-nfs-mount$ juju models
Controller: juju-controller

Model       Cloud/Region     Type  Status     Machines  Cores  Units  Access  Last connection
controller  devmaas/default  maas  available         1      1  -      admin   just now
default     devmaas/default  maas  available         0      -  -      admin   2020-11-27
k8s*        devmaas/default  maas  available         3     80  13     admin   3 minutes ago
```

Ok, add a k8s-model per the page above and see.

```
vamsi@MAAS:~$ juju add-model k8s-model juju-cluster
Added 'k8s-model' model on juju-cluster/default with credential 'juju-cluster' for user 'admin'
```

Now check pools again

```
vamsi@MAAS:~$ juju storage-pools
Name        Provider    Attrs
kubernetes  kubernetes 
```

Yay!! The others are gone since I am in the `k8s-model`: the preceeding `add-model` call made it current!

```
vamsi@MAAS:~/bitbucket/infrastructure/charms/remote-nfs-mount$ juju models
Controller: juju-controller

Model       Cloud/Region          Type        Status     Machines  Cores  Units  Access  Last connection
controller  devmaas/default       maas        available         1      1  -      admin   just now
default     devmaas/default       maas        available         0      -  -      admin   2020-11-27
k8s         devmaas/default       maas        available         3     80  13     admin   9 minutes ago
k8s-model*  juju-cluster/default  kubernetes  available         0      -  -      admin   1 minute ago
```

> **Smack head**: Reading the docs, I did not need to spend the inordinate amount of time I did for the remote-nfs-charm. I could have simply used the `remote-nfs` charm to make sure there was a mount on all the nodes. Then created the PV etc per the page above. Gah!! Anyway, learnt a lot.

The page uses 

```
juju create-storage-pool operator-storage kubernetes \
    storage-class=juju-operator-storage \
    storage-provisioner=kubernetes.io/no-provisioner
```

From my status, looks like I should use

```
juju create-storage-pool operator-storage kubernetes \
    storage-class=storageclass.storage.k8s.io/default \
    storage-provisioner=fuseim.pri/ifs

juju create-storage-pool k8s-pool kubernetes \
    storage-class=storageclass.storage.k8s.io/default\
    storage-provisioner=fuseim.pri/ifs    
```

Check the pools

```
vamsi@MAAS:~$ juju storage-pools
Name              Provider    Attrs
k8s-pool          kubernetes  storage-class=storageclass.storage.k8s.io/default storage-provisioner=fuseim.pri/ifs
kubernetes        kubernetes  
operator-storage  kubernetes  storage-class=storageclass.storage.k8s.io/default storage-provisioner=fuseim.pri/ifs
```

Hmm, worked with no error! Did it really though ? 
- Using native k8s, we can check this by making a claim and by using that claim in a new pod.
- Using juju, try to install kubeflow (which is what started this whole thing) and see if it uses the storage

> This did not workout finally as creatign a k8s pvc did not do anything. It would get stuck at `waiting for provisioner to create volumes`. Later realized that the nfs-client-provisioner-clientxxxx pod did not have any containers running.


## ✅️ Official nfs-subir provisioner

Looks pretty active. Similar to the exxactcorp ones.

 * `cd github`
 * `git clone https://github.com/kubernetes-sigs/nfs-subdir-external-provisioner.git`
 * Use the files under `nfs-subdir-external-provisioner/deploy` per https://github.com/kubernetes-sigs/nfs-subdir-external-provisioner
 * I have saved these under `infrastructure/k8s-nfs`



