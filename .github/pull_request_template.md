---

Thanks for taking the time !

Please ensure that some basic checks are passed before creating this Pull Request:

- [ ] Cargo Format
    - Run `cargo fmt` on the project.
    
- [ ] Clippy lints
    - Run `cargo clippy -- -Dwarnings` on the project to check for suggestions
    - Run `cargo clippy --fix` to let clippy apply the suggestions itself, if any.

This can be tiresome for frequent contributors, so it might would be better if
you used a git pre-push hook as mentioned in the [readme](readme.md#hacking).

The [Build CI](.github/workflow/buildci.yaml) will also check whether the project can be compiled and
cross-compiled to arm64 without any errors.
