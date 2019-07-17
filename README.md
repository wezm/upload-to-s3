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

Download
--------

`upload-to-s3` is a single binary available for a number of platforms. The binary
has no runtime dependencies.

* [FreeBSD 12.0 amd64](https://releases.wezm.net/upload-to-s3/upload-to-s3-0.1.0-amd64-unknown-freebsd.tar.gz)
* [Linux x86\_64](https://releases.wezm.net/upload-to-s3/upload-to-s3-0.1.0-x86_64-unknown-linux-musl.tar.gz)
* [MacOS x86\_64](https://releases.wezm.net/upload-to-s3/upload-to-s3-0.1.0-x86_64-apple-darwin.tar.gz)
* [OpenBSD 6.5 amd64](https://releases.wezm.net/upload-to-s3/upload-to-s3-0.1.0-amd64-unknown-openbsd.tar.gz)

Download and extract along these lines:

    curl -L https://releases.wezm.net/upload-to-s3/upload-to-s3-0.1.0-x86_64-unknown-linux-musl.tar.gz | tar zxf -

Development
-----------

**Build Status:**

* [![builds.sr.ht Arch Linux status](https://builds.sr.ht/~wezm/upload-to-s3/arch.yml.svg)](https://builds.sr.ht/~wezm/upload-to-s3/arch.yml?) — Arch Linux
* [![builds.sr.ht Debian Linux status](https://builds.sr.ht/~wezm/upload-to-s3/arch.yml.svg)](https://builds.sr.ht/~wezm/upload-to-s3/debian.yml?) — Debian Linux
* [![builds.sr.ht FreeBSD status](https://builds.sr.ht/~wezm/upload-to-s3/freebsd.yml.svg)](https://builds.sr.ht/~wezm/upload-to-s3/freebsd.yml?) — FreeBSD
* [![Mac OS Build Status](https://api.cirrus-ci.com/github/wezm/upload-to-s3.svg)](https://cirrus-ci.com/github/wezm/upload-to-s3) — Mac OS
* [![builds.sr.ht OpenBSD status](https://builds.sr.ht/~wezm/upload-to-s3/openbsd.yml.svg)](https://builds.sr.ht/~wezm/upload-to-s3/openbsd.yml?) — OpenBSD

**Minimum Supported Rust Version:** 1.33.0

Contributing
------------

If you have code or patches you wish to contribute, the preferred mechanism is
a git pull request. Push your changes to a git repository somewhere (Sourcehut,
GitHub, GitLab, whatever) and then follow these steps:

Change the git URL to match wherever your forked repo is:

    git remote rename origin upstream
    git remote add origin git@git.sr.ht:~yourname/upload-to-s3
    git push -u origin master

Then generate the pull request:

    git fetch upstream master
    git request-pull -p upstream/master origin

And copy-paste the result into an [email to me](mailto:wes@wezm.net).

You may alternately use a patch-based approach as described on
<https://git-send-email.io>.

[rustup]: https://www.rust-lang.org/tools/install
