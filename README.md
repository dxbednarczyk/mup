# pap-ng

This is a spiritual successor to [pap](https://github.com/talwat/pap), rewritten in Rust.

The current MSRV is the latest nightly toolchain.

## Goals
- [x] Easily set up a new Minecraft server from scratch, including mod and plugin support
- [x] Produce easily reproducible, portable server configuration
    - [x] Lockfile with detailed information on installed mods
    - [ ] Support for installing / updating lockfile dependencies

✅ Be an all-in-one tool that keeps it simple, stupid!

## Non-Goals
❌ Support client-side modification

❌ Support mod configuration

❌ Support self updating, unzipping, or anything that another program can do better
