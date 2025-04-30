<!-- TOC -->

- [Problem Log](#problem-log)
- [remote-nfs install fails](#remote-nfs-install-fails)
    - [nfs-commont apt package](#nfs-commont-apt-package)
    - [Logs](#logs)
    - [Errors](#errors)
    - [Solution](#solution)

<!-- /TOC -->

# Problem Log 

A list of the problems encountered so that even when I update the infra steps with updates, I still keep the problems and investigations around for references

# remote-nfs install fails

<pre><font color="#4E9A06"><b>vamsi@MAAS</b></font>:<font color="#3465A4"><b>~</b></font>$ juju status
Model  Controller       Cloud/Region     Version  SLA          Timestamp
k8s    juju-controller  devmaas/default  2.8.6    unsupported  10:41:55-08:00

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

**Now connect to kubernetes**

- Example show a relation to `kubernetes-worker` not sure if the master also needs it.

<pre><font color="#4E9A06"><b>vamsi@MAAS</b></font>:<font color="#3465A4"><b>~</b></font>$ juju add-relation kubernetes-worker remote-nfs
</pre>


Check if it worked!

<pre><font color="#4E9A06"><b>vamsi@MAAS</b></font>:<font color="#3465A4"><b>~</b></font>$ juju status
Model  Controller       Cloud/Region     Version  SLA          Timestamp
k8s    juju-controller  devmaas/default  2.8.6    unsupported  10:44:48-08:00

App                    Version  Status  Scale  Charm                  Store       Rev  OS      Notes
containerd             1.3.3    <font color="#4E9A06">active      </font>3  containerd             jujucharms   96  ubuntu  
easyrsa                3.0.1    <font color="#4E9A06">active      </font>1  easyrsa                jujucharms  338  ubuntu  
etcd                   3.4.5    <font color="#4E9A06">active      </font>1  etcd                   jujucharms  543  ubuntu  
flannel                0.11.0   <font color="#4E9A06">active      </font>3  flannel                jujucharms  512  ubuntu  
kubeapi-load-balancer  1.18.0   <font color="#4E9A06">active      </font>1  kubeapi-load-balancer  jujucharms  752  ubuntu  exposed
kubernetes-master      1.19.4   <font color="#4E9A06">active      </font>1  kubernetes-master      jujucharms  908  ubuntu  
kubernetes-worker      1.19.4   <font color="#4E9A06">active      </font>2  kubernetes-worker      jujucharms  711  ubuntu  exposed
remote-nfs                      <font color="#CC0000">error       </font>2  remote-nfs             jujucharms    2  ubuntu  

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
  remote-nfs/1            <font color="#CC0000">error     </font><font color="#4E9A06">idle   </font>         192.168.23.104                  hook failed: &quot;install&quot;
kubernetes-worker/1       <font color="#4E9A06">active    idle   </font>2        192.168.23.105  80/tcp,443/tcp  Kubernetes worker running.
  containerd/0*           <font color="#4E9A06">active    idle   </font>         192.168.23.105                  Container runtime available
  flannel/0*              <font color="#4E9A06">active    idle   </font>         192.168.23.105                  Flannel subnet 10.1.22.1/24
  remote-nfs/0*           <font color="#CC0000">error     </font><font color="#4E9A06">idle   </font>         192.168.23.105                  hook failed: &quot;install&quot;

Machine  State    DNS             Inst id  Series  AZ       Message
0        <font color="#4E9A06">started  </font>192.168.23.103  tiny1    focal   default  Deployed
1        <font color="#4E9A06">started  </font>192.168.23.104  tiny2    focal   default  Deployed
2        <font color="#4E9A06">started  </font>192.168.23.105  big-boy  focal   default  Deployed
</pre>

## nfs-commont apt package

hmm, maybe needs `focal` or `nfs-common` on all clients!

Try again..

```console
vamsi@MAAS:~$ juju remove-relation kubernetes-worker remote-nfs
vamsi@MAAS:~$ juju remove-application --force remote-nfs
```

See if asking for the nfs-common package works

```console
vamsi@MAAS:~$ juju deploy -m k8s cs:~chris.macnaughton/remote-nfs-2 --config "nfs-server=vamsi-nas" --config "nfs-target=/data/K8s" --config "local-target=/mnt/cluster-io" --config "extra_packages=nfs-common"
vamsi@MAAS:~$ juju add-relation kubernetes-worker remote-nfs
```

Still says the same thing *Error: hook failed: "install"*

## Logs 

Per more online resources, started looking at juju logs. Very near how it allows all of this.

`juju debug-log`

> I could have used `juju debug-log --tail=true remote-nfs` before installing as well.

Per more online resources, went to the machine and looked at `/var/log/juju/` and found `unit-remote-nfs-3.log`. In it, from the ERRORS

- testresources missing (`dependency of launchpadlib`)
- bunch of warnings

Turns out the warnings were about a missing attribute on a python object. This would have definitely been an error.

## Errors

The errors were about a missing attribute `linux_distribution` on module `platform`

Turns out this was deprecated in py3.7 and removed in 3.8. 3.8 is what juju on fossa uses.

- No reference in the git source for the `remote-nfs` project.
- Downloaded the entire charm and grepped: no luck.
- kept digging and found a `wheelhouse\charmhelpers-0.19.4.tar.gz`

When I unzipped `wheelhouse\charmhelpers-0.19.4.tar.gz`, I found a reference to `linux_distribution`

## Solution

After some more digging around, instead of figuring out how to upgrade the charmhelpers packages, I simply replaced the `charmhelpers\osplatform.py` file with the latest charmhelpers one on github (20.10 I think) which fixed the issue and repackaged the `wheelhouse\charmhelpers-0.19.4.tar.gz` archive.

After this, install the charm from the on-disk charm `~/charms/remote-nfs`. See [Installing offline charms](https://juju.is/docs/deploying-charms-offline)

> Looking at the charm source, I see that it installs nfs-common. 

```console
vamsi@MAAS:~$ juju deploy -m k8s --config "nfs-server=vamsi-nas" --config "nfs-target=/data/K8s" --config "local-target=/mnt/cluster-io" ~/charms/remote-nfs

vamsi@MAAS:~$ juju add-relation kubernetes-worker remote-nfs
```

Now everything works! You can see the status show the nfs mounts.

<pre>kubernetes-worker/1       <font color="#4E9A06">active    idle   </font>2        192.168.23.105  80/tcp,443/tcp  Kubernetes worker running.
  containerd/0*           <font color="#4E9A06">active    idle   </font>         192.168.23.105                  Container runtime available
  flannel/0*              <font color="#4E9A06">active    idle   </font>         192.168.23.105                  Flannel subnet 10.1.22.1/24
  remote-nfs/4*           <font color="#4E9A06">active    idle   </font>         192.168.23.105                  vamsi-nas:/data/K8s -&gt; /mnt/cluster-io
</pre>

