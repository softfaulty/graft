# runtime failure contract

`graftd` fails closed. an unavailable dependency, ambiguous identity or damaged state can remove root access; it must never broaden root access.

## invariants

- only a complete, valid authorization decision may create a root session
- a failed administrative write has no partial in-memory effect
- a request is never retried as root after a client or daemon failure
- daemon-owned child processes do not outlive `graftd`
- safe mode never changes selinux enforcement or verified-boot policy
- recovery actions preserve evidence before replacing damaged state

## authorization failures

peer identity comes from the kernel-facing local socket, not request data. an unreadable identity, invalid protocol message, unsupported protocol version or inconsistent process metadata is denied before policy evaluation.

an explicit valid grant may authorize a request without the manager being online. if no stored rule decides the request, it may wait for the manager until its fixed deadline. manager disconnect, invalid response, mismatched request id or timeout ends in denial. pending requests are never converted into approval by a default rule.

an allow-once decision is consumed by one matching request. cancellation, launch failure or daemon restart does not make it available to a different request. a persistent decision becomes effective only after its atomic state write succeeds.

if the audit sink cannot durably record an authorization and session start, the new session is denied and the daemon enters safe mode. root that cannot be accounted for is just a quieter incident.

## safe mode

`graftd` enters safe mode when any security-critical startup input cannot be trusted, including:

- invalid configuration or installation identity
- unsupported or failed state migration
- corrupt grant, key or manager-pairing state
- an audit store that cannot be opened safely
- repeated daemon crashes during one boot

safe mode exposes only daemon status, version, the reason code and read-only diagnostic export over the local socket. it denies execution, grant changes, ssh session forwarding, module activation and every other privileged or persistent mutation.

restarting the daemon does not clear safe mode. normal operation resumes only after the failed input is repaired or restored and the complete startup validation passes.

## daemon crashes

all sessions belong to a daemon-managed process group with a kernel-enforced parent-death mechanism where the platform provides one. an unexpected daemon exit terminates every running session and closes every pending request. clients receive a disconnected result and must not automatically replay execution requests.

init may restart `graftd`. the restarted daemon reloads only durable state that passes full validation. pending approvals and allow-once decisions are gone. grants explicitly scoped until reboot remain bound to the current verified boot identity, not to a daemon process id.

repeated startup or runtime crashes during one boot trip the persistent crash-loop marker and start the next instance in safe mode.

## corrupt state

state parsing is all-or-nothing per versioned state object. invalid encoding, an unknown required field, a failed integrity check or an incomplete migration causes that object to be quarantined unchanged for diagnostics. graft never replaces corrupt authorization state with an empty allow policy.

corruption of grants, trusted keys, manager pairing, boot identity or audit metadata enters safe mode. unrelated optional state may be disabled only when its schema explicitly declares that failure non-security-critical. every quarantine action records the original path, hash and reason when the audit store is usable.

## deterministic results

| failure                                  | result                                                               |
| ---------------------------------------- | -------------------------------------------------------------------- |
| malformed or unauthenticated client      | close the request, deny execution and audit when possible            |
| manager unavailable or decision timeout  | deny the pending request                                             |
| session launch fails before exec         | report failure, consume request-specific approval and leave no child |
| persistent state write fails             | reject the change and keep the previous durable state                |
| audit becomes unavailable                | deny new sessions and enter safe mode                                |
| daemon crashes                           | terminate sessions, cancel pending requests and restart through init |
| crash loop detected                      | start in safe mode                                                   |
| security-critical state is corrupt       | quarantine it and start in safe mode                                 |
| unknown failure at a privileged boundary | deny the operation and enter safe mode                               |

## rescue behavior

runtime safe mode is diagnostic, not a hidden emergency root shell. repair happens offline through the recovery patcher using the exact installation manifest and restore material.

the recovery path may export diagnostics, restore a previously verified state generation, disable `graftd` startup or uninstall graft. it must not silently discard grants, weaken selinux, disable avb or fabricate a clean state. if no restore action can be verified, it leaves graft disabled and reports the device state as unresolved.
