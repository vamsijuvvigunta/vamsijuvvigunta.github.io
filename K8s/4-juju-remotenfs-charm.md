<!-- TOC -->

- [Remote nfs mount charm](#remote-nfs-mount-charm)
    - [Resources](#resources)
    - [Sequencing flow via reactive calls](#sequencing-flow-via-reactive-calls)
    - [Remote NFS Mount charm flow](#remote-nfs-mount-charm-flow)
    - [Build charm locally](#build-charm-locally)
    - [Deploy local charm](#deploy-local-charm)
    - [Relate to consumer K8s charm](#relate-to-consumer-k8s-charm)
    - [Resulting K8s storageclass](#resulting-k8s-storageclass)
    - [Create juju storage class and pool from K8s](#create-juju-storage-class-and-pool-from-k8s)
    - [TODO](#todo)
- [Charm development log](#charm-development-log)
    - [Create charm skeleton](#create-charm-skeleton)
    - [Add charm content](#add-charm-content)
        - [layers](#layers)
        - [Metadata](#metadata)
        - [Reactive code](#reactive-code)
    - [Build charm](#build-charm)
    - [Deploy](#deploy)
    - [Deploy 1](#deploy-1)
    - [Deploy to machine 1](#deploy-to-machine-1)
    - [Deploy after fix](#deploy-after-fix)
    - [Deploy after fix](#deploy-after-fix)
    - [Deploy and test again](#deploy-and-test-again)
- [What be the problem!](#what-be-the-problem)
    - [Deploy and test again](#deploy-and-test-again)

<!-- /TOC -->
# Remote nfs mount charm

## Resources

- [Charm FAQ](charm-FAQ.md)
- [Charm development KB](https://juju.is/tutorials/tutorial-charm-development-part1)
- [Mike wilson's article](https://medium.com/@knobby/nfs-default-storage-in-kubernetes-with-cdk-847336cc4a72)
- [Setting up static storage with kubernetes](https://discourse.charmhub.io/t/setting-up-static-kubernetes-storage-tutorial/1193)


## Sequencing flow via reactive calls

Built up from the recommendations. See [charms-FAQ.md](charms-FAQ.md). The remote-nfs charm follows this except, it also uses an extra *joined* flag to handle relations.

![charm-sequencing](https://www.plantuml.com/plantuml/png/dLPjYnCn4FwUNp7UbzukjtlS-KtaE105FmY2WegLaardNsYQr2IrTuZ_tIUxUsssje8k57OpRy_CF3FflG_IXUrQ2p6ZJ_AHWgNGTcP5ihBRG6kxPDsPXYHv_hvbevXiJKfgxM7H6GWSInMw7qogbMvD_j47NWiu3amRwyEYC_ZGMeREcaKVa5w-hArRtTt5i1zwpPiNzDunfRgZZulxNSjcOMoud7tkOvIZ5YzdsGL9JmzHkjYV2dmnQQv317iiVFt0QXlzKu_5KykEnHij6qyI46gfFLFrI4kkvLO7yQOpdM_PJy15pnl7XUFlFa8GSXDe8zLADapIB556HWI6Vqdxu5GZZ6288yH3zhV87gKwd_vVKZyu_oa4qTHAbYxs3Itf3Ku_S2CNUprwGKgpT9TvdbZdUJRNSvtdH4kxa9TPdiTl9DZhoH-oqxBItEl5quiyhrBpY-m4kBtPB9kRyRIQGWQuwYIuwXouQWgkEWrE_IUuKwVg3yXg0XbyG5mxr-WgJMfT57UJz22kZiIfA_5huBRhcdOazu7OVp28j6qw9GQQb_HQgfRghL6z39DUSQHmtJt0ywuBBKbnbcn2l8t3hgHNSiabSDwC7W5iCpG54n9cdaQuSMtqTEyvTdtjBSIqisOMQCMmQJZqIgeFSNqDnTkItactYeuywvhsnORiUKaVaNV3Y8FX8eCOVN38af4wjt9YATDgOVQUNNu-f9WH52O7ucl-RJIRRf2ZxzZWJMIQjsiky5gCUm1qEw4xi389T4ejXZsJYTZZKjRKNVEgbQO1gibNJ2OCrK98iBqNbcgGbgfkRlByNqrk_z44j2-yBQo9WqcHs-DEEzYSsJJcS3uPNdDc-9Fpq_hgZBuwqY_npCtJLajjn1FelxLcEfoZXJWgovShIhfF71H6mA2RQoOHhrJpIDOjsO7mbd1Huj8TRr-CozLmAOiu0hNLskwUPYXUm9qHKj1yGHNrSuSt9SOrZtVG7ilfy8Xnu1FIuzOZhFUu3r65fFvPty5uc1G_U_AGAev74xNe5eivyP7eqyY3r6AlgL8tQ4_yGVWH-GzMQkiScr0qe5OsrCk3ZDPmuLiRG5hUN_aMPNExplCLxHXB3rMRx28nBADaduXxlECVqcy0 "charm-sequencing")

## Remote NFS Mount charm flow


![charm-remote-nfs](https://www.plantuml.com/plantuml/png/dLR1Rjim3BtxAuXSdBfrTkcfM8i20tRiW6stjZ1KXxQrAf8doKcp8ly-odBI9P4TPq61k2Bv-FX8ohatbcdRB4GKHVHe4IRtgmfb9fMTnXeNoc8g2vCkL2FjZ4jo4WBdySK4c867RpypicRUCJjmYXwJPqZJkrDkmsQu0LRRQP8u6hbQB9HCaelJGGPTK7y98q0Szm7UG9yzrKOeDf3jdG3fRolQYd6R5Kfdk4HfoKIsBjbtp94lAQGrjRWevxNYX4S9Ph_egTKWIGBdIK981-YKB-19VY7SyzdbIXQy_58nMNQy_CaizqUpj_MVbbO7GRtBhEPbYHhm3VF6uhm5sXxUG2wGQMgmK3aJNikCBHaNx4MWw-rnZKHxZmYvx1CZXuy4Hnn9WUlKe5wYRhcU6PlIffLepmvj2zm5jpDt111g2En0dVbVcIJYdAh_a7jevOFwTiCS08N0uGqqilqVb9XtTbzFZxiZuTmbhjp2W1SzxXA2DteXGk5-y-CmSpyQ0s4Q1RDSILhJGgEfEhiBxCrv9F4kNxzxJvwFjcpFTuwbE_tycPhyUdTtn6INiaCF15D4JzR1cBux8EpV9Wan3uMOYjUUlpcSn-7Bz4dqEVKCR1UGCRcUFWdNoJXGLtpr95lR4UoMSTgLa7PZuR7yFWnLvxm6xBJvMCFjJkkiq6gH5OALqqc0vsGxYwT0yw8ar4gfLvJBcRysfXSZ0hs0v52J1JF_uYMezqePAza2hw0mTbtJjwhRyCq8L9fTP7b5jSve_RDjajyD6cj6yGAecQs8Y5RAmYVFyp5cTIruxgVDiOkVHs7HRKk4w6NiGDkh-NJClZ9E4dohbRQPLuNFwVxb1KTzvLtRti5c91_h-81fNj8vrw0AUFZwmzTA0yOa-5ndKlLCBo_3NVsFgWrStXuSZPhmRW8RZTC3GDVxZPlfJHJHcmZ1gjgLF_Jxoqcpl8OLGaxAL6o9mC1mMT8bw7K2Bi5MI9lYto7mqjZMUKzcuxJbXVDS4vfs_amOrO6OY3OG7AaekgT7-hdy5m00 "charm-remote-nfs")


When you have multiple reactive functions, ensuring deterministic sequencing is the most important. 
- Use custom flags for each function
- use `register_trigger` to prioritize charm-system flags which we convert to custom flags.
- Make sure you consider which system/interface flags are auto-reset and which should be manually reset

If you look at the code below, this is what should happen.

**init**
- All functions except `install_nfs` are blocked as they wait for `remote-nfs-mount.installed` which defaults to `False`.
- `install_nfs` sets the `remote-nfs-mount.installed` flag which allows the rest to proceed (they are gated on additional flags as well)

Passing through the `@when(remote-nfs-mount.installed)` gate, I want local nfs mounting to be done next
- gate `setup_local_mount()` with a `@when_not(remote-nfs-mount.local_mount_available)`
  - default state allows it to execute as soon as `@when(remote-nfs-mount.installed)` allows
  - mark as available and refresh_needed

Note that the `set_trigger` calls ensure that any cnofig change immeditely clears `remote-nfs-mount.local_mount_available`. Since this gates the refresh_relation_mounts. Any incoming join will not take effect till the config changes are processed first.


```python
from charms.reactive import when, when_any, when_not
from charms.reactive import set_flag, clear_flag
from charms.reactive import register_trigger
...
...
register_trigger(when='config.changed.nfs4-server'  , clear_flag='remote-nfs-mount.local_mount_available')
register_trigger(when='config.changed.nfs-target'  , clear_flag='remote-nfs-mount.local_mount_available')
register_trigger(when='config.changed.local-target', clear_flag='remote-nfs-mount.local_mount_available')

@when_not('remote-nfs-mount.installed')
def install_nfs():
    ...
    ...    
    set_flag('remote-nfs-mount.installed')

@when('remote-nfs-mount.installed')
@when_not('remote-nfs-mount.local_mount_available')
def setup_local_mount():
    ...
    ...
    set_flag('remote-nfs-mount.local_mount_available')
    set_flag('remote-nfs-mount.relation_refresh_needed')


@when('remote-nfs-mount.installed')
@when('remote-nfs-mount.local_mount_available')
@when_any(
    'endpoint.nfs.joined',
    'remote-nfs-mount.relation-refresh-needed'
    )
def refresh_relation_mounts():
   clear_flag('remote-nfs-mount.relation_refresh_needed')

```

## Build charm locally

```console
vamsi@MAAS:~/bitbucket/infrastructure/charms$ charm build remote-nfs-mount
```

## Deploy local charm

> Deploys to **juju-k8s** model.
> see output of `juju status` to ensure machine `1` is acceptable
> Note: One of the NAS's 10G ports is attached to the same switch as MAAS. So will be on the MAAS subnet. Confirm the IP from vamsi-nas.

```
cd ~/charms/builds
juju deploy -m juju-k8s ./remote-nfs-mount --to 1 --config "nfs4-server=192.168.23.190" --config "nfs-target=/data/K8s" --config "local-target=/mnt/cluster-io"
```

- deploy to `juju-k8s` model. Allows me to delete the entire model out for cleanup.
- all configs specified inline
- deployed to specified machine (*see output of `juju status`*) otherwise juju will request MAAS for a new node and wait forever for a new machine.

## Relate to consumer K8s charm

```
juju add-relation remote-nfs-mount:nfs kubernetes-worker:nfs
```

- explicitly joins the `nfs` endpoint to make things clear
- kubernetes-worker's metadata.yaml specifies that it consumes `nfs` with `mount` interface
- kubernetes-master does it as well. Not sure how that join differs though.
- See [kubernetes-nfs-client](https://gist.github.com/vfarcic/2a19d910db0fda391494709dbff8de5f) for the amount of work the kubernetes-worker charm does when the `nfs` relation is setup! Awesome.

## Resulting K8s storageclass

```diff
vamsi@MAAS:~/charms/builds$ kubectl get sc
NAME                  PROVISIONER       RECLAIMPOLICY   VOLUMEBINDINGMODE   ALLOWVOLUMEEXPANSION   AGE
+default (default)     fuseim.pri/ifs    Delete          Immediate           false                  86s
```

## Create juju storage class and pool from K8s

```console
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

## TODO

- [ ] Actually test that the NFS server and exports are v4
- [ ] Lifecycle handling
  - [ ] Break relationship
  - [ ] Upgrade
- [ ] Multiple unit: Connect more than one unit of the same name.

# Charm development log

## Create charm skeleton

create under `~/bitbucket/infrastructure/charms`

```console
vamsi@MAAS:~/bitbucket/infrastructure/charms$ charm create remote-nfs-mount
INFO: Using default charm template (reactive-python). To select a different template, use the -t option.
INFO: Generating charm for remote-nfs-mount in ./remote-nfs-mount
INFO: No remote-nfs-mount in apt cache; creating an empty charm instead.
Cloning into '/tmp/tmpth8cpmzs'...
warning: templates not found /snap/charm/562/share/git-core/templates
remote: Enumerating objects: 32, done.
remote: Total 32 (delta 0), reused 0 (delta 0), pack-reused 32
Unpacking objects: 100% (32/32), done.
vamsi@MAAS:~/bitbucket/infrastructure/charms$ ls -l
total 4
drwx------ 4 vamsi vamsi 4096 Dec 12 07:58 remote-nfs-mount

vamsi@MAAS:~/bitbucket/infrastructure/charms$ tree
.
└── remote-nfs-mount
    ├── config.yaml
    ├── icon.svg
    ├── layer.yaml
    ├── metadata.yaml
    ├── reactive
    │   └── remote_nfs_mount.py
    ├── README.ex
    └── tests
        ├── 00-setup
        └── 10-deploy
```


## Add charm content

Left most things untouched except the meat of the charm. Updated the following (details below)
- metadata.yaml
- config.yaml
- reactive_code

### layers

Added the following layers

- basic : standard for everything. Provides basic support
- apt   : provides suport for apt as we need it to install new packages

Added this one interface that we must provide for kubernetes-worker. Initially simply took it from whatever the nfs charm specified. But later, I read the `metadata.yaml` and saw that it consumed a `nfs` endpoint with a `mount` interface: this is right way to figure dependencies.
- mount

### Metadata

This is where the overall description of the charm is specified. I started with copying items from `nfs` and `remote-nfs` and adjusting. Ended up with 

```yaml
name: remote-nfs-mount

summary: Provides mount relation for external NFS4 exports

maintainer: Vamsidhar Juvvigunta <Vamsidhar.Juvvigunta@MAAS>

description: |
  Mounts an NFS4 share via a specified mount point
  Provides the 'nfs' endpoint with 'mount' interface (same as nfs charm)
    so that any charm that can relate to nfs will work here except it 
    uses an exteral mount.
  Uses nfs4 capability of being able to mount sub-directories of the one
  single export asif the sub-directory were also explicitly exported.

tags:  
  - storage
  - network

# This is not a subordinate (does not need to be running in the same container
# as the apps it is related to).
subordinate: false

provides:
  # The goal is to interface with kubernetes via nfs as shown in the examples of
  # the nfs charm: except use an external nfs mount
  # Needs the 'nfs` endpoint with mount interface as this is what kubernetes-worker
  # expects (See it's charm code)
  nfs:
    interface: mount

series:
  - focal
```

### Reactive code

This is where the fun was. Copied the code for `nfs` charm and started iterating it. Ended up with something very different. Now that I have one under my belt, any new one can start with a proper flag flow. 

started iterating along these lines
- simply creating local mount point from my readynas nfs
- plenty of bug fixes and debugging loops
- added joining flag
- bug fixes
- switch metadata to use the `nfs` endpoint instead of the initial `remote-nfs-mount` one where I assumed that the `mount` interface was all that was needed. However `kubernetes-worker` consumes the `nfs` endpoint.
- ended with `kubectl get sc` showed me the right output.

## Build charm

> `charm build remote-nfs-mount`

```console
vamsi@MAAS:~/bitbucket/infrastructure/charms$ charm build remote-nfs-mount
build: Please add a `repo` key to your layer.yaml, with a url from which your layer can be cloned.
build: Destination charm directory: /home/vamsi/charms/builds/remote-nfs-mount
build: Processing layer: layer:options
build: Processing layer: layer:basic
build: Processing layer: layer:status
build: Processing layer: layer:apt
build: Processing layer: remote-nfs-mount (from remote-nfs-mount)
build: Processing interface: mount
build: 
build: ---------------------------------------
build:               Build Report
build: ---------------------------------------
build: New build; all files were modified.
proof: I: `display-name` not provided, add for custom naming in the UI
proof: W: Includes template README.ex file
proof: W: README.ex includes boilerplate: Step by step instructions on using the charm:
proof: W: README.ex includes boilerplate: You can then browse to http://ip-address to configure the service.
proof: W: README.ex includes boilerplate: - Upstream mailing list or contact information
proof: W: README.ex includes boilerplate: - Feel free to add things if it's useful for users
proof: I: config.yaml: option nfs-server has no default value
proof: I: config.yaml: option nfs-target has no default value
proof: I: config.yaml: option local-target has no default value
```

## Deploy

To debug while deploying. Keep juju logs open

- `watch -n0.5 --color juju status --color`
- `juju debug-log --tail-true`

----

## Deploy 1

```console
cd ~/charms/builds
juju deploy -m k8s ./remote-nfs-mount --config "nfs-server=192.168.23.196" --config "nfs-target=/data/K8s" --config "local-target=/mnt/cluster-io"
```

**Errors**
- It was looking for a new machine to deploy the charm to!!
- Maybe this is what the subordinate charm is for ? Looking at the [subordinate charms](https://juju.is/docs/charm-writing/subordinates) docs however, I see that I just need one instance of this on an existing node.
- use the `--to x` arg to deploy it to a known machine

-----

## Deploy to machine 1

pick a machine from `juju status`
- same machine as infra 2 (non k8s master)

```console
cd ~/charms/builds
juju deploy -m k8s ./remote-nfs-mount --to 1 --config "nfs-server=192.168.23.196" --config "nfs-target=/data/K8s" --config "local-target=/mnt/cluster-io"
```

**Error: hook failed: install**
- check logs
- `juju ssh remote-nfs-mount/2`
- examine `/var/log/juju/unit-remote-nfs-mount-2.log`

```log
2020-12-13 20:47:02 ERROR juju-log Hook error:
Traceback (most recent call last):
  File "/var/lib/juju/agents/unit-remote-nfs-mount-2/.venv/lib/python3.8/site-packages/charms/reactive/__init__.py", line 71, in main
    bus.discover()
  File "/var/lib/juju/agents/unit-remote-nfs-mount-2/.venv/lib/python3.8/site-packages/charms/reactive/bus.py", line 426, in discover
    _register_handlers_from_file(search_path, filepath)
  File "/var/lib/juju/agents/unit-remote-nfs-mount-2/.venv/lib/python3.8/site-packages/charms/reactive/bus.py", line 466, in _register_handlers_from_file
    _load_module(root, filepath)
  File "/var/lib/juju/agents/unit-remote-nfs-mount-2/.venv/lib/python3.8/site-packages/charms/reactive/bus.py", line 444, in _load_module
    return importlib.import_module(package + module)
  File "/usr/lib/python3.8/importlib/__init__.py", line 127, in import_module
    return _bootstrap._gcd_import(name[level:], package, level)
  File "<frozen importlib._bootstrap>", line 1014, in _gcd_import
  File "<frozen importlib._bootstrap>", line 991, in _find_and_load
  File "<frozen importlib._bootstrap>", line 975, in _find_and_load_unlocked
  File "<frozen importlib._bootstrap>", line 671, in _load_unlocked
  File "<frozen importlib._bootstrap_external>", line 779, in exec_module
  File "<frozen importlib._bootstrap_external>", line 916, in get_code
  File "<frozen importlib._bootstrap_external>", line 846, in source_to_code
  File "<frozen importlib._bootstrap>", line 219, in _call_with_frames_removed
  File "/var/lib/juju/agents/unit-remote-nfs-mount-2/charm/reactive/remote_nfs_mount.py", line 128
    mount_responses = []
    ^
SyntaxError: invalid syntax
```

- but the error was not there. Reading the code carefully, I see that I was using a non existant `mount_options` variable. Fixed and re-deployed
- another error. Un-terminated format string in an earlier command

-----

## Deploy after fix

```console
cd ~/charms/builds
juju deploy -m k8s ./remote-nfs-mount --to 1 --config "nfs-server=192.168.23.196" --config "nfs-target=/data/K8s" --config "local-target=/mnt/cluster-io"
```

Worked great! Status shows the nfs export mount.

Now try the relation and see if it creates app specific exports

`juju add-relation remote-nfs-mount kubernetes-worker`

**Error**
- hook failed in `juju status`
- error logs on the machine shows another python issue.

```log
2021-02-06 04:01:08 WARNING remote-nfs-mount-relation-joined Traceback (most recent call last):
2021-02-06 04:01:08 WARNING remote-nfs-mount-relation-joined   File "/var/lib/juju/agents/unit-remote-nfs-mount-6/charm/hooks/remote-nfs-mount-relation-joined", line 22, in <module>
2021-02-06 04:01:08 WARNING remote-nfs-mount-relation-joined     main()
2021-02-06 04:01:08 WARNING remote-nfs-mount-relation-joined   File "/var/lib/juju/agents/unit-remote-nfs-mount-6/.venv/lib/python3.8/site-packages/charms/reactive/__init__.py", line 74, in main
2021-02-06 04:01:08 WARNING remote-nfs-mount-relation-joined     bus.dispatch(restricted=restricted_mode)
2021-02-06 04:01:08 WARNING remote-nfs-mount-relation-joined   File "/var/lib/juju/agents/unit-remote-nfs-mount-6/.venv/lib/python3.8/site-packages/charms/reactive/bus.py", line 390, in dispatch
2021-02-06 04:01:08 WARNING remote-nfs-mount-relation-joined     _invoke(other_handlers)
2021-02-06 04:01:08 WARNING remote-nfs-mount-relation-joined   File "/var/lib/juju/agents/unit-remote-nfs-mount-6/.venv/lib/python3.8/site-packages/charms/reactive/bus.py", line 359, in _invoke
2021-02-06 04:01:08 WARNING remote-nfs-mount-relation-joined     handler.invoke()
2021-02-06 04:01:08 WARNING remote-nfs-mount-relation-joined   File "/var/lib/juju/agents/unit-remote-nfs-mount-6/.venv/lib/python3.8/site-packages/charms/reactive/bus.py", line 181, in invoke
2021-02-06 04:01:08 WARNING remote-nfs-mount-relation-joined     self._action(*args)
2021-02-06 04:01:08 WARNING remote-nfs-mount-relation-joined   File "/var/lib/juju/agents/unit-remote-nfs-mount-6/charm/reactive/remote_nfs_mount.py", line 101, in refresh_relation_mounts
2021-02-06 04:01:08 WARNING remote-nfs-mount-relation-joined     mount_interface = endpoint_from_flag('endpoint.remote-nfs-mount.joined')
2021-02-06 04:01:08 WARNING remote-nfs-mount-relation-joined NameError: name 'endpoint_from_flag' is not defined
2021-02-06 04:01:08 ERROR juju.worker.uniter.operation runhook.go:136 hook "remote-nfs-mount-relation-joined" (via explicit, bespoke hook script) failed: exit status 1
2021-02-06 04:01:08 INFO juju.worker.uniter resolver.go:143 awaiting error resolution for "relation-joined" hook
```

Ok. Delete the charm

```console
juju remove-application --force remote-nfs-mount
```

Fixed by looking at the source for the nfs charm. I needed a new import `from charms.reactive.relations import endpoint_from_flag, endpoint_from_name`

---

## Deploy after fix


```
cd ~/bitbucket/infrastructure/charms
charm build remote-nfs.mount
cd ~/charms/builds
juju deploy -m k8s ./remote-nfs-mount --to 1 --config "nfs-server=192.168.23.196" --config "nfs-target=/data/K8s" --config "local-target=/mnt/cluster-io"
```

That went fine. Now relate

```
juju add-relation remote-nfs-mount kubernetes-worker
```

Gah! Again hook failed. 

```
juju ssh remote-nfs-mount/7
vi /var/log/juju/unit-remote-nfs-mount-7.log
```

Of all the stupidity!

```log
2021-02-06 18:39:08 ERROR juju-log remote-nfs-mount:22: Hook error:
Traceback (most recent call last):
  File "/var/lib/juju/agents/unit-remote-nfs-mount-7/.venv/lib/python3.8/site-packages/charms/reactive/__init__.py", line 74, in main
    bus.dispatch(restricted=restricted_mode)
  File "/var/lib/juju/agents/unit-remote-nfs-mount-7/.venv/lib/python3.8/site-packages/charms/reactive/bus.py", line 390, in dispatch
    _invoke(other_handlers)
  File "/var/lib/juju/agents/unit-remote-nfs-mount-7/.venv/lib/python3.8/site-packages/charms/reactive/bus.py", line 359, in _invoke
    handler.invoke()
  File "/var/lib/juju/agents/unit-remote-nfs-mount-7/.venv/lib/python3.8/site-packages/charms/reactive/bus.py", line 181, in invoke
    self._action(*args)
  File "/var/lib/juju/agents/unit-remote-nfs-mount-7/charm/reactive/remote_nfs_mount.py", line 138, in refresh_relation_mounts
    local_path = os.path.join(nfs_root_mount, mount['application_name'])
NameError: name 'os' is not defined
```

Add `import os` and try again. Script also uses the `PermissionsError` exception but that is described as a built in. Maybe no import needed for that one.

## Deploy and test again

```
cd ~/bitbucket/infrastructure/charms
charm build remote-nfs.mount
cd ~/charms/builds
juju deploy -m k8s ./remote-nfs-mount --to 1 --config "nfs-server=192.168.23.196" --config "nfs-target=/data/K8s" --config "local-target=/mnt/cluster-io"
juju add-relation remote-nfs-mount kubernetes-worker
```


Yay!!! Notes line shows

> 192.168.23.196:/data/K8s -> /mnt/cluster-io. Serving [kubernetes-worker]

<pre><font color="#4E9A06"><b>vamsi@MAAS</b></font>:<font color="#3465A4"><b>~</b></font>$ juju status
Model  Controller       Cloud/Region     Version  SLA          Timestamp
k8s    juju-controller  devmaas/default  2.8.6    unsupported  10:49:23-08:00

App                    Version  Status  Scale  Charm                  Store       Rev  OS      Notes
containerd             1.3.3    <font color="#4E9A06">active    </font><font color="#C4A000">2/3  </font>containerd             jujucharms   96  ubuntu  
easyrsa                3.0.1    <font color="#4E9A06">active      </font>1  easyrsa                jujucharms  338  ubuntu  
etcd                   3.4.5    <font color="#4E9A06">active      </font>1  etcd                   jujucharms  543  ubuntu  
flannel                0.11.0   <font color="#4E9A06">active    </font><font color="#C4A000">2/3  </font>flannel                jujucharms  512  ubuntu  
kubeapi-load-balancer  1.18.0   <font color="#4E9A06">active      </font>1  kubeapi-load-balancer  jujucharms  752  ubuntu  exposed
kubernetes-master      1.19.7   <font color="#4E9A06">active      </font>1  kubernetes-master      jujucharms  908  ubuntu  
kubernetes-worker      1.19.7   <font color="#4E9A06">active    </font><font color="#C4A000">1/2  </font>kubernetes-worker      jujucharms  711  ubuntu  exposed
remote-nfs-mount                <font color="#4E9A06">active      </font>1  remote-nfs-mount       local        10  ubuntu  

Unit                      Workload  Agent  Machine  Public address  Ports           Message
easyrsa/0*                <font color="#4E9A06">active    idle   </font>0        192.168.23.103                  Certificate Authority connected.
etcd/0*                   <font color="#4E9A06">active    idle   </font>0        192.168.23.103  2379/tcp        Healthy with 1 known peer
kubeapi-load-balancer/0*  <font color="#4E9A06">active    idle   </font>0        192.168.23.103  443/tcp         Loadbalancer ready.
kubernetes-master/0*      <font color="#4E9A06">active    idle   </font>0        192.168.23.103  6443/tcp        Kubernetes master running.
  containerd/2            <font color="#4E9A06">active    idle   </font>         192.168.23.103                  Container runtime available
  flannel/2               <font color="#4E9A06">active    idle   </font>         192.168.23.103                  Flannel subnet 10.1.6.1/24
kubernetes-worker/0*      <font color="#4E9A06">active    idle   </font>1        192.168.23.104  80/tcp,443/tcp  Kubernetes worker running.
  containerd/1*           <font color="#4E9A06">active    idle   </font>         192.168.23.104                  Container runtime available
  flannel/1*              <font color="#4E9A06">active    idle   </font>         192.168.23.104                  Flannel subnet 10.1.14.1/24
kubernetes-worker/1       <font color="#C4A000">unknown   lost   </font>2        192.168.23.105  80/tcp,443/tcp  agent lost, see &apos;juju show-status-log kubernetes-worker/1&apos;
  containerd/0            <font color="#C4A000">unknown   lost   </font>         192.168.23.105                  agent lost, see &apos;juju show-status-log containerd/0&apos;
  flannel/0               <font color="#C4A000">unknown   lost   </font>         192.168.23.105                  agent lost, see &apos;juju show-status-log flannel/0&apos;
remote-nfs-mount/8*       <font color="#4E9A06">active    idle   </font>1        192.168.23.104                  192.168.23.196:/data/K8s -&gt; /mnt/cluster-io. Serving [kubernetes-worker]

Machine  State    DNS             Inst id  Series  AZ       Message
0        <font color="#4E9A06">started  </font>192.168.23.103  tiny1    focal   default  Deployed
1        <font color="#4E9A06">started  </font>192.168.23.104  tiny2    focal   default  Deployed
2        <font color="#CC0000">down     </font>192.168.23.105  big-boy  focal   default  Deployed
</pre>

Check the NAS itself. Looks good!

```console
juju ssh remote-nfs-mount/8
ubuntu@tiny2:/$ cd /mnt/cluster-io/
ubuntu@tiny2:/mnt/cluster-io$ ls -al
total 4
dr-xr-xr-x 1 root   root    186 Feb  6 18:51 .
drwxr-xr-x 3 root   root   4096 Nov 27 21:44 ..
drwxrwxrwx 1 root   root      0 Feb  6 18:51 kubernetes-worker
```

Check kubernetes storage configuration.

<pre><font color="#4E9A06"><b>vamsi@MAAS</b></font>:<font color="#3465A4"><b>~/charms/builds</b></font>$ kubectl get sc
NAME                  PROVISIONER       RECLAIMPOLICY   VOLUMEBINDINGMODE   ALLOWVOLUMEEXPANSION   AGE
managed-nfs-storage   example.com/nfs   Delete          Immediate           false                  69d
</pre>


**However, this is the old sc** created via the yaml file. The new one is not showing up! Something else messed up. I might need to actually ask the forums for help!

# What be the problem!

Maybe the `provides:nfs` is the key instead of `provides: remote-nfs-mount`. Maybe that is the name of the relation that does it.

Checked the source of the kubernets-worker as that is what is used in the examples.

Yup. Right there in kubernetes-worker's `metadata.yaml` file

```yaml
requires:
  ...
  nfs:
    interface: mount
```

Reactive code looks for `fstype=nfs`. Blindly assuming semantic equivalence to linux mount-type and using *nfs4* was obviously wrong.

Next steps
- provide `nfs` relation (change metadata.yaml)
- Also renamed `nfs-server` to `nfs4-server` to make it explicit.
- rename current random provides name to `nfs` and update code: mostly `endpoint.nfs.joined`
- explicitly relate to `remote-nfs-mount:nfs` to make it clear that the app is remote-nfs-mount.

## Deploy and test again

```
cd ~/bitbucket/infrastructure/charms
charm build remote-nfs-mount
cd ~/charms/builds
juju deploy -m k8s ./remote-nfs-mount --to 1 --config "nfs4-server=192.168.23.196" --config "nfs-target=/data/K8s" --config "local-target=/mnt/cluster-io"
juju add-relation remote-nfs-mount:nfs kubernetes-worker
```

**Fix 1** - Seemed to work ok but status was unchanged. When I looked at log, turns out I was checking for end-points for `endpoint.remote-nfs-mount.joined` instead of `endpoint.nfs.joined`. Fixing and restarting

Ok. Seems to work now. Shows "Serving kubernetes-worker" in status!

And Yay!!!! Same as the example in [Mike wilson's article](https://medium.com/@knobby/nfs-default-storage-in-kubernetes-with-cdk-847336cc4a72) (Author of the nfs charm )

```diff
vamsi@MAAS:~/charms/builds$ kubectl get sc
NAME                  PROVISIONER       RECLAIMPOLICY   VOLUMEBINDINGMODE   ALLOWVOLUMEEXPANSION   AGE
+default (default)     fuseim.pri/ifs    Delete          Immediate           false                  86s
managed-nfs-storage   example.com/nfs   Delete          Immediate           false                  78d
```
