# AGENTS.md

Guidance for AI coding agents (and humans) working in this repository.

This file is the single source of truth for repository-wide conventions and
agent behavior in `openDevicePartnership/mimxrt685s-pac`. It is a superset of
`.github/copilot-instructions.md`; if anything appears to contradict, this
file wins.

---

## 1. Project overview

`mimxrt685s-pac` is a **Peripheral Access Crate (PAC)** for the NXP
**MIMXRT685S** (Cortex-M33) microcontroller. The crate is `#![no_std]` and
exposes typed, register-level access to every memory-mapped peripheral on the
device. It is the foundation that higher-level HALs and embedded-hal
implementations build on.

Key facts:

- Crate name: `mimxrt685s-pac`
- Edition: 2021
- MSRV: **1.76** (declared in `Cargo.toml` and enforced in CI)
- License: MIT
- Default features: none beyond the generated peripheral modules
- Optional features:
  - `rt` — enable `cortex-m-rt` device support and link in `device.x`
  - `defmt` — derive `defmt::Format` on register enums/values
  - `debug` — enable `Debug` impls (large; gated to keep release builds lean)
- Primary target: `thumbv8m.main-none-eabihf`

The vast majority of `src/` is **machine-generated** by `svd2rust` from an
SVD description of the chip. See §3.

---

## 2. Repository layout

```
.
├── AGENTS.md                   ← you are here
├── Cargo.toml                  ← crate metadata, features, MSRV
├── build.rs                    ← copies device.x into OUT_DIR when `rt` is on
├── device.x                    ← cortex-m-rt interrupt vector defaults
├── deny.toml                   ← cargo-deny configuration
├── README.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── SECURITY.md
├── LICENSE
├── CODEOWNERS
├── .github/
│   ├── copilot-instructions.md ← legacy AI instructions (points to AGENTS.md)
│   └── workflows/
│       ├── check.yml           ← fmt, clippy, semver, doc, hack, deny, msrv
│       └── nostd.yml           ← cross-compile check for thumbv8m
├── svd/
│   └── MIMXRT685S_cm33.svd     ← upstream NXP SVD (vendor source of truth)
└── patch/
    ├── MIMXRT685S_cm33.yaml    ← top-level svdtools patch (lists includes)
    ├── adc.yaml
    ├── crc.yaml
    ├── ctimer.yaml
    ├── dma.yaml
    ├── i2s.yaml
    ├── inputmux.yaml
    └── usart.yaml
```

`src/` is intentionally **not** listed file-by-file: it is generated code (one
module per peripheral plus `lib.rs` and `generic.rs`). Do not memorize it; do
not edit it directly. See §3 and §4.

---

## 3. Source of truth: SVD + svdtools patches

The PAC is generated. The pipeline is:

```
svd/MIMXRT685S_cm33.svd            (vendor description from NXP)
        │
        ▼  svdtools patch patch/MIMXRT685S_cm33.yaml
svd/MIMXRT685S_cm33.svd.patched    (corrected description, gitignored)
        │
        ▼  svd2rust ...
lib.rs                              (one giant file)
        │
        ▼  form -i lib.rs -o src
src/**/*.rs                         (split into one module per peripheral)
        │
        ▼  cargo fmt
src/**/*.rs                         (committed)
```

**The authoritative inputs are:**

1. `svd/MIMXRT685S_cm33.svd` — the upstream NXP System View Description.
   Treat this as a vendor artifact. Do not hand-edit it except to fix a
   demonstrable upstream bug; prefer expressing fixes as a patch in
   `patch/`.
