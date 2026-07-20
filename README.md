# graft

graft is a system image patcher and native android root service, it inspects an existing android installation, prepares and verifies the required platform changes and then installs `graftd` as a small privileged session broker for local tools such as `su` and `graftctl`. the installer handles the system integration; the daemon handles explicit, auditable root sessions after boot

## support target

global support means one graft distribution that detects a device's actual capabilities and selects a verified patch and flash path, it does not mean every android device can be patched (sadly). graft targets unlocked devices where it can identify the partition layout, filesys, selinux policy inputs, affected avb chain, staging space and a safe flashing backend. a locked bootloader, an unknown layout, unresolved verification metadata or a recovery that cannot safely write the result is unsupported, in those cases graft must refuse the install or produce a verified artifact for an explicit fastbootd handoff.

## non-goals

graft does not:

- deliver privilege escalation or bootloader unlock exploits
- spoof platform integrity or verified boot state
- hide root or bypass root detection systems
- patch or maintain device kernels
- build or maintain a custom aosp distributions

## docs

- [initial compatibility floor](docs/compatibility-floor.md)
- [installer transaction contract](docs/installer-transaction.md)
- [repository hygiene](docs/repository-hygiene.md)
- [runtime failure contract](docs/runtime-failure-contract.md)
- [support grades](docs/support-grades.md)
- [trusted computing base](docs/trusted-computing-base.md)

## license

apache-2.0, see [LICENSE](LICENSE). bundled third-party attributions belong in [NOTICE](NOTICE).
