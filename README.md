# pap-ng

This is a spiritual successor to the original, now abandoned (?) [pap](https://github.com/talwat/pap), rewritten in Rust.

The current MSRV is the latest nightly toolchain.

## Why `pap`?

Instead of browsing for server modpacks, or even downloading mods and server jarfiles them one-by-one,
`pap` can set up a server from scratch in just a few commands. Here's a video example:

https://github.com/dxbednarczyk/pap-ng/assets/54457902/1a0c8266-cbf8-48b1-bec8-aa04f1f0eb5d

## Goals
- [x] Easily set up a new Minecraft server from scratch, including mod and plugin support
- [x] Produce easily reproducible, portable server configuration
    - [x] Lockfile with detailed information on installed mods
    - [x] Support for installing lockfile dependencies
    - [x] Support for updating lockfile dependencies

✅ Be an all-in-one tool that keeps it simple, stupid!

## Non-Goals
❌ Support client-side modification

❌ Support self updating, unzipping, or anything that another program can do better
