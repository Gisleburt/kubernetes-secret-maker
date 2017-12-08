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

```bash
$ NAME=test SK_MY_SECRET="my secret" secret-maker | kubectl apply -f -
```

or with docker

```bash
$ docker run --rm -e NAME=test -e SK_MY_SECRET="my secret" secret-maker | kubectl apply -f -
```

added safety

```bash
$ HISTCONTROL=ignorespace
$  NAME=test SK_MY_SECRET="my secret" secret-maker | kubectl apply -f -
  ^ extra space
```

How it works
------------

```bash
$ NAME=test SK_MY_SECRET="my secret" secret-maker
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
