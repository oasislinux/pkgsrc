$NetBSD$

--- security/sandbox/linux/broker/SandboxBrokerPolicyFactory.cpp.orig	Thu Jan 23 02:45:30 2020
+++ security/sandbox/linux/broker/SandboxBrokerPolicyFactory.cpp
@@ -287,6 +287,7 @@ SandboxBrokerPolicyFactory::SandboxBrokerPolicyFactory
   policy->AddDir(rdonly, "/etc");
   policy->AddDir(rdonly, "/usr/share");
   policy->AddDir(rdonly, "/usr/local/share");
+  policy->AddDir(rdonly, "@PREFIX@");
   // Various places where fonts reside
   policy->AddDir(rdonly, "/usr/X11R6/lib/X11/fonts");
   policy->AddDir(rdonly, "/nix/store");
