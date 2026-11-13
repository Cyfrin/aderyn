# Aderyn releases are retired

Aderyn is no longer maintained. Do not cut or publish new releases, push release
tags, or re-enable publishing automation.

Homebrew and npm publishing jobs have been removed from the distribution
configuration. The `cargo patch` and `cargo minor` shortcuts have also been
removed. Historical release instructions remain available in Git history.

The remaining distribution tooling is retained for reference; its presence does
not indicate that releases are supported. Repository administrators must keep
release automation disabled, including workflows on historical branches and
tags.

Keep the Homebrew tap repository and administrator access available while
retiring its formula. Identify the owner and any shared uses of publishing
credentials before revoking them.
