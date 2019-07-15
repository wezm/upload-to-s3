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
depends on OpenSSL on FreeBSD and Linux.

<!-- http://releases.wezm.net.s3-website-us-east-1.amazonaws.com/ -->

* [FreeBSD 12.0 amd64][freebsd-bin]
* [Linux x86\_64][linux-bin]
* [macOS x86\_64][macos-bin]

<!--
* NetBSD 8.0 x86_64:
* OpenBSD 6.5 x86_64:
* Raspberry Pi:
-->

Development
-----------

**Build Status:**

* Arch Linux: [![builds.sr.ht Arch Linux status](https://builds.sr.ht/~wezm/upload-to-s3/arch.yml.svg)](https://builds.sr.ht/~wezm/upload-to-s3/arch.yml?)
* FreeBSD: [![builds.sr.ht FreeBSD status](https://builds.sr.ht/~wezm/upload-to-s3/freebsd.yml.svg)](https://builds.sr.ht/~wezm/upload-to-s3/freebsd.yml?)
* mac OS: [![builds.sr.ht Mac OS status](https://builds.sr.ht/~wezm/upload-to-s3/mac-os.yml.svg)](https://builds.sr.ht/~wezm/upload-to-s3/mac os.yml?)

**Minimum Supported Rust Version:** Latest Stable

Contributing
------------

If you have code or patches you wish to contribute, the preferred mechanism is
a git pull request. Push your changes to a git repository somewhere (Sourcehut,
GitHub, GitLab, whatever). <!--Ensure that contributions don't break [the
tests](#testing) and add new ones when appropriate.-->

Assuming you have followed the [build steps](#build) above you would do the
following to push to your own fork on Sourcehut or your preferred git host:

Change the git URL to match wherever your forked repo is:

    git remote rename origin upstream
    git remote add origin git@git.sr.ht:~yourname/upload-to-s3
    git push -u origin master

Then generate the pull request:

    git fetch upstream master
    git request-pull -p upstream/master origin

And copy-paste the result into a plain-text [email to me](mailto:wes@wezm.net).

You may alternately use a patch-based approach as described on
<https://git-send-email.io>.

[rustup]: https://www.rust-lang.org/tools/install
