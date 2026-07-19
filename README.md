# graft

graft is a system image patcher and native android root service, it inspects an existing android installation, prepares and verifies the required platform changes and then installs `graftd` as a small privileged session broker for local tools such as `su` and `graftctl`. the installer handles the system integration; the daemon handles explicit, auditable root sessions after boot