2. `patch/*.yaml` — [`svdtools`](https://github.com/rust-embedded/svdtools)
   patches that correct, enrich, or rename items in the SVD. This is where
   chip-specific corrections live.
3. `device.x` — linker symbols providing default ISR handlers for
   `cortex-m-rt`. Hand-maintained, but tightly coupled to the interrupt
   table generated from the SVD.
4. `build.rs` — copies `device.x` into `OUT_DIR` when the `rt` feature is
   enabled.

Everything in `src/` is **derived output**. The README's "Regenerating the
PAC" section is the canonical regeneration recipe.

### ⚠️ Do not hand-edit generated files

Files under `src/` (including `src/lib.rs`, `src/generic.rs`, every
`src/<peripheral>.rs`, and every `src/<peripheral>/**/*.rs`) are produced
by `svd2rust` + `form` + `cargo fmt`. **Any direct edit will be silently
overwritten** the next time the PAC is regenerated.

If you need to change generated code:

- **Register/field is wrong or missing** → edit the relevant
  `patch/*.yaml`, regenerate, commit both the patch and the regenerated
  `src/`.
- **A peripheral is wrong** → add or extend a `patch/<peripheral>.yaml`
  (and reference it from `patch/MIMXRT685S_cm33.yaml`'s `_include` list).
- **An svd2rust codegen quirk** → upstream the fix to `svd2rust` or
  `svdtools`; do not paper over it in `src/`.
- **Interrupt handler symbol missing** → edit `device.x`.

If a change *must* be made directly in `src/` (e.g., to unblock a release
while a patch is in flight), call it out loudly in the commit message and
open a follow-up issue to move the change into a patch.

### Regeneration commands

From the repository root, on a unix-like shell:

```sh
svdtools patch patch/MIMXRT685S_cm33.yaml
svd2rust -i svd/MIMXRT685S_cm33.svd.patched \
         --reexport-interrupt --ignore-groups \
         --impl-defmt defmt --impl-debug --impl-debug-feature debug
rm -r src/*
form -i lib.rs -o src
rm lib.rs
cargo fmt
```

On Windows (PowerShell or cmd), substitute `\` for `/` and run `dos2unix`
on the generated files so they match the LF convention used in the
repository:

```powershell
svdtools.exe patch patch\MIMXRT685S_cm33.yaml
svd2rust.exe -i svd\MIMXRT685S_cm33.svd.patched `
             --reexport-interrupt --ignore-groups `
             --impl-defmt defmt --impl-debug --impl-debug-feature debug
Remove-Item -Recurse src\*
form -i lib.rs -o src
Remove-Item lib.rs
cargo fmt
cd src
dos2unix **\*.rs *.rs
cd ..
```

Required tool versions match the comment generated into `src/lib.rs`
(currently `svd2rust v0.35.0`). Bumping the generator is a deliberate
change; expect large diffs and verify CI carefully.

---

## 4. What changes are in scope

| Change kind                              | Where it goes                              |
|------------------------------------------|--------------------------------------------|
| Add/correct a register, field, enum      | `patch/<peripheral>.yaml`, then regenerate |
| Add a new peripheral patch file          | new `patch/<name>.yaml` + include it in `patch/MIMXRT685S_cm33.yaml` |
| Add/remove an interrupt handler default  | `device.x`                                 |
| Add or change a crate feature            | `Cargo.toml` (mind feature-powerset CI)    |
| Bump MSRV                                | `Cargo.toml` `rust-version` **and** `msrv` matrix in `.github/workflows/check.yml` |
| Update svd2rust / regenerate             | regenerate per §3; commit `src/` diff in a single commit |
| Tooling / CI                             | `.github/workflows/*.yml`, `deny.toml`     |
| Documentation                            | `README.md`, this file, doc-comments in non-generated source |

Out of scope for this crate (belongs in a downstream HAL):

- Any high-level driver logic (clocks, pin muxing helpers, DMA orchestration)
- `embedded-hal` trait implementations
- Async runtimes or executors

---

## 5. Build, test, lint commands

All commands are run from the repository root.

### Format

```sh
cargo fmt --check        # CI gate
cargo fmt                # apply
```

There is no `rustfmt.toml`; defaults are used. Generated code is formatted
as part of regeneration (§3), so any `cargo fmt` change to `src/` outside
of a regeneration commit is a smell.

### Clippy

```sh
cargo clippy
```

CI runs clippy via `giraffate/clippy-action` on both `stable` and `beta`.
**Note:** the generated crate currently emits thousands of warnings (mostly
lifetime-elision cosmetic warnings from `svd2rust`). CI does not deny
warnings, and neither should you locally unless you are fixing the
generator. Do not chase clippy fixes inside `src/` by hand.

### Cross-compile (no-std)

```sh
rustup target add thumbv8m.main-none-eabihf
cargo check --target thumbv8m.main-none-eabihf --no-default-features
```

This mirrors `.github/workflows/nostd.yml` and is the most representative
sanity check for downstream consumers.

### Feature powerset

```sh
cargo install cargo-hack
cargo hack --feature-powerset check
```

Required because features must remain **additive** (no feature combination
may break the build). New features must be exercised here before merging.

### MSRV

```sh
rustup toolchain install 1.76
cargo +1.76 check
```

If you change the MSRV in `Cargo.toml`, also update the `msrv` matrix in
`.github/workflows/check.yml`.

### Docs

```sh
cargo +nightly doc --no-deps --all-features
```

Set `RUSTDOCFLAGS=--cfg docsrs` to match docs.rs locally.

### Supply chain

```sh
cargo install cargo-deny
cargo deny --all-features check
```

Configuration lives in `deny.toml`.

### Tests

This crate currently has no `#[test]`s — it is a pure PAC and is exercised
by downstream HALs. `cargo test` will compile the crate but run zero
tests. Do not add host-side tests against MMIO; they cannot work.

---

## 6. Coding conventions

For the small amount of hand-written code in this repo:

- Rust 2021 edition, `#![no_std]` at the crate root.
- Keep `build.rs` minimal; it must remain `std`-only and have no runtime
  dependencies beyond `std`.
- Public items must compile under MSRV 1.76. Avoid newer language/library
  features.
- `device.x`: one `PROVIDE(<NAME> = DefaultHandler);` per interrupt, in the
  order they appear in the SVD interrupt table. The names must exactly
  match those `extern "C"` declared in the generated `src/lib.rs`.
- No `unsafe` outside what `svd2rust` generates.
- No new dependencies without a justification in the PR description; PACs
  are dependency-sensitive (they propagate to every downstream HAL).

### Line endings

- The repository uses **LF** line endings throughout.
- `git config core.autocrlf false` is the expected local configuration.
- On Windows, regeneration must end with `dos2unix` (see §3) to keep the
  generated `src/` files LF.
- There is no `.gitattributes`; rely on the configuration above.

### Encoding

- All source files are UTF-8 without BOM.

---

## 7. Commit conventions

(These are normative for both human and AI contributors. The
`.github/copilot-instructions.md` file is a subset and points here.)

- **Subject line:** capitalized, ≤ 50 characters, imperative mood
  (“Fix bug”, not “Fixed bug” or “Fixes bug”).
- **Blank line** between subject and body.
- **Body wrapped at 72 columns.**
- The body explains **what** and **why**, not **how**. The diff already
  shows the how.
- Reference issues with `Refs: #N` or `Fixes: #N` trailers.
- One logical change per commit. Regeneration commits (`src/**`) should be
  separate from patch edits (`patch/**`) when feasible — but if a patch
  necessarily implies the regenerated diff, a single commit that contains
  both is acceptable and preferable to a half-regenerated tree.
- **Never** commit `svd/*.svd.patched` (it is `.gitignore`d) or anything
  under `target/`.
- Do not commit `Cargo.lock`; this is a library crate and the lockfile is
  `.gitignore`d on purpose.

### AI attribution (mandatory)

Every commit that includes AI-generated or AI-assisted work **must**
contain an `Assisted-by` trailer:

```
Assisted-by: AGENT_NAME:MODEL_VERSION [TOOL1] [TOOL2]
```

- `AGENT_NAME`: the AI tool/framework (e.g., `GitHub Copilot`,
  `GitHub Copilot CLI`).
- `MODEL_VERSION`: the exact model version used
  (e.g., `claude-opus-4.7`, `gpt-5.2`). **Verify your own identity before
  composing this trailer.** Do not copy a model name from a previous
  session or another project.
- Optional bracketed items list specialized analysis tools
  (e.g., `coccinelle`, `sparse`, `clang-tidy`). **Do not** list everyday
  development tools (git, cargo, editors, rust-analyzer).

### Sign-off

AI agents **must not** add `Signed-off-by` trailers. The DCO can only be
certified by a human contributor.

### Authorship

When an AI agent makes commits on behalf of a human, set author identity
**per-commit** with `git -c user.name=... -c user.email=...`. Do not
modify the global or repository `user.name`/`user.email` config.

Example:

```sh
git -c user.name="Felipe Balbi" \
    -c user.email="felipe.balbi@microsoft.com" \
    commit -m "Subject line

Body...

Assisted-by: GitHub Copilot:claude-opus-4.7"
```

---

## 8. Branching and pull requests

- Default branch: `main`.
- Work on a topic branch (`<area>-<short-description>`); never push to
  `main` directly.
- Keep PRs focused. A regeneration PR should not also reformat unrelated
  files.
- Rebase, do not merge, when bringing in `upstream/main` updates.
- **Never force-push** to a branch that has open review activity. For
  agent-driven workflows, prefer `git push` (no `--force`/`--force-with-lease`)
  unless explicitly instructed.
- AI agents pushing to a personal fork **must not** open the PR
  automatically unless explicitly told to.

### Required CI checks (from `.github/workflows/`)

A PR is considered ready when all of these are green:

- `check / stable / fmt`
- `check / stable / clippy` and `check / beta / clippy`
- `check / semver`
- `check / nightly / doc`
- `check / ubuntu / stable / features` (cargo-hack feature powerset)
- `check / ubuntu / stable / deny` (cargo-deny)
- `check / ubuntu / 1.76` (MSRV)
- `no-std / thumbv8m.main-none-eabihf`

Reproduce these locally with the commands in §5 before pushing.

---

## 9. Embedded constraints

This crate is consumed from `#![no_std]` firmware. That implies:

- No `std` in the public API or in `build.rs`-emitted Rust code.
- No allocator assumptions; nothing in this crate may require `alloc`.
- No `println!`, `eprintln!`, `dbg!`, or `std::*`. Use `defmt` (gated by
  the `defmt` feature) when logging is needed in hand-written code.
- Public APIs must remain object-safe and `#[inline]`-friendly; the
  generator already produces `#[inline(always)]` accessors and that is
  intentional — do not "simplify" them.
- The `rt` feature pulls in `cortex-m-rt/device` and is mutually
  compatible with downstream HALs that also enable `cortex-m-rt`.

---

## 10. Versioning and releases

- Semver applies. `cargo-semver-checks` runs in CI on every PR.
- A regeneration that adds/renames registers is a **breaking change**;
  bump the major (or, pre-1.0, the minor) version.
- A patch that only adds documentation or `#[derive(defmt::Format)]`
  coverage is typically a minor bump.
- Update `Cargo.toml` `version` in the same PR as the regeneration that
  motivates the bump.
- Release tagging follows `vMAJOR.MINOR.PATCH` on `main` after the
  version-bump PR merges. (Release automation is not yet wired up; do
  this by hand or via `cargo release` locally.)

---

## 11. Common agent workflows

### Workflow A — "Fix a wrong register definition"

1. Identify the peripheral and field. Confirm against the NXP reference
   manual (RM68S, public version) and the SVD.
2. Add or amend the relevant entry in `patch/<peripheral>.yaml`. If the
   peripheral has no patch file yet, create one and add it to
   `_include:` in `patch/MIMXRT685S_cm33.yaml`.
3. Regenerate (§3).
4. Verify locally: `cargo fmt --check`, `cargo check`,
   `cargo check --target thumbv8m.main-none-eabihf --no-default-features`,
   `cargo hack --feature-powerset check`.
5. Commit: one commit with the patch + regenerated `src/` diff, subject
   like `Fix <PERIPH> <REG>.<FIELD> width`, body explaining what the RM
   says and why the SVD was wrong.

### Workflow B — "Add an interrupt"

1. Confirm the new interrupt is present in the SVD (or add it via a patch).
2. Regenerate (§3) — the `extern "C"` block in `src/lib.rs` and the
   `Interrupt` enum will update automatically.
3. Add a matching `PROVIDE(<NAME> = DefaultHandler);` line to `device.x`,
   preserving SVD order.
4. Run `cargo check --features rt` and the no-std cross-check.
5. Commit.

### Workflow C — "Bump svd2rust"

1. Regenerate with the new generator (§3). Expect large diffs.
2. Read the svd2rust CHANGELOG; some changes affect downstream HALs
   (e.g., `unsafe` accessor signatures). Reflect any deliberate behavioral
   changes in the crate version bump and the changelog (if/when one is
   added).
3. Run the **full** CI command set locally (§5), including
   `cargo +1.76 check`.
4. Two commits preferred: one to update the toolchain pin / docs, one for
   the regenerated `src/`.

### Workflow D — "Edit documentation only"

1. Edit `README.md`, `AGENTS.md`, or doc comments.
2. `cargo doc --no-deps --all-features` to ensure docs still build.
3. Commit. No regeneration needed.

---

## 12. Things that look like bugs but aren't

- **Thousands of clippy/rustc warnings** under `src/`: these are from the
  current `svd2rust` output and are expected. Don't "fix" them by editing
  generated code.
- **`Cargo.lock` missing**: deliberate; this is a library crate.
- **`svd/MIMXRT685S_cm33.svd.patched` missing**: it's an intermediate
  artifact, gitignored.
- **`device.x` not used in a regular build**: it is only linked when the
  `rt` feature is enabled.
- **`exclude = [ "svd/*", "patch/*" ]` in `Cargo.toml`**: keeps the
  published crate small; the generation inputs are not shipped to
  crates.io. Don't remove this without a very good reason.

---

## 13. Quick reference

| Task                       | Command                                                                 |
|----------------------------|-------------------------------------------------------------------------|
| Format check               | `cargo fmt --check`                                                     |
| Clippy                     | `cargo clippy`                                                          |
| Host build                 | `cargo check`                                                           |
| No-std build               | `cargo check --target thumbv8m.main-none-eabihf --no-default-features`  |
| Feature powerset           | `cargo hack --feature-powerset check`                                   |
| MSRV check                 | `cargo +1.76 check`                                                     |
| Docs                       | `cargo +nightly doc --no-deps --all-features`                           |
| Supply chain               | `cargo deny --all-features check`                                       |
| Regenerate PAC             | see §3                                                                  |

---

## 14. When in doubt

- If a question is about an individual register or bit field, the **SVD +
  patches** are authoritative over the generated Rust.
- If a question is about commit hygiene or AI attribution, **this file**
  is authoritative over `.github/copilot-instructions.md`.
- If a question is about build/test commands, the **workflows in
  `.github/workflows/`** are authoritative; this file should match them
  and is updated when they change.
- If you are about to hand-edit anything under `src/`, stop and re-read
  §3 and §4.

## Model selection & cost discipline

Premium models (Opus, GPT-5 family, "high"/"xhigh" reasoning variants)
cost an order of magnitude more than standard models (Sonnet, Haiku,
mini). Most steps in a typical task do not need premium reasoning,
and over-using premium models wastes credits without improving
outcomes. The rules below apply to *all* model selection: your own
session, sub-agents launched via the `task` tool, and parallel work
launched via `/fleet`.

### Default posture

- **Default to the cheapest model that can do the job.** Reach for a
  premium model only when one of the escalation triggers below is hit.
- **Plan with premium, execute with cheap.** Spend at most one or two
  premium turns on design / planning, then downshift to a cheaper
  model for mechanical execution of the plan.
- **Never bump the model "just in case."** If you cannot articulate
  *why* a cheaper model would fail, use the cheaper model.

### Escalation triggers (use a premium model)

Reach for a premium model when *any* of these are true:

- Cross-module refactor, architectural design, or API design from
  scratch.
- Subtle correctness reasoning: concurrency, lifetimes, `unsafe`,
  FFI ABI, cryptography, safety-critical control paths.
- Debugging a failure that survived one prior cheap-model attempt.
- Reviewing code on a safety-, security-, or money-critical path.
- The diff cannot be predicted in advance — i.e. there is genuine
  creative or design work to do, not just typing.

### De-escalation triggers (use a cheap model)

Use the cheapest available model when *any* of these are true:

- Searching, reading, summarising files or docs.
- Single-file mechanical edits: rename, format, lint fix, dependency
  bump, boilerplate, scaffolding from a known template.
- Generating tests for code that already works.
- Running builds, tests, linters, or other commands where the model
  only needs to report success/failure.
- Routine commits, PR descriptions, changelog entries.
- The diff is essentially predictable before generation.

### Sub-agent routing (the `task` tool)

When delegating with the `task` tool, set `model:` explicitly. Do not
let sub-agents inherit a premium default for cheap work.

| Sub-agent type    | Default model             | Override to                                     |
|-------------------|---------------------------|-------------------------------------------------|
| `explore`         | cheap                     | keep cheap (`claude-haiku-4.5` or `gpt-5-mini`) |
| `task` (run cmd)  | cheap                     | keep cheap                                      |
| `research`        | cheap for breadth         | premium only for the final synthesis            |
| `general-purpose` | match task                | cheap for mechanical work; premium for design   |
| `rubber-duck`     | premium                   | keep premium — this is where reasoning pays off |
| `code-review`     | premium on critical paths | cheap on cosmetic / mechanical diffs            |

### `/fleet` (parallel sub-agents) rules

- Fleet mode multiplies cost by the fleet width. Apply the rules
  above *per worker*, not in aggregate.
- Split a fleet job along complexity lines: route the cheap,
  parallelisable workers (file edits, test runs, doc updates) to a
  cheap model; reserve premium models for the small number of
  workers that need real reasoning.
- If every worker in a fleet would need a premium model, the work is
  probably not a good fit for fleet mode — reconsider the
  decomposition before paying N× premium.

### Session hygiene

- Keep sessions short and focused. Long premium sessions are the
  single largest source of waste because every turn re-processes the
  full history.
- Use `/compact` when the conversation grows long, and `/new` for
  unrelated work.
- Prefer `/ask` for one-off side questions so they don't extend the
  main session.

### When in doubt

Ask: *"If a cheaper model produced the wrong answer here, would I
catch it in seconds (compiler, tests, my own review) or in
weeks (production incident)?"* If the former, use the cheap model
and let the feedback loop do its job.
