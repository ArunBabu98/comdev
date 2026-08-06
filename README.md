# comdev

A from-scratch communication program written in Rust, built as a long-term
learning project. It starts as a minimal peer-to-peer messenger and is intended
to grow, step by step, into an anonymous communication and file-sharing system
in the spirit of Tor, I2P, and Freenet.

> **Educational and research use only.** comdev is a study of how anonymity
> networks are designed and where they break. It is not audited, not hardened,
> and not a privacy tool. Do not use it to protect anyone whose safety depends
> on it — use [Tor](https://www.torproject.org/) instead.

## Why build this

Anonymity systems are usually consumed as finished products, which hides the
reasoning behind them: why circuits have three hops, why padding matters, why
directory authorities exist, why "just encrypt it" is not anonymity. The only
way to internalize those answers is to build the thing, get it wrong, and see
the failure modes first-hand.

comdev is deliberately written without networking or crypto frameworks doing
the interesting parts. Every layer — framing, handshakes, routing, cover
traffic — is implemented explicitly so it can be read, broken, and revised.

## Status

Very early. The project currently consists of a skeleton binary with a stub
menu for listening or connecting, plus placeholder `node` and `network`
modules. Nothing is functional yet.

| Area | State |
| --- | --- |
| CLI entry point | Stub menu, no behaviour |
| Node model | Placeholder struct |
| Transport | Not started |
| Encryption | Not started |
| Routing / anonymity | Not started |
| File transfer | Not started |

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

**Phase 0 — Skeleton (current)**
CLI shell, module layout, project conventions.

**Phase 1 — Direct connection**
TCP listener and dialer, a length-prefixed message frame, a clean read/write
loop, and sane handling of partial reads, disconnects, and malformed input.

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

## Planned architecture

```
src/
  main.rs      CLI entry point and command dispatch
  node.rs      Node identity, local state, and lifecycle
  network.rs   Transport, framing, and connection handling
```

Later phases are expected to add modules for the cryptographic session,
circuit construction, the peer directory, and file transfer. The layout will be
revised as the design becomes clearer rather than being fixed up front.

## Building

Requires a Rust toolchain with support for the 2024 edition (Rust 1.85 or
newer). The project has no external dependencies at this stage.

    cargo build
    cargo run

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
