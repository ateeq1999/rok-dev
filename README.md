# rok-dev

Runs a project's stack: its database, API, web App and mobile App, from
`rok.toml`'s `[dev.<name>]` tables. It is the standalone `rok-dev` binary,
and `rok dev`, `rok run`, `rok doctor` and `rok port` in the toolchain's
`rok` ([rok-v2](https://github.com/ateeq1999/rok-v2)).

| Command | Does |
| --- | --- |
| `rok dev [NAME…]` | start the Runnables (all, or the named ones and what they depend on), in dependency order, reusing what is already up |
| `rok run <NAME>` | run one Runnable in the foreground, in its `dir`, with its `.env` |
| `rok doctor` | check Docker, cargo, bun, Flutter, the Android SDK and the known traps, naming each fix |
| `rok port list` / `rok port free <PORT>…` | find and free what holds a port |

**Status:** a skeleton. Every command prints "not built yet" and exits 1
until its ticket lands (rok-data's `.scratch/rok-dev/issues/`). The design
is rok-data's spec `docs/specs/rok-dev.md` and workspace ADR 0034.

## Terms

[GLOSSARY.md](GLOSSARY.md).

## License

MIT or Apache-2.0, at your option ([LICENSE-MIT](LICENSE-MIT),
[LICENSE-APACHE](LICENSE-APACHE)).
