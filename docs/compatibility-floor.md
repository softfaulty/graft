# initial compatibility floor

the first graft runtime targets 64-bit arm android devices running android 10 or newer. this is an engineering floor, not a claim that every device above it is installable.

| capability        | initial requirement                                           | outside the first target                          |
| ----------------- | ------------------------------------------------------------- | ------------------------------------------------- |
| cpu and userspace | arm64 with the `arm64-v8a` abi                                | 32-bit arm, x86 and x86_64                        |
| android version   | android 10 / api 29 or newer                                  | android 9 and older                               |
| boot state        | bootloader unlocked with an identified write path             | locked or unverifiable boot state                 |
| selinux           | enforcing with a known policy integration path                | permissive-only bring-up or unknown policy inputs |
| partitions        | fully detected slot and partition topology                    | guessed targets or unresolved snapshot state      |
| filesystems       | a detected filesystem with a verified graft backend           | unknown or unsupported filesystem features        |
| verified boot     | every affected avb descriptor and chain is understood         | partial or unresolved verification metadata       |
| workspace         | enough non-live storage to stage outputs and restore material | in-place patching or unverified temporary storage |

meeting the floor only allows probing to continue. installation still requires a complete plan, a supported backend for every affected artifact and the verification gates in the [installer transaction contract](installer-transaction.md).

## older android later

older android versions and other architectures may be added through explicit platform adapters after the first target works. shared protocol, policy and image code must not depend on api 29 behavior when the dependency belongs in the android platform boundary. unsupported capabilities return a refusal; they do not fall through to the nearest-looking implementation.

## experimental distribution policy

while graft is experimental, development is source-only. the project does not publish prebuilt installers, device support sheets or compatibility claims. exact test hardware and rom details remain private until a complete evidence record reaches the intended public support grade.
