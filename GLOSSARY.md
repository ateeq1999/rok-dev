# rok-dev

Runs a project's stack locally: starts, supervises and stops the things a
project needs running, and checks the machine they run on.

## Language

**Runnable**:
One thing `rok dev` starts and supervises: an App, or a container an App
needs (the compose database). Each is one `[dev.<name>]` table in
`rok.toml`, with a `run` argv or a `compose` file, a `dir`, a `ready` check
and its `depends`; its `.env` is read from its `dir`.
_Avoid_: Service, process, task, target

## Relationships

- A **Runnable** that is an App is a rok-data **App** (rok-data's glossary);
  `migrate` applies to it.
- A **Runnable** depends on other Runnables; `depends` orders start-up and
  readiness, never builds.

## Flagged ambiguities

- "Service" means a docker compose service; a Runnable may be one, but
  rok-dev's word is Runnable.
