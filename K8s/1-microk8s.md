# MicroK8s
<!-- TOC -->

- [MicroK8s](#microk8s)
- [Microk8s Resources](#microk8s-resources)
- [Motivation](#motivation)
- [Setup](#setup)
    - [Check versions](#check-versions)
    - [Install microk8s](#install-microk8s)
    - [Configure firewall](#configure-firewall)
    - [Save cluster config information](#save-cluster-config-information)
    - [Add user to group](#add-user-to-group)
    - [Setup aliases](#setup-aliases)
    - [Enable addons](#enable-addons)
        - [List addons](#list-addons)
        - [Which ones do I want](#which-ones-do-i-want)
    - [Kubeflow](#kubeflow)
        - [Earlier problems](#earlier-problems)
        - [Jupyter problem](#jupyter-problem)

<!-- /TOC -->

# Microk8s Resources

* [Main webpage](https://ubuntu.com/tutorials/install-a-local-kubernetes-with-microk8s)
* [Launch Configurations](https://microk8s.io/docs/ref-launch-config)

# Motivation 

I had initially started with full fat kubernetes as I thought I simply had to replicate a HA system at home. The goal was to be able to seamlessly move to a k8s platform on GKE for instance.

I did spend an inordinate amount of time getting started with k8s and still stuck on kubeflow. More recently _Aug 2021_ I was wondering how to handle linux development and how I could reuse my big 32 core system for that and all that. Turns out I can simply use that threadripper system as-is
 * Directly connect to my network
 * no MAAS, subnets etc
 * Login directly as a dev box
 * Use microk8s with storage, gpu, kf for all the experiments. 32 cores should be plenty and I can always add more RAM.


# Setup

 Follow instructions from [official Ubuntu docs](https://ubuntu.com/tutorials/install-a-local-kubernetes-with-microk8s)

## Check versions

```shell
vamsi@tr32:~/Documents/bitbucket$ snap info microk8s
name:      microk8s
summary:   Lightweight Kubernetes for workstations and appliances
publisher: Canonical✓
store-url: https://snapcraft.io/microk8s
contact:   https://github.com/ubuntu/microk8s
license:   Apache-2.0
description: |
  MicroK8s is the smallest, simplest, pure production Kubernetes for
  clusters, laptops, IoT and Edge, on Intel and ARM. One command installs a
  single-node K8s cluster with carefully selected add-ons on Linux, Windows
  and macOS.  MicroK8s requires no configuration, supports automatic updates
  and GPU acceleration. Use it for offline development, prototyping, testing,
  to build your CI/CD pipeline or your IoT apps.
snap-id: EaXqgt1lyCaxKaQCU349mlodBkDCXRcg
channels:
  1.21/stable:      v1.21.3  2021-07-27 (2346) 191MB classic
  1.21/candidate:   v1.21.4  2021-08-20 (2407) 191MB classic
  1.21/beta:        v1.21.4  2021-08-20 (2407) 191MB classic
  1.21/edge:        v1.21.4  2021-08-23 (2427) 191MB classic
  latest/stable:    v1.21.3  2021-07-28 (2346) 191MB classic
  latest/candidate: v1.22.1  2021-08-20 (2424) 195MB classic
 ```

## Install microk8s

 `sudo snap install microk8s --classic`

 Ran very fast! Few seconds.

## Configure firewall

Not sure if this is needed but doing it anyway

`sudo ufw allow in on cni0 && sudo ufw allow out on cni0`
`sudo ufw default allow routed`

## Save cluster config information

There are many misc tool clis that are used to work with a k8ds cluster _(kubectl, helm, argocd etc)_ and all of these need to know how to contact the cluster. By convention, this information is stored in `~/.kube/config`.

For a newly created cluster, we need to generate this file. Do this via

```bash
mkdir  ~/.kube
microk8s config > ~/.kube/config
```

## Add user to group

 Some commands (_specifically `microk8s enable kubeflow`_) cannot be run as root or as regular user. Do this instead and then we can run as regular user.

 * `sudo usermod -a -G microk8s vamsi`
 * `sudo chown -f -R vamsi ~/.kube`
 * `newgrp microk8s` to load the new info (or reboot)

## Setup aliases

There are two ways of controlling/querying this microk8s cluster.
 - Use the builtin `kubectl` via `microk8s kubectl`
 - Use a separately installed `kubectl`
   - This needs to know how to communicaz=te with the cluster. This is done via the `~/.kube/config` file that was created in the earlier steps

Save this in `~/.bashrc` or `~/.bash_aliases`

```shell
alias kubectl='microk8s kubectl'
alias ktl='microk8s kubectl'
alias mk8s=microk8s
```

## Enable addons

### List addons

Note that this list keeps changing. The following is true as of August 2023

`microk8s status`

```shell
vamsi@mk8s:~$ mk8s version
MicroK8s v1.27.4 revision 5643

vamsi@mk8s:~$ mk8s status
microk8s is running
high-availability: no
  datastore master nodes: 127.0.0.1:19001
  datastore standby nodes: none
addons:
  enabled:
    dns                  # (core) CoreDNS
    ha-cluster           # (core) Configure high availability on the current node
    helm                 # (core) Helm - the package manager for Kubernetes
    helm3                # (core) Helm 3 - the package manager for Kubernetes
  disabled:
    cert-manager         # (core) Cloud native certificate management
    community            # (core) The community addons repository
    dashboard            # (core) The Kubernetes dashboard
    gpu                  # (core) Automatic enablement of Nvidia CUDA
    host-access          # (core) Allow Pods connecting to Host services smoothly
    hostpath-storage     # (core) Storage class; allocates storage from host directory
    ingress              # (core) Ingress controller for external access
    kube-ovn             # (core) An advanced network fabric for Kubernetes
    mayastor             # (core) OpenEBS MayaStor
    metallb              # (core) Loadbalancer for your Kubernetes cluster
    metrics-server       # (core) K8s Metrics Server for API access to service metrics
    minio                # (core) MinIO object storage
    observability        # (core) A lightweight observability stack for logs, traces and metrics
    prometheus           # (core) Prometheus operator for monitoring and logging
    rbac                 # (core) Role-Based Access Control for authorisation
    registry             # (core) Private image registry exposed on localhost:32000
    storage              # (core) Alias to hostpath-storage add-on, deprecated
```

Compared to the last time I ran this maybe 2 years ago.
 - cilium moved to a higher level `mk8s cilium`
 - openebs replaced with `mayastor` ?
 - storage is deprecated and replaced with `host-access`
 - Following were deleted                  
   - `kubeflow`    # Kubeflow for easy ML deployments        
  - Following are moved to _community addon_ which is enabled by `microk8s enable community`
    - `ambassador` # Ambassador API Gateway and Ingress
    - `cilium`     # SDN, fast with full network policy    
    - `fluentd`    # Elasticsearch-Fluentd-Kibana logging and monitoring        
    - `istio`      # Core Istio service mesh services
    - `jaeger`     # Kubernetes Jaeger operator with its simple config
    - `keda`       # Kubernetes-based Event Driven Autoscaling
    - `knative`    # The Knative framework on Kubernetes.
    - `linkerd`    # Linkerd is a service mesh for Kubernetes and other frameworks
    - `openebs`    # OpenEBS is the open-source storage solution for Kubernetes
    - `openfaas`   # openfaas serverless framework
    - `portainer`  # Portainer UI for your Kubernetes cluster    
    - `traefik`    # traefik Ingress controller for external access
    - `multus`     # Multus CNI enables attaching multiple network interfaces to pods                
  - Following new, interesting ones are in community
    - `argocd` _argocd_
    - `gopaddle-lite` _Simple “no-code” platform for Kubernetes developers._
    - `kwasm` _Add WebAssembly support to your Kubernetes nodes_    
    - `parking` _Parking for static sites_
    - `trivy` _Open source security scanner for Kubernetes_
    - `nfs` _Add nfs based storage_
    - `microrocks` _Cloud native mocking and testing_
    


### Which ones do I want

  |*Addon*|*Status*|
  | ----- | -----|
  | dashboard| Overall dashboard. Likely always need it|
  | dns      | needed for pod to pod comm ?|
  | gpu      | needed for ML tasks |
  | ~~ingress~~  | Need to talk from outside. The other options here are `Ambassador`, `Traefik` and even `istio`. However ([./FAQ-Kubernetes](./FAQ-Kubernetes.md)) looks like *ingress* is plenty for me since all i care about at this point is `gRPC` and `http(s)`|
  | istio    | The service mesh that also does ingress. Kubeflow needs this anyway so might as well use this instead of the standard `ingress` |
  | kubeflow | The ML framework |
  | storage  | expose host disks |
  | registry | private docker registry to load my builds. Esposed on `localhost:32000`|
  | prometheus | Monitoring and logging|
  | openfaas | Need to explore !|

 > looks like I need to install things in a specific order for kubeflow to work properly

 * `microk8s enable storage dns gpu`
 * `microk8s status --wait-ready`
 * `microk8s enable registry`
 * `microk8s status --wait-ready`
 * `microk8s enable istio`
 * ~~`microk8s status --wait-ready`~~
 * ~~`microk8s enable kubeflow`~~ _No longer using. Too resource intensive. Using kserve (previously KFServing) instead for serving_

 ## Kubeflow

 ```console
 The dashboard is available at http://10.64.140.44.nip.io

    Username: admin
    Password: XWN43SYWQM50653KH84R3V5VTPXWWJ

To see these values again, run:

    microk8s juju config dex-auth static-username
    microk8s juju config dex-auth static-password
```

### Earlier problems

Now, if I visit http://10.64.140.44.nip.io and supply the creds (_using admin when asked for email_), I get an *Access denied* error. Searching on the net leads me to [this](https://stackoverflow.com/questions/67805796/microk8s-kubeflow-dashboard-access-failed-to-exchange-authorization-code-with) from which I did a 
 * `microk8s kubectl get services -n kubeflow`
 * saw 10.152.183.238:8082 for kubeflow-dashboard and when I try that, I immediately get to the dashboard with no authentication. Maybe the other IP only works from outside ? Who does the DNS then ?

 Turns out that the installation needs to be done in a specific way. I have redone it with the updated instructions above. Basically, dns and istio first and wait for it before kubeflow.

 ### Jupyter problem

When using 1.21/stable, I see some notebook server problems: If I hit the `Create new notebook server`, I get a *The URL was not found on the server* error.

  I re-installed multiple times
   * without registry
   * with nothing, just kubeflow
   * with everything else (- registry) and then kubeflow


1.20/stable also has the same problems

   None of them worked. I continue to get the same error with notebook servers. Looks like this microk8s is on top of juju and other layers, maybe simplest to simply go back to full k8s and just deal with the multiple machines!