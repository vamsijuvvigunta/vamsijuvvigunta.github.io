<!-- TOC -->

- [Juju and Kubeflow on top of kubernetes](#juju-and-kubeflow-on-top-of-kubernetes)
- [Resources](#resources)
- [Installation](#installation)
    - [Which bundle](#which-bundle)
    - [overlay notes](#overlay-notes)
    - [Customize install with overlay](#customize-install-with-overlay)
    - [Install into kubeflow model](#install-into-kubeflow-model)
        - [Create custom kubeflow model](#create-custom-kubeflow-model)
        - [Dry run deployment](#dry-run-deployment)
        - [Deploy](#deploy)
        - [Verify](#verify)
        - [Istio setup](#istio-setup)
- [Configure Kubeflow dashboard for external access](#configure-kubeflow-dashboard-for-external-access)
    - [Export istio-ingress](#export-istio-ingress)
        - [Metallb prereqs](#metallb-prereqs)
        - [Deploy metallb](#deploy-metallb)
        - [Security setup](#security-setup)
        - [Test exposure](#test-exposure)
    - [Exploration](#exploration)
- [History](#history)
    - [Failed attemps](#failed-attemps)
    - [Overlays are different](#overlays-are-different)
    - [Configure Kubeflow dashboard for external access](#configure-kubeflow-dashboard-for-external-access)
    - [Portforwarding](#portforwarding)

<!-- /TOC -->


# Juju and Kubeflow on top of kubernetes

> **This is done after k8s storage is setup and juju is bootstrapped onto the k8ds cloud**. See details in
>- [3-juju-k8s](./3-juju-k8s.md) 
>- [4-juju-k8s-storage](./4-juju-k8s-storage.md)
>
> for final updates to this saga.

*Pre Requisites*
 - k8s cluster with storage
 - juju bootstrapped onto that cluster
 - A bunch of local routable IPs (_I reserved some in MAAS_) to expose kubeflow UI outside the cluster

# Resources
- [Install Kubeflow - Oct 20](https://discourse.charmhub.io/t/install-kubeflow/3670)
  - Bootstrap controller into the k8s cloud
- [Intro to Charmed Kubeflow - Oct 20](https://discourse.charmhub.io/t/introduction-to-charmed-kubeflow/3749)
  - K8s operator
- [Video installation]()
  - Talks through a bunch of low level details (load balancing, proxy etc)
  - Helped finalize the install.

# Installation

Used overlays to customize the bundle the same way I did k8s (_with some differences_)

## Which bundle

![Stable from charmhub](./img/charmhub-kflite-stableversion.png)

- ❌️ should have been **54** but that bundle always just hangs with juju 2.9 and k8s 1.21
- ✅️ bundle 67 which I got from googling it worked (need to still verify if everything works though)

## overlay notes

- [Bundle Reference](https://juju.is/docs/sdk/bundle-reference)
- [K8s well known labels](https://kubernetes.io/docs/reference/labels-annotations-taints/)

> See output of `kubectl get nodes --show-labels` to find current labels

```console
vamsi@MAAS:~/bitbucket$ kubectl get nodes --show-labels
NAME      STATUS   ROLES    AGE   VERSION   LABELS
big-boy   Ready    <none>   14h   v1.21.4   beta.kubernetes.io/arch=amd64,beta.kubernetes.io/os=linux,cuda=true,gpu=true,juju-application=kubernetes-worker,kubernetes.io/arch=amd64,kubernetes.io/hostname=big-boy,kubernetes.io/os=linux
tiny2     Ready    <none>   14h   v1.21.4   beta.kubernetes.io/arch=amd64,beta.kubernetes.io/os=linux,juju-application=kubernetes-worker,kubernetes.io/arch=amd64,kubernetes.io/hostname=tiny2,kubernetes.io/os=linux
```

I find the following pre-existing labels
 * `beta.kubernetes.io/arch`=amd64
 * `beta.kubernetes.io/os`=linux
 * `juju-application`=kubernetes-worker
 * `kubernetes.io/arch`=amd64
 * `kubernetes.io/hostname`=tiny2
 * `kubernetes.io/os`=linux
 * `series` is replaced with a fixed: `bundle: kubernetes` as the substrate/target is not an OS but k8s
 * `to` placement is replaced by `placement: foo=bar` where _foo=bar_ is a k8s node selector.    
   * Since I just want to target one host I can use the pre-existing label **kubernetes.io/hostname=big-boy**
   * I could also have used `cuda=true, gpu=true`


## Customize install with overlay

```yaml
name: Kubeflow lite 
description: customized to install on the one node with gpu which is also a k8s worker
bundle: kubernetes

applications:
  seldon-controller-manager:
    placement: kubernetes.io/hostname=big-boy
  dex-auth:    
    placement: kubernetes.io/hostname=big-boy
  mlmd:    
    placement: kubernetes.io/hostname=big-boy
  kfp-viz:    
    placement: kubernetes.io/hostname=big-boy
  istio-pilot:    
    placement: kubernetes.io/hostname=big-boy
  jupyter-ui:    
    placement: kubernetes.io/hostname=big-boy
  minio:    
    placement: kubernetes.io/hostname=big-boy
  tfjob-operator:    
    placement: kubernetes.io/hostname=big-boy
  kfp-persistence:    
    placement: kubernetes.io/hostname=big-boy
  argo-controller:    
    placement: kubernetes.io/hostname=big-boy
  kfp-db:    
    placement: kubernetes.io/hostname=big-boy
  kfp-schedwf:    
    placement: kubernetes.io/hostname=big-boy
  istio-ingressgateway:    
    placement: kubernetes.io/hostname=big-boy
  kubeflow-dashboard:    
    placement: kubernetes.io/hostname=big-boy
  kubeflow-volumes:
    placement: kubernetes.io/hostname=big-boy
  kfp-api:
    placement: kubernetes.io/hostname=big-boy
  kfp-ui:
    placement: kubernetes.io/hostname=big-boy
  kubeflow-profiles:
    placement: kubernetes.io/hostname=big-boy
  pytorch-operator:
    placement: kubernetes.io/hostname=big-boy
  kfp-viewer:
    placement: kubernetes.io/hostname=big-boy
  jupyter-controller:
    placement: kubernetes.io/hostname=big-boy
  admission-webhook:
    placement: kubernetes.io/hostname=big-boy
  oidc-gatekeeper:
    placement: kubernetes.io/hostname=big-boy  
```

## Install into kubeflow model

### Create custom kubeflow model

> 👉 a juju model created in k8s will automatically create a namespace with the same name. 
> All artifcats in that model get created under that namespace.
> **Note**: This must be under the juju k8s cloud controller, not the MAAS controller.

We install this into it's own model
 * Allows KF to be kept separate from core k8s and my app pods
 * As we add/remove stuff to/from KF, they keep going into that model
 * KF model can be torn down entirely and redone if needed.
 * Use the **kubeflow** model name. Worried something could be hardcoding it.


`juju add-model kubeflow` _this automatically makes it current, Otherwise do a `juju switch newModel`_

```console
vamsi@MAAS:~/bitbucket/infrastructure/k8s/nfs$ juju add-model kubeflow
Added 'kubeflow' model with credential 'myk8scloud' for user 'admin'
```

### Dry run deployment
```
vamsi@MAAS:~/bitbucket/infrastructure/configs/juju/bundles$ juju deploy -m kubeflow --dry-run ./charmed-kubeflow-lite-67-focal-bundle.yaml --overlay ./charmed-kubeflow-lite-67-focal-k8s-overlay.yaml
```

### Deploy

`juju deploy -m kubeflow ./charmed-kubeflow-lite-67-focal-bundle.yaml --overlay ./charmed-kubeflow-lite-67-focal-k8s-overlay.yaml --debug`

```
vamsi@MAAS:~/bitbucket/infrastructure/configs/juju/bundles$ juju deploy -m kubeflow ./charmed-kubeflow-lite-54-focal-bundle.yaml --overlay ./charmed-kubeflow-lite-54-focal-k8s-overlay.yaml --debug
```

To watch while deploying
 - on the juju side `watch -n5 --color juju status --color`
 - on the k8s side `watch -n5 --color kubectl get all -n kubeflow`

### Verify

```console
vamsi@MAAS:~/bitbucket/infrastructure/configs/juju/bundles$ kubectl get all -n kubeflow
NAME                                     READY   STATUS    RESTARTS   AGE
pod/admission-webhook-7bd59bb8d8-4m2qk   1/1     Running   0          77s
pod/admission-webhook-operator-0         1/1     Running   0          110s
pod/argo-controller-operator-0           1/1     Running   0          59s
pod/dex-auth-ffd749879-rkq2s             1/1     Running   0          31s
pod/dex-auth-operator-0                  1/1     Running   0          50s
pod/istio-ingressgateway-operator-0      1/1     Running   0          43s
pod/kfp-persistence-operator-0           1/1     Running   0          13s
pod/kfp-viz-operator-0                   1/1     Running   0          10s
pod/kubeflow-dashboard-operator-0        1/1     Running   0          6s
pod/kubeflow-profiles-operator-0         1/1     Running   0          4s
pod/kubeflow-volumes-operator-0          1/1     Running   0          2s
pod/modeloperator-555bc5cbb6-f2bf2       1/1     Running   0          11m

NAME                                    TYPE        CLUSTER-IP       EXTERNAL-IP   PORT(S)     AGE
service/admission-webhook               ClusterIP   10.152.183.27    <none>        443/TCP     98s
service/admission-webhook-operator      ClusterIP   10.152.183.236   <none>        30666/TCP   111s
service/argo-controller-operator        ClusterIP   10.152.183.125   <none>        30666/TCP   89s
service/dex-auth                        ClusterIP   10.152.183.222   <none>        5556/TCP    32s
service/dex-auth-operator               ClusterIP   10.152.183.172   <none>        30666/TCP   53s
service/istio-ingressgateway-operator   ClusterIP   10.152.183.81    <none>        30666/TCP   44s
service/kfp-persistence-operator        ClusterIP   10.152.183.65    <none>        30666/TCP   14s
service/kfp-viz-operator                ClusterIP   10.152.183.198   <none>        30666/TCP   12s
service/kubeflow-dashboard-operator     ClusterIP   10.152.183.66    <none>        30666/TCP   9s
service/kubeflow-profiles-operator      ClusterIP   10.152.183.152   <none>        30666/TCP   6s
service/kubeflow-volumes-operator       ClusterIP   10.152.183.171   <none>        30666/TCP   4s
service/minio-operator                  ClusterIP   10.152.183.38    <none>        30666/TCP   1s
service/modeloperator                   ClusterIP   10.152.183.83    <none>        17071/TCP   11m

NAME                                READY   UP-TO-DATE   AVAILABLE   AGE
deployment.apps/admission-webhook   1/1     1            1           77s
deployment.apps/dex-auth            1/1     1            1           31s
deployment.apps/modeloperator       1/1     1            1           11m

NAME                                           DESIRED   CURRENT   READY   AGE
replicaset.apps/admission-webhook-7bd59bb8d8   1         1         1       77s
replicaset.apps/dex-auth-ffd749879             1         1         1       31s
replicaset.apps/modeloperator-555bc5cbb6       1         1         1       11m

NAME                                             READY   AGE
statefulset.apps/admission-webhook-operator      1/1     110s
statefulset.apps/argo-controller-operator        1/1     80s
statefulset.apps/dex-auth-operator               1/1     50s
statefulset.apps/istio-ingressgateway-operator   1/1     44s
statefulset.apps/kfp-persistence-operator        1/1     13s
statefulset.apps/kfp-viz-operator                1/1     10s
statefulset.apps/kubeflow-dashboard-operator     1/1     7s
statefulset.apps/kubeflow-profiles-operator      1/1     5s
statefulset.apps/kubeflow-volumes-operator       1/1     2s
```
### Istio setup

 Once completed, the `istio-ingressgateway/0` unit remains waiting. The operator needs to be authorized to talk to the api service (per https://charmed-kubeflow.io/docs/install)

 * Grant istio operator API permissions: `kubectl patch role -n kubeflow istio-ingressgateway-operator -p '{"apiVersion":"rbac.authorization.k8s.io/v1","kind":"Role","metadata":{"name":"istio-ingressgateway-operator"},"rules":[{"apiGroups":["*"],"resources":["*"],"verbs":["*"]}]}'`

 Now if you do a `kubectl get service/istio-ingressgateway` you'll see that the ExternaIP is listed as `<pending>`. At this point, we need to add metallb so it can provide it an external ip from a pool. 
 
 _Alternately, we patch `istio-ingressgateway` to be a `NodePort` and then simply access it using the node's IP at the specified port_.

# Configure Kubeflow dashboard for external access

 This seems to have many complex pieces that need to come together. Took a combination of these reseources to get things fully (_actual understanding of the concepts not there yet_)
  - https://charmed-kubeflow.io/docs/install
    - Which points to https://www.youtube.com/watch?v=M_Ama8ZY8OQ _pointed to from the above link_
  - https://github.com/kubeflow/manifests/issues/974
    - _Thomas Albrecht: a simpler way (instead of loadbalancer) could be Nodeports. Instead of setting up a Loadbalancer, the only thing you have to do is, patching ingres-gateway to NodePort instead of LoadBalancer_ 
  - https://www.kubeflow.org/docs/distributions/microk8s/kubeflow-on-microk8s/ 
  - 👉 These articles by Nana clarified a whole bunch of things
    - [Kubernetes ingress](https://www.youtube.com/watch?v=80Ew_fsV4rM&t=1s)
    - [Kubernetes service types](https://www.youtube.com/watch?v=T4Z7visMM4E)
      - And now I understand what Albrecht was saying about NodePort.

## Export istio-ingress

Looks like `kubectl-dashboard` itself shuold not be exposed. There is this whole ecosystem of securiy, ingress and other stuff I do not understand yet.
 - istio-ingress
 - dex
 - oidc

 ### Metallb prereqs

 For a service of type `LoadBalancer`, we need a LoadBalancer component installed (_in the same namesapce?_) which will provide an IP
  - In my case, reserve a block of IPs from DHCP `[192.168.23.10 - 192.168.23.99]` (_see how the MAAS subnets are setup_)
  - All of these are routable from inside my network. `MAAS Master` is router and they will reach all the MAAS nodes. At the nodes, since this is a `LoadBalancer` service which layers on NodePort: `istio-ingressgateway` listens on every node. As long as request incoming via one of the LoadBalancer IPs reaches the node, it will reach the istio-ingressgateway service eventually.

 * create a juju model (_k8ds namespace_) called *metallb-system* which is what all the examples yaml files assume. Simplest to keep it that way. `juju add-model metallb-system` 
 * `wget https://raw.githubusercontent.com/charmed-kubernetes/metallb-operator/master/docs/rbac-permissions-operators.yaml`
 *  `kubectl apply -f rbac-permissions-operators.yaml`
 
### Deploy metallb

 Create a metallb-overlay.yaml file with the following contents
 ```yaml
 applications:
  metallb-controller:
    options:
      iprange: "192.168.23.10-192.168.23.99"
 ```
 
 * `cd ~/bitbucket/infrastructure/configs/juju/bundles`
 * `juju deploy cs:~containers/metallb --overlay ./metallb-overlay.yaml` (_see https://jaas.ai/u/containers/metallb/bundle_)
 * `watch -n5 --color juju status --color`

 *Test*: `kubectl get services --all-namespace | grep istio` shows 192.168.23.10 as the external IP for the istio-ingressgateway. Yay!

### Security setup

To finalize (_all these UI compoents use dex. Not sure what oidc is yet_), the security guys should be told what IP the services should be reachable at and a basic auth setup. Take the ip from the `External IP` field of `kubectl get service/istio-ingressgateway -n kubeflow`.

 - `juju config dex-auth -m kubeflow public-url=http://192.168.23.10`
 - `juju config oidc-gatekeeper -m kubeflow public-url=http://192.168.23.10`
 - `juju config dex-auth -m kubeflow static-username=admin`
 - `juju config dex-auth -m kubeflow static-password=letmein`

 ### Test exposure

 Now hitting http://192.168.23.10 takes me to the kubeflow dashboard
  - Give this some time. dex-auth's name/pwd take a while to flow through. Several minutes looks like
  - If you get auth failed, try again after 5 minutes



## Exploration

Just to see where kubefow-dashboard integrates in, did a `kubectl -n kubeflow describe service/istio-ingressgateway` and got the following

```console
vamsi@MAAS:~$ kubectl describe -n kubeflow service/istio-ingressgateway
Name:                     istio-ingressgateway
Namespace:                kubeflow
Labels:                   app.kubernetes.io/managed-by=juju
                          app.kubernetes.io/name=istio-ingressgateway
Annotations:              controller.juju.is/id: b34b9e53-7916-43e6-8779-20206d16a185
                          model.juju.is/id: 664d9110-8a64-40ad-8de3-4d5325cc76b7
Selector:                 app.kubernetes.io/name=istio-ingressgateway
Type:                     LoadBalancer
IP Family Policy:         SingleStack
IP Families:              IPv4
IP:                       10.152.183.143
IPs:                      10.152.183.143
LoadBalancer Ingress:     192.168.23.150
Port:                     status-port  15020/TCP
TargetPort:               15020/TCP
NodePort:                 status-port  32709/TCP
Endpoints:                10.1.50.81:15020
Port:                     http2  80/TCP
TargetPort:               80/TCP
NodePort:                 http2  31507/TCP
Endpoints:                10.1.50.81:80
Port:                     https  443/TCP
TargetPort:               443/TCP
NodePort:                 https  32021/TCP
Endpoints:                10.1.50.81:443
Port:                     kiali  15029/TCP
TargetPort:               15029/TCP
NodePort:                 kiali  31052/TCP
Endpoints:                10.1.50.81:15029
Port:                     prometheus  15030/TCP
TargetPort:               15030/TCP
NodePort:                 prometheus  30350/TCP
Endpoints:                10.1.50.81:15030
Port:                     grafana  15031/TCP
TargetPort:               15031/TCP
NodePort:                 grafana  31992/TCP
Endpoints:                10.1.50.81:15031
Port:                     tracing  15032/TCP
TargetPort:               15032/TCP
NodePort:                 tracing  32710/TCP
Endpoints:                10.1.50.81:15032
Port:                     tls  15443/TCP
TargetPort:               15443/TCP
NodePort:                 tls  30695/TCP
Endpoints:                10.1.50.81:15443
Port:                     pilot  15011/TCP
TargetPort:               15011/TCP
NodePort:                 pilot  30415/TCP
Endpoints:                10.1.50.81:15011
Port:                     citadel  8060/TCP
TargetPort:               8060/TCP
NodePort:                 citadel  32110/TCP
Endpoints:                10.1.50.81:8060
Port:                     dns-tls  853/TCP
TargetPort:               853/TCP
NodePort:                 dns-tls  32637/TCP
Endpoints:                10.1.50.81:853
Session Affinity:         None
External Traffic Policy:  Cluster
Events:                   <none>
```
 
 * Don't see any direct kubectl dashboard. Must be all related to http/https in that case
  

# History

## Failed attemps

Tried installing it into the `k8s-model` model but it bombed right away with 'storage configuration' and I had to go back to figuring out to add storage to juju. I either did not see or grok [Setting up static storage with kubernetes](https://discourse.charmhub.io/t/setting-up-static-kubernetes-storage-tutorial/1193) and so started down a rabbit hole of figuring out nfs storage with charms. Wasted three months I think overall! Gah!!

how I could use a charm that worked off my NAS.

The `nfs` charm allows you to create any cluster machine into a NFS server. However, my cluster has a bunch of throwaway machines with limited storage expansion: did not want to use those as NFS servers. *In hind sight, if I had, I would be much further along and would have had to use my NAS only when storage needs required it. One more way, the lack of a proper startup mindset is hurting me*.


**Notes**

- Need to add k8s-storage explicitly first
- Then add `juju add-k8s`

## Overlays are different

- Cannot use `placement: 0` switched to `to: ['0']` instead
- *series not valid*: removed the *series : focal*, guess it depends on the kubernetes runtime not the ubuntu version.
- `machines` not valid for kubernetes bundles. Looks like unlike plain juju, kubernetes bundles need their deployment to be handled via node-pools. Do I really care that these get installed on all the kubernetes machines ? Maybe

 ✅️ See [Bundle Reference](https://juju.is/docs/sdk/bundle-reference) and adjust overlay correctly

## Configure Kubeflow dashboard for external access

This was tricky to figure out. Online examples showed plenty of things to try, the details of which I forget now. They involved exposing kubeflow-dashboard directly. kubectl shows that kubeflow-dashboard is serving at 8082
 - kubectl forward-port: `kubectl port-forward -n kubeflow-model service/kubeflow-dashboard 8080:8082` and then look at localhost:8080
 - kubectl proxy

```console
kubectl port-forward -n kubeflow-model service/kubeflow-dashboard 8080:8082

> Forwarding from 127.0.0.1:8080 -> 8082
> Forwarding from [::1]:8080 -> 8082
```

 None of these worked. Exposing via port-forward let me see the app but links inside were broken.

 ## Portforwarding

 Some of the docs use a metallb IP range which is not directly on the machine (they use .xip.io wirldcard DNS). In this case, they suggest

 - `ssh -D9999 ubuntu@node` and leaving it on. Then update networking to setup socks at this por on localhost. This performs SOCKS proxying (all networking on the machine gets tunneled over that port to `node`)
 - Alternately, avoid the whole machine proxying and just setup proxying in firefox
 - Using a naturally reachable set of IPs, I did not need any socks proxying.