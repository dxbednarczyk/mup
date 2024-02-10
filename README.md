# pap-ng

This is a spiritual successor to [pap](https://github.com/talwat/pap), rewritten in Rust.

The current MSRV is the latest nightly toolchain.

## Why `pap`?

Instead of browsing for server modpacks, or even downloading mods and server jarfiles them one-by-one,
`pap` can set up a server from scratch in just a few commands:

```shell
# Set up the latest Fabric server for Minecraft 1.20.4
$ pap server init -m 1.20.4 -l fabric
# Add the mods / plugins you want
$ pap project add fabric-api
# You can also specify specific version IDs
$ pap project add chunky --version-id=t8SbUchF
$ java -jar fabric.jar
```

Any edits you make using `pap` are saved in `pap.lock`, which lives in the
root of your server directory.

If you have a server with a valid `pap.lock` and want to host it on a different machine, just copy that file over
and run `pap server install`.

**WARNING: This will NOT copy over server configuration. This feature may or may not be added at a later date.**

## Goals
- [x] Easily set up a new Minecraft server from scratch, including mod and plugin support
- [x] Produce easily reproducible, portable server configuration
    - [x] Lockfile with detailed information on installed mods
    - [x] Support for installing lockfile dependencies
    - [ ] Support for updating lockfile dependencies

✅ Be an all-in-one tool that keeps it simple, stupid!

## Non-Goals
❌ Support client-side modification

❌ Support self updating, unzipping, or anything that another program can do better
