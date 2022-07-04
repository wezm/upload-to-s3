upload-to-s3
============

Small binary that can upload a file to an Amazon S3 bucket.

This tool was built to allow uploading files to S3 from a continuous
integration pipeline. Pre-built binaries are provided for several platforms,
which can be downloaded as part of a CI build.

Usage
-----

`upload-to-s3` expects to retrieve AWS credentials from the following
environment variables:

* `AWS_ACCESS_KEY_ID`
* `AWS_SECRET_ACCESS_KEY`

It will also read values for these variables from a `.env` file if present.

Upload a file as follows:

    ./upload-to-s3 -b bucket.example.com some-file.tar.xz object/on/s3/some-file.tar.xz

For a more complete example of usage see [the CI configuration for this
repository][repo].


Download
--------

`upload-to-s3` is a single binary available for a number of platforms. The binary
has no runtime dependencies.

* [FreeBSD 12.1 amd64](https://releases.wezm.net/upload-to-s3/0.1.10/upload-to-s3-0.1.10-amd64-unknown-freebsd.tar.gz)
* [Linux x86\_64](https://releases.wezm.net/upload-to-s3/0.1.10/upload-to-s3-0.1.10-x86_64-unknown-linux-musl.tar.gz)
* [MacOS x86\_64](https://releases.wezm.net/upload-to-s3/0.1.10/upload-to-s3-0.1.10-x86_64-apple-darwin.tar.gz)
* [Windows x86\_64](https://releases.wezm.net/upload-to-s3/0.1.10/upload-to-s3-0.1.10-x86_64-pc-windows-msvc.zip)

Download and extract along these lines:

    curl https://releases.wezm.net/upload-to-s3/0.1.10/upload-to-s3-0.1.10-x86_64-unknown-linux-musl.tar.gz | tar zxf -

Development
-----------

**Build Status:** [![Build Status](https://api.cirrus-ci.com/github/wezm/upload-to-s3.svg)](https://cirrus-ci.com/github/wezm/upload-to-s3)

**Minimum Supported Rust Version:** 1.61.0

Publishing
----------

Notes on publishing a build:

    git tag -a x.y.z
    git push --tags origin $(current_branch)

CI will publish the binary if the build is successful. If successful, update
README with links to new binaries.

[rustup]: https://www.rust-lang.org/tools/install
[repo]: https://github.com/wezm/upload-to-s3
