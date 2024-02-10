# pap-ng

This is a spiritual successor to [pap](https://github.com/talwat/pap), rewritten in Rust.

The current MSRV is the latest nightly toolchain.

## Why `pap`?

Instead of browsing for server modpacks, or even downloading mods and server jarfiles them one-by-one,
`pap` can set up a server from scratch in just a few commands. Here's a video example:

![pap video example](./assets/pap.mp4)

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
