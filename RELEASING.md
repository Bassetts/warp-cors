# Releasing warp-cors

Release tags match the version in `Cargo.toml` without a `v` prefix, for example
`0.4.0`.

## New releases

1. Update the package version and lockfile, commit the changes, push them to
   `main`, and wait for CI to pass.
2. Create and push a signed tag for that commit:

   ```shell
   git tag -s 0.4.1 -m "Release 0.4.1"
   git push origin 0.4.1
   ```

3. Wait for the `release` workflow to finish building the binary archives and
   creating the draft GitHub release.
4. Open the draft in **Releases**, review its notes and attached archives, and
   publish it.
5. Wait for the `docker` and `publish` workflows to complete successfully.
6. Confirm the new version is available on Docker Hub and crates.io.

## Publishing an existing tag

1. Open **Actions → publish → Run workflow**.
2. Select `main`, enter the existing tag (for example, `0.4.0`) for `tag`, leave
   `dry_run` checked, and run the workflow.
3. After validation passes, run it again with the same tag and `dry_run` unchecked
   to publish that tag's source to crates.io.
4. Wait for the workflow to complete successfully and confirm the version is
   available on crates.io.
