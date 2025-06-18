# Setup

After cloning this repository, you need to initialize git submodules to populate the
`extern/ktx` linked repository. In theory you should be able to...

```bash
> git submodule init
```

... but if `extern/ktx` is still empty, you might need the more explicit...

```bash
> git submodule update --init --recursive
```

> Sorry, I'm too lazy to go read the docs and figure out what the difference is.
