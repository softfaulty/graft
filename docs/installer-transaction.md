# installer transaction contract

the installer is a staged transaction over one exact device state. it may inspect freely, but it must not write a live partition until every output and its restore material have passed verification.

```text
inspect -> plan -> stage -> verify -> flash -> confirm -> complete
                                   |         |
                                   |         `-> rollback on any failure
                                   `-> no partition writes are permitted here
```

## transaction record

every run has one durable transaction record containing:

- the device, build fingerprint, active slot and partition identities
- hashes of every source artifact
- the complete write plan and selected backend
- hashes, sizes and verification results for every staged output
- restore material for every target that may be changed
- the current phase and a journal of completed actions

the record and staged files live away from the source partitions. source artifacts are opened read-only. resuming a transaction requires the recorded device state and source hashes to still match.

## 1. inspect

inspection identifies the exact installed build, slot state, snapshot state, partition map, filesystems, selinux inputs, avb chain, recovery capabilities and available staging space. it does not mutate mounts, logical-partition metadata or source artifacts.

an unknown or contradictory result is unsupported. inspection must stop before planning rather than substitute a device default.

## 2. plan

planning produces the full transaction before any artifact is rebuilt. each planned write names:

- the exact target partition and slot
- the expected source identity and hash
- the patch backend and intended content changes
- the maximum permitted output size
- dependent selinux and avb changes
- the flashing and read-back verification method
- the restore artifact and rollback action

the plan is immutable after staging begins. a changed target, backend or dependency creates a new transaction.

## 3. stage

staging copies or rebuilds every output in the transaction workspace. it never patches the only source copy or writes through a mounted live filesystem. all cross-partition outputs are staged together, including policy and verification metadata.

before verification starts, the installer also creates complete restore material for every planned target and proves that it is readable and fits its original destination.

## 4. verify

verification checks the staged transaction as a set, not as unrelated files. the checks include:

- output hashes and sizes
- filesystem structure and backend-specific consistency
- the exact runtime payload, ownership, modes and init integration
- selinux policy compilation and expected labels or contexts
- sparse or logical-partition container validity where applicable
- avb descriptors, chains, signatures and partition size constraints
- agreement between every staged artifact and the immutable plan
- restore artifact hashes and destination mapping

successful verification writes a durable verified marker bound to the plan hash and every staged artifact hash. changing any bound file or plan field invalidates the marker.

**the flash phase has one entry condition: a valid verified marker for the current transaction. there is no force flag, debug shortcut or backend-specific exception.**

## 5. flash

immediately before the first write, the installer rechecks the device, active slot, snapshot state, target identities and source hashes. any drift invalidates verification and returns the transaction to inspection.

the flash backend may write only the targets declared by the verified plan. it verifies each input hash again, records intent before the write, records completion after the write and performs the planned read-back check. dependency roots such as vbmeta are committed only in the order declared by the verified plan.

multi-partition flashing is not physically atomic. the complete restore set and durable journal are therefore mandatory before the first write.

## 6. confirm

confirmation reopens every changed target and validates the flashed bytes, filesystem metadata, payload, policy outputs and avb relationships against the transaction record. it then writes a final installation manifest and marks the transaction complete.

reaching the end of a write command is not confirmation. until every target passes read-back verification, the installation remains incomplete and must not be reported as successful.

## 7. rollback

any failure after the first partition write enters rollback. rollback restores every changed target from the verified restore set in reverse plan order, read-back verifies each restoration and records the result in the same journal.

an interrupted transaction must be reconciled from its journal before a new install can start. if restoration cannot be verified, the installer stops in an explicit unsafe state, preserves all diagnostics and restore material, and does not claim that the device is bootable.

failures before the first partition write only discard staged outputs. the original installation remains untouched.
