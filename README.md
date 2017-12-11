Kubernetes Secret Maker
=======================

**Problem:** 
- You want to put secrets inside your kubernetes cluster
- You want to do it in a clean repeatable way
- You don't want to leave those secrets lying around in files

**Solution:**
- Temporarily put the secrets in the environment
- Generate a secrets file on stdout
- Pipe it into `kubectl apply -f -`

Usage
-----

You can use this tool through Rust's Cargo:

```bash
$ cargo install kubesm
$ NAME=test SK_MY_SECRET="my secret" kubesm | kubectl apply -f -
```

or with Docker:

```bash
$ docker run --rm -e NAME=test -e SK_MY_SECRET="my secret" apolitical/kubesm | kubectl apply -f -
```

For added safety, you can tell your shell not to remember commands that started with a space.

```bash
$ HISTCONTROL=ignorespace
$  NAME=test SK_MY_SECRET="my secret" kubesm | kubectl apply -f -
  ^ extra space
```

How it works
------------

Provide a `NAME` for the resource that will be created, each secret should be prefixed with `SK_`, but this will be
removed when output. Here's what just the output would look like: 

```bash
$ NAME=test SK_MY_SECRET="my secret" kubesm
---
apiVersion: v1
kind: Secret
metadata:
  name: test
type: Opaque
data:
  MY_SECRET: bXkgc2VjcmV0
```

Limitations
-----------

Most of the above is statically stored in the binary. There's currently no flexability in the `apiVersion` or `type`

Contributing
------------

If you want to help, that's brilliant! Have a look at our [Contributing Guide](CONTRIBUTING.md). We also adhere to a 
[Code of Conduct](CODE_OF_CONDUCT.md), so please check that out, it includes details on who to contact if you have any
concerns. 
