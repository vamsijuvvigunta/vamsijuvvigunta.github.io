
<!-- TOC -->

- [Dec 2020 - Juju storage](#dec-2020---juju-storage)
- [Nov 2020 - Kubeflow](#nov-2020---kubeflow)
    - [Juju Deployment](#juju-deployment)
        - [Charmed Kubernets](#charmed-kubernets)
        - [Kubeflow](#kubeflow)
- [Nov 2020 - VLANS](#nov-2020---vlans)
- [July to Nov 2020](#july-to-nov-2020)
- [Feb - July 2020](#feb---july-2020)
- [Start](#start)

<!-- /TOC -->

A progress log to track the evolution of the infrastucture and it's decisions. Reverse chronological order.

# Feb 2021 - Kubeflow installation restarted

 Started playing with a --dryrun and realized a bunch of things
  - overlay cannot have normal charm like --to
  - needs special kuberetes node selectors
  - messed up big-boy because I wanted the 10G port but it was comissioned on the 2.5G port so need to redo it again anyway
  - also, don't think ubuntu automatically installed nvidia drivers when I installed k8s.
  - so restart

cleanup juju
- `juju destroy-model kds-model` empty for now so simple
- `juju destroy-model k8s`: the main model. This should free up all the MAAS nodes. Since I'll be redoing them, no need to destroy-storage.
- `juju destroy-controller --all-models --remove-storage`

Move to non-beta MAAS

- Then `sudo snap refresh --channel=2.9/stable maas`
- This did not work as expected (althought might has been other problems not sure) so removed and reinstalled maas. Worked smoothly enough this time.

Redo K8s

- Reinstaled k8s and everything worked ok
- Checked automatic GPU integration. Had to fiddle with BIOS and reboot to get it. Then figure out that X was up and needed to disable all sleep to keep machine awake. Thought the power supply was dying but `last` showed no crashes 😌. Turned out I needed to disable all sleep modes.

# Feb 2021 - Juju storage completed
  
- Examine the kubernetes-worker charm to see how it uses nfs
  - Good thing is that [Kubernetes storage](./4-juju-k8s-storage.md) seems to be accepted by juju when adding other k8s stuff and so kubeflow might just work and 
  allow me to proceed.
  - Still examine kubernetes-worker charm and see how it sets up storage with the nfs charm.

Yup. Right there in kubernetes-worker's `metadata.yaml` file

```yaml
requires:
  ...
  nfs:
    interface: mount
```

Reactive code looks for `fstype=nfs`. Blindly assuming semantic equivalence to linux mount-type and using *nfs4* was obviously wrong.

Fixes
- provide `nfs` relation
- explicitly relate to `remote-nfs-mount:nfs` to make it clear that the app is remote-nfs-mount.

[Setting up static storage with kubernetes](https://discourse.charmhub.io/t/setting-up-static-kubernetes-storage-tutorial/1193) was the final piece. **If I had been patient with understanding this, I could have supplied storage off my NAS without getting into this charm business. Oh well!

See final form of . I modified some of the instructions from [Setting up static storage with kubernetes](https://discourse.charmhub.io/t/setting-up-static-kubernetes-storage-tutorial/1193) to setup nfs backed by the NFS mount. My charm simply provides mount points but the core k8s integration is done by the kubernetes-worker charm.

Alternately, I could have
- Used the `remote-nfs` to ensure a local mount point on all nodes for the NFS
- Setup static PVs like in [Setting up static storage with kubernetes](https://discourse.charmhub.io/t/setting-up-static-kubernetes-storage-tutorial/1193)
- Setup juju storageclass and pools based on those. Maybe.

Anyway, juju/k8s storage aspects seem to be taken care of.

# Dec 2020 - Juju storage

The juju storage docs imply I need lot more fundas to figure things out. Maybe simpler to just target kubernetes storage
- created a new `remote-nfs-mount` charm by copying from `nfs` charm. Quite a learning experience.
  - Got deployed charm working
  - Got remote-nfs-charm related to kubernets-worker and I see a new export being created.
  - However. This does not create a new storage-class in juju.
    - I used a new remote-nfs-mount end-point. Maybe that is the problem, simply having the mount interface might not help

# Nov 2020 - Kubeflow

Since I am reinstalling everything on Ubuntu 20.04 and MAAS 2.9rc1, time for a rethink regd the k8s deployment. Since both k8s and kubeflow have demos where they run on a single workstation, I should be ok with stuffing a lot of things into the m93p tiny PCs.

Also, turns out there is a much simpler K3s. However

- No juju charm is the killer. Or is it ?
- Given that I can stuff k8s onto a small machine, park this till there is a juju charm.

Potential deployment. Mainly

- Put non POD related items on the small m93ps or nucs
- Put GPU and POD related items on the 32 core beast

## Juju Deployment

Given the 3 machines I have, to have something mostly stable

  * 2x m93p mostly unchanging admin type items
  * 1 Worker for GPU and other tasks
  * Doubt I need any more workers but can likely add 1 or more workers and if needed 1 or more master nodes later on. In any case reconfiguring is not too hard as long as my MAAS node remains untouched.

in terms of components (*units of deployment charms*), I am likely looking at the following

### Charmed Kubernets

From [this bundle: 1010](https://jaas.ai/canonical-kubernetes)

- m93p (*tags: k8, k8_admin*)
  - kubernetes master
  - kubeapi load balancer
  - etcd
  - flannel
  - easyrsa
  - containerd
- m93p (*tags: k8, kf_admin*)
  - containerd
  - flannel
  - kubernetes worker  
- big boy (*tags: k8, worker, gpu*)
  - containerd
  - flannel
  - kubernetes worker  

  Previously (eary 2020) had used the approach from https://medium.com/@madushan1000/how-to-run-kubernetes-on-bare-metal-with-maas-juju-d5ba8e981710 which directly modified the bundle file. Now I am **using overlays** to retain the original bundle file but apply modifications via a separate file. Worked great once some quirks were ironed out.

### Kubeflow

  * [Kubeflow Lite bundle](https://jaas.ai/kubeflow-lite/bundle/17) which seems reasonable 
  * [Kubeflow](https://jaas.ai/kubeflow/bundle/230) which is more standard 
    * Both has tf-serving missing but there is a [charm for that](https://github.com/juju-solutions/charm-tf-serving)

- m93p (*tags: k8, k8_admin*)
  - k8s master
  - kubeapi-load-balancer
  - etcd
  - easyrsa
  - api-server
  - kube scheduler
- m93p (*tags: k8, kf_admin*)
  - kubelet
  - kube_proxy
  - some KF
- big boy (*tags: k8, worker, gpu*)
  - kubelet
  - kube_proxy
  - KF
    - Jupyter
    - PyTorch operator
    - TF Serving

However, juju refused to install kubeflow saying that storage was not configured! Taking a detour to figure out storage.

# Nov 2020 - VLANS 

 * Realized NAS backups were failing and took a bunch of time for upgrades
   * New NAS: 626X with dual 10Gbe
   * New Ubiquiti X16G 10G switch (4 rj45, rest SFP+)
   * New disks all over for the two NASs   
   * New APC UPS (sine wave this time)
 * Re did the rack to make more space
 * Removed the extra switch used for MAAS and moved everything to the main Ubiquiti Switches on a different VLAN. Big boy connected via 10G Nic.
 * Details at [tools/networking](../tools/Networking.md)

Now I can work on the storage for k8s, kubeflow and all that. Or so I thought. Turns out everything broke with VLANs. 

   * ping woud work from within but all DNS were broken (need dns resolution for the machines to work with maas)
   * reinstalled MAAS with 20.04 and latest MAAS 2.9 and VLAN support from scratch
     * MAAS machine with VLANs on the main MAAS interface (bridged)
     * MAAS configured to provide DHCP on the VLAN

  Nothing but problems so gave up. Will buy another 10G switch in the future if I really need 10G and switched back to older config 

    * Separate switch for MAAS
    * No VLANS

# July to Nov 2020

 * Got the APC PDU and now I have a network controllable start/stop for the MAAS boxes. [tools/APC_PDU](../tools/APC_PDU.md)
 * Stuck on work projects
 * Pondering how to use the MAAS/k8s infra to also handle regular ML development and experimentation
   * Can use the `i7 + 2080Ti` to do this but it will be entirely independent and any serving I do will be separate. However, ok for scala development etc.
   * Discovered Kubeflow and that looks ideal for me. 
     * An environment within K8s which includes jupyter lab
     * Specify storage on NFS (exported from the NAS) for k8s and use that 

# Feb - July 2020
  * Settled on MAAS
  * New hardware
    * Two used Skullcanyon i7/16Gb Nucs
    * Two m93P thinkcenter i7/16GB 
    * 1 Silver i5 Nuc as MAAS box.
    * Beastly 32core AMD Threadripper with 64GB and 1TB SSDs

Took a good while to figure all of this out and get them all working.
 * [1 - MAAS Setup](1-maas-onprem-cloud.md)
 * [2 - Juju on MAAS](2-juju.md) - juju controller is on a VM on the MAAS node and setup to start with the host (takes 1 core)
 * [3 - Kubernetes with Juju](3-juju-k8s.md)


# Start

Started this in earnest after I spoke with Dha in Feb 2020. Right before covid struck. His advice (*turned out to be true*) was to stick to google cloud and go from there. However,
 * Not knowing anything about this, I thought I should have my own hardware and learn. Otherwise, during crunch time, I will be forced to learn anyway.
