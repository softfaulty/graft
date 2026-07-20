# support grades

graft support is an evidence record for one exact device and installed build. a device name by itself is never a support claim.

each record identifies:

- device codename, hardware variant and region where relevant
- full build fingerprint and security patch level
- slot, dynamic-partition and filesystem layout
- recovery or fastbootd implementation and version
- graft version or source revision
- test date, grade and evidence links
- the maintainer responsible for a `Maintained` record

an ota, repartition, recovery change or materially different regional variant creates a new support target. results do not inherit from a device that merely has a similar name.

## grades

| grade             | claim                                                                                | required evidence                                                                                                                                    |
| ----------------- | ------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Experimental`    | the target has probe data or active development, but no boot or installation promise | a sanitized probe report and a list of unknown or untested capabilities                                                                              |
| `Booted`          | a Graft-patched image has booted on this exact build                                 | captured build identity, enforcing selinux, expected `graftd` process and context, and a clean boot log                                              |
| `Root Works`      | the native runtime can create and deny audited root sessions on this exact build     | all `Booted` evidence plus successful `graftctl status`, `su -c id`, interactive session, denial and daemon-restart tests                            |
| `Installer Works` | the declared installation path safely installs and removes Graft on this exact build | all `Root Works` evidence plus probe, plan, staged verification, flash read-back, reboot, rollback or uninstall, and restored-state verification     |
| `Maintained`      | an identified maintainer currently revalidates releases for this exact target        | all `Installer Works` evidence, a named maintainer, current recovery instructions, known-good restore material and a dated passing release checklist |

grades are cumulative. a target cannot skip a lower grade because one later-looking command happened to work.

## reporting rules

- say `Experimental`, `Booted`, `Root Works`, `Installer Works` or `Maintained`; do not say vaguely supported
- publish the exact target identity and test date beside the grade
- list limitations and required handoffs beside the successful evidence
- treat missing evidence as untested, not as a pass
- demote or expire a grade when its build, toolchain or installation path can no longer be reproduced
- classify an unknown target as unsupported until probing produces a new record

only `Installer Works` and `Maintained` are installation-support claims. the lower grades are useful engineering milestones, not advice to flash a primary phone.
