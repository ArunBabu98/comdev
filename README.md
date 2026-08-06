# comdev

A from-scratch communication program written in Rust, built as a long-term
learning project. It starts as a minimal peer-to-peer messenger built directly
on raw OS sockets and is intended to grow, step by step, into an anonymous
communication and file-sharing system in the spirit of Tor, I2P, and Freenet.

> **Educational and research use only.** comdev is a study of how anonymity
> networks are designed and where they break. It is not audited, not hardened,
> and not a privacy tool. There is currently no encryption of any kind — all
> traffic is plaintext. Do not use it to protect anyone whose safety depends on
> it; use [Tor](https://www.torproject.org/) instead.

## Why build this

Anonymity systems are usually consumed as finished products, which hides the
reasoning behind them: why circuits have three hops, why padding matters, why
directory authorities exist, why "just encrypt it" is not anonymity. The only
way to internalize those answers is to build the thing, get it wrong, and see
the failure modes first-hand.

comdev is deliberately written without networking or crypto frameworks doing
the interesting parts. It does not use `std::net`. Sockets are created by
calling the operating system directly — `libc` on Unix, and hand-declared
`ws2_32` bindings on Windows — so that every layer, from `socket()` upward, is
visible and modifiable.

## Status

**Working:** a plaintext TCP echo chat between two machines, verified across
platforms. A Windows laptop running as client has successfully connected to a
Mac mini running as server over a LAN, with messages arriving and being
acknowledged.

| Area | State |
| --- | --- |
| Raw socket layer (Unix) | Working — `socket`/`bind`/`listen`/`accept`/`connect`/`send`/`recv` via `libc` |
| Raw socket layer (Windows) | Working — direct `ws2_32` FFI with `WSAStartup` |
| CLI menu | Working — listen, connect, exit |
| Server (listen + echo) | Working — single client, blocking loop |
| Client (connect + send) | Working — interactive send/receive loop |
| Socket cleanup | Working — RAII via `Drop` |
| Message framing | Not started — fixed 1 KB reads, no length prefix |
| Concurrency | Not started — one client at a time |
| Encryption | Not started |
| Routing / anonymity | Not started |
| File transfer | Not started |

`node.rs` holds a placeholder `Node` struct that is not yet used.

## Requirements

- A Rust toolchain supporting the 2024 edition (Rust 1.85 or newer; developed
  on 1.97).
- The only dependency is `libc`, used on Unix targets. Windows support uses
  hand-written FFI declarations and needs no crate.

## Building

    cargo build

Do this on each machine you intend to run a node on. The Windows and Unix
socket layers are selected at compile time by `#[cfg]`, so the same source tree
builds natively on both.

## Running

comdev is a single binary that can act as either side. Launch it and pick a
role from the menu:

    cargo run

    ComDev Started..........

    Choose: 1. Listen 2. Connect 3. Exit
    >

### Both nodes on one machine

Useful for a first smoke test. Open two terminals.

1. In the first, run `cargo run`, choose `1`. It binds `0.0.0.0:8080` and waits.
2. In the second, run `cargo run`, choose `2`, and enter `127.0.0.1` when asked
   for the server IP.
3. Type messages in the client. Each one is printed by the server and echoed
   back as `ACK: <message>`.
4. Type `exit` in the client to end the session. Both sides return to the menu.

### Across two machines (the verified setup)

This is the configuration that has been tested: **Mac mini as server, Windows
laptop as client**, both on the same local network.

**On the server (Mac mini):**

Find the LAN address the client should dial:

    ipconfig getifaddr en0

Then start the server:

    cargo run

Choose `1`. It binds `0.0.0.0:8080`, so it accepts connections on every
interface, and prints `Listening on 0.0.0.0:8080...`.

macOS may show a firewall prompt asking whether to allow incoming connections
for the binary — allow it, or the client's connection will hang or be refused.
Both machines must be on the same network, and any router-level client
isolation (common on guest Wi-Fi) will block the connection.

**On the client (Windows laptop):**

    cargo run

Choose `2`, then enter the Mac mini's LAN address at the prompt, for example
`192.168.1.42`. On success the client prints
`Connected successfully to 192.168.1.42:8080` and the server prints
`Client connected from: <client-ip>:<port>`.

Type a message and press enter. The server prints
`Received from client: <message>` and replies, which the client shows as
`Server reply: ACK: <message>`.

Type `exit` on the client to close the session cleanly. The server logs
`Client disconnected.` and returns to its menu.

The roles are not tied to platform — the Mac can equally be the client and the
Windows machine the server. That direction simply hasn't been exercised yet.

### Notes and current limits

- **The port is hardcoded to 8080** on both sides ([main.rs:25](src/main.rs#L25)
  and [main.rs:38](src/main.rs#L38)). Changing it means editing the source.
- **The server accepts exactly one client**, serves it until disconnect, then
  returns to the menu. There is no concurrency yet.
- **Reads are a flat 1 KB buffer** with no framing, so a message larger than
  1024 bytes, or two messages sent quickly, may be split or merged. Message
  framing is Phase 1 work.
- **Everything is plaintext.** Anyone on the path can read it.
- If a run leaves the port occupied, the next bind will fail with
  "Address already in use" — the socket has no `SO_REUSEADDR` yet. Wait for the
  OS to release it or change the port.

## Goals

- **Direct peer messaging first.** Two nodes, one TCP connection, a hand-rolled
  message frame. Get correctness and error handling right before anything else.
- **Confidentiality and authenticity.** An explicit handshake with ephemeral key
  exchange and authenticated encryption, so message contents and framing are not
  readable on the wire.
- **Multi-hop routing.** Layered ("onion") encryption across relays so no single
  relay learns both the sender and the recipient.
- **Peer discovery.** Move from hardcoded addresses to a distributed peer/relay
  directory, and understand the trust problem that creates.
- **Traffic analysis resistance.** Padding, fixed-size cells, and timing
  strategies — plus honest measurement of how much they actually help.
- **Anonymous file sharing.** Chunked, content-addressed transfer layered on top
  of the anonymous transport.

## Non-goals

- Being a production or "daily driver" privacy tool.
- Interoperating with the Tor or I2P networks.
- Novel cryptography. Where crypto is used, it should be standard primitives
  used in standard ways; the research interest is in the network design.
- Evading any particular adversary's detection. The interest here is in how
  anonymity properties are constructed and where they leak, not in helping
  anyone hide from lawful scrutiny.

## Roadmap

Each phase is meant to be finished and understood before the next begins.

**Phase 0 — Skeleton — done**
CLI shell, module layout, project conventions.

**Phase 1 — Direct connection — in progress**
Cross-platform raw sockets, connect/listen/send/recv, and RAII cleanup are
working. Remaining: a length-prefixed message frame, correct handling of
partial reads and merged messages, configurable ports, `SO_REUSEADDR`, and
handling more than one client.

**Phase 2 — Encrypted channel**
A handshake with ephemeral key agreement, an authenticated-encryption session,
replay protection, and a documented state machine for the handshake.

**Phase 3 — Relaying**
A node that forwards traffic for others. Introduces circuits, per-hop keys, and
the distinction between a message's route and its content.

**Phase 4 — Onion routing**
Layered encryption over multi-hop circuits, circuit setup and teardown, and
fixed-size cells.

**Phase 5 — Discovery**
Peer and relay directories, node descriptors, and the trust and Sybil problems
that come with them.

**Phase 6 — Traffic analysis defences**
Cover traffic, padding schedules, and measurement of correlation resistance.

**Phase 7 — File sharing**
Content addressing, chunking, resumable transfers, and storage over the
anonymous layer.

## Architecture

```
src/
  main.rs      CLI entry point, role selection, and the interactive loops
  sys.rs       Raw OS socket layer — libc on Unix, ws2_32 FFI on Windows
  network.rs   ComDevnw: safe wrapper over sys, connection and session handling
  node.rs      Node identity and local state (placeholder)
```

**`sys.rs`** is the only place `unsafe` appears. It exposes one portable API —
`create_tcp_socket`, `bind_socket`, `listen_socket`, `accept_socket`,
`connect_socket`, `send_bytes`, `recv_bytes`, `close_socket` — with two
`#[cfg]`-gated implementations behind it. The Unix side calls `libc`; the
Windows side declares the `ws2_32` symbols and the `SOCKADDR_IN` / `WSADATA`
layouts by hand and runs `WSAStartup` on socket creation. `RawSocket` is a
`c_int` on Unix and a `usize` on Windows.

**`network.rs`** wraps that in `ComDevnw`, which owns a socket and closes it in
`Drop`, so a socket cannot outlive its owner or leak on an error path.

Later phases are expected to add modules for the cryptographic session, circuit
construction, the peer directory, and file transfer. The layout will be revised
as the design becomes clearer rather than being fixed up front.

## Design notes

**Threat model.** Not yet defined, and that is deliberate — writing it down is
part of Phase 3, once there is something whose properties can meaningfully be
described. Anything claimed about anonymity before then should be read as an
aspiration, not a property.

**On rolling your own.** Implementing a protocol by hand is how you learn it and
also how you produce something insecure. Both are true at once. comdev accepts
the second in exchange for the first, which is exactly why it carries the
warning at the top of this file.

## Further reading

- [Tor: The Second-Generation Onion Router](https://svn.torproject.org/svn/projects/design-paper/tor-design.pdf) — Dingledine, Mathewson, Syverson
- [Tor design specifications](https://spec.torproject.org/)
- [I2P technical documentation](https://geti2p.net/en/docs)
- [Anonymity Bibliography](https://www.freehaven.net/anonbib/) — the standard index of anonymity research

## License

Not yet chosen.
