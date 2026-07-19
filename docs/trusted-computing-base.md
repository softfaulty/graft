# trusted computing base

graft has two privileged components with separate lifetimes. the recovery patcher changes the installed system while android is offline. `graftd` brokers root sessions after android boots. everything else is a client or transport and receives no direct root authority.

```text
installation

signed graft payload
        |
        v
recovery patcher [privileged, offline]
        |
        +-- inspect source artifacts read-only
        +-- build and verify staged artifacts
        `-- flash only the verified result
                    |
                    v
          platform partitions and avb chain

runtime

su -----------+
graftctl -----+---- local socket ----> graftd [privileged]
manager ------+                         |
                                        `---- authorized root session
remote ssh --> graft-sshd -------------^
               [network-facing,
                no direct root authority]
```

## component boundaries

| component        | privilege boundary                                | one responsibility                                                              |
| ---------------- | ------------------------------------------------- | ------------------------------------------------------------------------------- |
| recovery patcher | privileged in recovery; absent after installation | stage, verify, and flash the declared system changes as one offline transaction |
| `graftd`         | privileged Android service; no network listener   | authorize and create root sessions from kernel-verified local peers             |
| `su`             | unprivileged local client                         | translate conventional `su` usage into one `graftd` session request             |
| `graftctl`       | unprivileged administration client                | request status and explicit administrative actions from `graftd`                |
| manager          | unprivileged Android app                          | present authorization state and return authenticated user decisions             |
| `graft-sshd`     | separate network-facing process                   | authenticate SSH clients and forward approved session requests to `graftd`      |

`su`, `graftctl`, the manager, and `graft-sshd` cannot change uid, launch a root child, edit policy, or write platform partitions themselves. compromising one client must not grant more authority than the request identity and policy accepted by `graftd`.

android's kernel, init, selinux enforcement, verified-boot implementation, and the active recovery environment remain external foundations of this model. graft narrows its own boundary; it does not replace those trust roots.
