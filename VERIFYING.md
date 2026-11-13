# Verifying releases

Release archives are accompanied by `SHA256SUMS` and a signed provenance bundle,
`provenance.json`. Docker images have signed provenance attestations stored on
GitHub and Docker Hub. These are generated automatically using GitHub Actions
and Sigstore; no project signing key needs to be installed.

These instructions apply to releases with those verification assets. Older
releases, including `0.4.1`, do not have these checksums or attestations.

Checksums detect changed or incomplete downloads. Provenance verification checks
that the artifact was attested by this repository's expected release workflow
for the specified tag. It does not establish that the software is free of bugs
or vulnerabilities.

Use a current [GitHub CLI](https://cli.github.com/) and authenticate with
`gh auth login` before running `gh attestation verify`. The examples use
`RELEASE_VERSION` as a placeholder: replace it with the desired release tag,
without a `v` prefix.

## Binary archives

Download the archive for your platform and `SHA256SUMS` from the same
[GitHub release](https://github.com/Bassetts/warp-cors/releases) into an otherwise
empty directory. Verify both the checksum and provenance before extracting it.

### Linux and macOS

Set the version and archive filename to match your download:

```shell
version=RELEASE_VERSION
archive="warp-cors-${version}-x86_64-unknown-linux-musl.tar.gz"
```

On Linux, check the checksum with:

```shell
sha256sum --check --ignore-missing SHA256SUMS
```

On macOS, use:

```shell
shasum -a 256 --check --ignore-missing SHA256SUMS
```

The command must succeed and report your archive as `OK`. Missing archives for
other platforms are skipped.

Verify the archive's provenance:

```shell
gh attestation verify "$archive" \
  --repo Bassetts/warp-cors \
  --signer-workflow Bassetts/warp-cors/.github/workflows/release.yml \
  --source-ref "refs/tags/$version" \
  --deny-self-hosted-runners
```

### Windows (PowerShell)

```powershell
$version = "RELEASE_VERSION"
$archive = "warp-cors-$version-x86_64-pc-windows-msvc.zip"
$entry = Get-Content SHA256SUMS | Where-Object { $_.EndsWith("  $archive") }
if (@($entry).Count -ne 1) { throw "Expected one checksum for $archive" }
$expected = $entry.Split(' ')[0]
$actual = (Get-FileHash $archive -Algorithm SHA256).Hash
if ($actual -ne $expected) { throw "Checksum mismatch for $archive" }

gh attestation verify $archive `
  --repo Bassetts/warp-cors `
  --signer-workflow Bassetts/warp-cors/.github/workflows/release.yml `
  --source-ref "refs/tags/$version" `
  --deny-self-hosted-runners
if ($LASTEXITCODE -ne 0) { throw "Provenance verification failed" }
```

### Using the downloaded provenance bundle

By default, the CLI retrieves attestations from GitHub. To use the bundle
attached to the release instead, download `provenance.json` and add
`--bundle provenance.json` to the same verification command. Keep the repository,
workflow, and source-ref checks. This option replaces attestation retrieval;
fully offline verification also requires preparing trusted roots as described
in [GitHub's offline verification instructions](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/verify-attestations-offline).

## Docker images

Wait for the release's `docker` workflow to succeed. Use a versioned tag to look
up the image digest:

```shell
version=RELEASE_VERSION
docker buildx imagetools inspect "docker.io/bassetts/warp-cors:$version"
```

Copy the top-level `Digest: sha256:...` value from that output, then set `image`
using that exact digest. Replace `REPLACE_WITH_DIGEST` with its hexadecimal value:

```shell
image=docker.io/bassetts/warp-cors@sha256:REPLACE_WITH_DIGEST
docker login docker.io
gh attestation verify "oci://$image" \
  --repo Bassetts/warp-cors \
  --signer-workflow Bassetts/warp-cors/.github/workflows/docker.yml \
  --source-ref "refs/tags/$version" \
  --deny-self-hosted-runners
```

Only after verification succeeds, run that same digest:

```shell
docker run --rm -p 127.0.0.1:3030:3030 "$image"
```

Using the digest ensures you run the verified image even if a tag subsequently
changes. To retrieve the attestation from Docker Hub rather than GitHub, add
`--bundle-from-oci` to the verification command and retain the identity checks.
