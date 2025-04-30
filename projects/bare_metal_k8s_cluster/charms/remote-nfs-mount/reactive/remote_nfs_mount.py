import os
from charms.reactive import when, when_any, when_not 
from charms.reactive import set_flag, clear_flag
from charms.reactive import register_trigger
from charms.reactive.relations import endpoint_from_flag, endpoint_from_name
import charms.apt
import charmhelpers.core.hookenv as hookenv
import charmhelpers.core.host as host

# Trigger immediate clear of 'available' flag on config change.
register_trigger(when='config.changed.nfs4-server'  , clear_flag='remote-nfs-mount.local_mount_available')
register_trigger(when='config.changed.nfs-target'  , clear_flag='remote-nfs-mount.local_mount_available')
register_trigger(when='config.changed.local-target', clear_flag='remote-nfs-mount.local_mount_available')

@when_not('remote-nfs-mount.installed')
def install_nfs():
    hookenv.status_set('maintenance', 'Installing remote-nfs-mount')
    charms.apt.queue_install(['nfs-common'])
    hookenv.status_set('maintenance', 'Installed remote-nfs-mount')
    set_flag('remote-nfs-mount.installed')

# Any config changes, this gets called first to
# refresh local mounts. The local mounts can be used
# by charms but is meant more to create the sub-dirs needed
# for the applications that relate to us.
#
# note: cofig.changed items are supposed to auto-reset
#
# sets: remote-nfs-mount.relation_refresh_needed
@when('remote-nfs-mount.installed')
@when_not('remote-nfs-mount.local_mount_available')
def setup_local_mount():
    hookenv.status_set('maintenance', 'Refreshing local NFS mount')
    config = hookenv.config()
    new_server = config.get('nfs4-server')
    new_target = config.get('nfs-target')
    new_path = config.get('local-target')
    hookenv.log("Starting local_mount reactions to config change")
    hookenv.log("About to check for {} / {} / {}".format(new_server, new_target, new_path))
    if not (new_server and new_target and new_path):
        missing = "Missing required configuration: "
        if not new_server:
            missing += "new_server, "
        if not new_target:
            missing += "new_target, "
        if not new_path:
            missing += "new_path, "
        hookenv.status_set('blocked', missing)
        return
    hookenv.log("Setting up NFS share - {}:{} -> {}".format(new_server, new_target, new_path))
    old_server = config.previous('nfs4-server')
    old_target= config.previous('nfs-target')
    old_path = config.previous('local-target')
    if old_path:
        hookenv.log("About to unmount {}".format(
            old_path))
        try:
            unmount(old_path)
        except:
            pass
        try:
            host.fstab_remove(old_path)
        except:
            pass
    hookenv.log("About to mount {}:{} at {} as nfs4 filesystem".format(
        new_server, new_target, new_path))

    host.fstab_add(
        "{}:{}".format(new_server, new_target),
        new_path,
        "nfs4",
        options=config.get('mount-options')
        )
    try:
        host.mkdir(new_path)
    except PermissionError:
        pass
    host.fstab_mount(new_path)
    hookenv.status_set('active', "{}:{} -> {}".format(new_server, new_target, new_path))

    hookenv.log("Setting the `remote-nfs-mount.relation_refresh_needed` flag")
    set_flag('remote-nfs-mount.local_mount_available')
    set_flag('remote-nfs-mount.relation_refresh_needed')

#-------------------------------------------------------------
# Copied and modified from nfs charm's 
#      reactive/nfs.py/nfs_relation_changed
#      reactive/nfs.py/
#
# New join, this gets called (joined is auto cleared)
#-------------------------------------------------------------
# Final step of the updates.
# Refresh all the relations recorded in the `mount` interface
# that we provide.
@when('remote-nfs-mount.installed')
@when('remote-nfs-mount.local_mount_available')
@when_any(
    'endpoint.nfs.joined',
    'remote-nfs-mount.relation-refresh-needed'
    )
def refresh_relation_mounts():
    hookenv.log("Handling nfs_relation_changed")    
    mount_interface = endpoint_from_flag('endpoint.nfs.joined')
    if mount_interface is None:
        hookenv.log('No mount interface, bailing')
        return

    config            = hookenv.config()
    nfs_server        = config.get('nfs4-server')
    nfs_target        = config.get('nfs-target')
    nfs_root_mount    = config.get('local-target')
    nfs_mount_options = config.get('mount-options')

    hookenv.status_set('maintenance', "{}:{} -> {}. Creating nfs mounts for `mount` requests.".format(
        nfs_server, 
        nfs_target, 
        nfs_root_mount))

    # prepare common mount response
    # Assumes server running NFS4: as only this allows us to mount sub-directories of the global    
    # nfs_target path on nfs_server. This way, we have one export from the NFS server
    # but each endpoint request gets it's own app qualified sub-dir which can be mounted
    # as if it were exported by the NFS server
    #
    # Originally thought that I should use 'nfs4' as the 'fstype': assumed type meant the same 
    # as `mount -t type`. Alas not to be. Studying the kubernetes-worker code
    # - send in fstype="nfs"
    # - use 'nfs' end-point
    mount_response_common = {
            'hostname': nfs_server,
            'fstype'  : 'nfs',
            'options' : nfs_mount_options
    }

    # get desired mount requests    
    mount_responses = []    
    mount_requests  = mount_interface.get_mount_requests()    
    for mount in mount_requests:
        if not mount['application_name']:
            continue

        # We create the local path simply to ensure it exists
        # Then supply the path relative to the original nfs_target for mounting
        local_path = os.path.join(nfs_root_mount, mount['application_name'])
        mount_path = os.path.join(nfs_target    , mount['application_name'])
        if os.path.exists(local_path):
            hookenv.log('local data path {} for end-point application:{} already exists'.format(
                local_path, 
                mount['application_name'])
            )
        else:                
            hookenv.log('creating local data path {} for end-point application {}'.format(
                local_path,
                mount['application_name'])
            )            
            os.makedirs(local_path)
            # my soul hurts, but without something like LDAP to make user
            # id's consistent, we can't really know what user will need
            # to read/write to this path. Unfortunately we don't really
            # have a choice.
            os.chmod(local_path, 0o777)

        mount_response = {
            'export_name': mount['application_name'],
            'identifier' : mount['identifier'],
            'mountpoint' : mount_path,
        }
        mount_response.update(mount_response_common)
        mount_responses.append(mount_response)

    # Inform the mount layer
    hookenv.log("Configuring `mount` interface with join response")
    mount_interface.configure(mount_responses)
            
    hookenv.log("Clearing remote-nfs-mount.relation_refresh_needed")
    clear_flag('remote-nfs-mount.relation_refresh_needed')

    hookenv.status_set('active', "{}:{} -> {}. Serving [{}]".format(
        nfs_server, 
        nfs_target, 
        nfs_root_mount,
        ', '.join([m['application_name'] for m in mount_requests]))
    )    
