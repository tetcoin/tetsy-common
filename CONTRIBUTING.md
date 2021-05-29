# Contributing to tetsy-common

tetsy-common welcomes contribution from everyone in the form of suggestions, bug
reports, pull requests, and feedback. This document gives some guidance if you
are thinking of helping us.

Please reach out here in a GitHub issue or in the tetsy channel on [gitter] if we can do anything to help you contribute.

[gitter]: https://gitter.im/tetcoinorg/community

## Submitting bug reports and feature requests

When reporting a bug or asking for help, please include enough details so that
the people helping you can reproduce the behavior you are seeing. For some tips
on how to approach this, read about how to produce a [Minimal, Complete, and
Verifiable example].

[Minimal, Complete, and Verifiable example]: https://stackoverflow.com/help/mcve

When making a feature request, please make it clear what problem you intend to
solve with the feature, any ideas for how tetsy-common could support solving that problem, any possible alternatives, and any disadvantages.

## Versioning

As many crates in the rust ecosystem, all crates in tetsy-common follow [semantic versioning]. This means bumping PATCH version on bug fixes that don't break backwards compatibility, MINOR version on new features and MAJOR version otherwise (MAJOR.MINOR.PATCH). Versions < 1.0 are considered to have the format 0.MAJOR.MINOR, which means bumping MINOR version for all non-breaking changes.

If you bump a dependency that is publicly exposed in a crate's API (e.g. `pub use dependency;` or `pub field: dependency::Dependency`) and the version transition for the dependency was semver-breaking, then it is considered to be a breaking change for the consuming crate as well. To put it simply, if your change could cause a compilation error in user's code, it is a breaking change.

Bumping versions should be done in a separate from regular code changes PR.

[semantic versioning]: https://semver.org/

## Releasing a new version

This part of the guidelines is for tetsy-common maintainers.

When making a new release make sure to follow these steps:
* Submit a PR with a version bump and list all major and breaking changes in the crate's changelog

After the PR is merged into master:
* `cargo publish` on the latest master (try with `--dry-run` first)
* Add a git tag in format `<crate-name>-v<version>`,
e.g. `git tag tetsy-impl-serde-v0.2.2` and push it with `git push origin tetsy-impl-serde-v0.2.2`

## Conduct

We follow [Tetcore Code of Conduct].

[Tetcore Code of Conduct]: https://github.com/tetcoin/tetcore/blob/master/CODE_OF_CONDUCT.adoc

## Attribution

This guideline is adapted from [Serde's CONTRIBUTING guide].

[Serde's CONTRIBUTING guide]: https://github.com/serde-rs/serde/blob/master/CONTRIBUTING.md
