# Packages that are required in order to run the project

- This tutorial is based on that you have already installed rustup & cargo. If you haven't, I wouldn't recommend continuing this until you install both which can be installed really easily with a small web search.

- **Also this is for fedora only**. If you want for other arch/debian based distro(s), You can do a quick web search on "How to install <`package-name`> in <`distro`>", An example would be like: "How to install base-x in ubuntu", If you didn't find the first one, It should be `xorg-dev` in debian based distro(s)

## Packages:

We need to install some of packages, like cmake, x11, randr, inerma and so on...
And to do this you can type/copy all of the following commands, If everything got installed, 99.9% It will work.

### Commands:

```bash
$ sudo dnf install cmake @base-x

$ sudo dnf install libX11-devel libXrandr-devel libXinerama-devel

$ sudo dnf install libXcursor-devel libXi-devel

$ sudo dnf install alsa-lib-devel
```

*Note: I know that this all could've fitted into 1 singular command, But it wouldn't be comfortable for any dev's eye.*