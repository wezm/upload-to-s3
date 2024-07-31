upload-to-s3
============

Small binary that can upload a file to an Amazon S3 bucket.

This tool was built to allow uploading files to S3 from a continuous
integration pipeline. Pre-built binaries are provided for several platforms,
which can be downloaded as part of a CI build.

Usage
-----

```
Usage: upload-to-s3 [options] FILE OBJECT

Options:
    -b, --bucket NAME   S3 bucket
    -r, --region REGION S3 region (default: us-east-1)
    -h, --help          Print this help information
```

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
has no runtime dependencies. Pre-compiled binaries are available for:

* FreeBSD 14+ amd64
* Linux x86\_64
* MacOS Universal
* Windows x86\_64

Check the [latest release] for download links.

Download and extract along these lines:

    curl https://releases.wezm.net/upload-to-s3/0.2.0/upload-to-s3-0.2.0-x86_64-unknown-linux-musl.tar.gz | tar zxf -

Development
-----------

**Build Status:** [![Build Status](https://api.cirrus-ci.com/github/wezm/upload-to-s3.svg)](https://cirrus-ci.com/github/wezm/upload-to-s3)

**Minimum Supported Rust Version:** 1.79.0

Publishing
----------

Notes on publishing a build:

    git tag -a x.y.z
    git push --tags origin $(current_branch)

CI will publish the binary if the build is successful.

[rustup]: https://www.rust-lang.org/tools/install
[repo]: https://github.com/wezm/upload-to-s3
