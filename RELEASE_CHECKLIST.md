# Release Checklist

- Ensure local `main` is up to date with respect to `origin/master`.
- Update dependencies using `make update`
- Update the changelog as appropriate
- Edit the Cargo.toml to set the new Surrogate version.
- Push changes to GitHub, NOT including the tag.
- Once CI for master finishes successfully, push the version tag. (Trying to do this in one step seems to result in 
GitHub Actions not seeing the tag push and thus not running the release workflow.)
- Wait for CI to finish creating the release. If the release build fails, then delete the tag from GitHub, make fixes, re-tag, delete the release and push.
