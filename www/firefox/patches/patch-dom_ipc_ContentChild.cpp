$NetBSD$

--- dom/ipc/ContentChild.cpp.orig	Mon Jul 20 22:49:37 2020
+++ dom/ipc/ContentChild.cpp
@@ -115,9 +115,7 @@
 #include "GMPServiceChild.h"
 #include "nsIStringBundle.h"
 #include "Geolocation.h"
-#include "audio_thread_priority.h"
 #include "nsIConsoleService.h"
-#include "audio_thread_priority.h"
 #include "nsIURIMutator.h"
 #include "nsIInputStreamChannel.h"
 #include "nsFocusManager.h"
@@ -1621,10 +1619,6 @@ mozilla::ipc::IPCResult ContentChild::RecvSetProcessSa
   sandboxEnabled = SandboxInfo::Get().CanSandboxContent();
 
   if (StaticPrefs::media_cubeb_sandbox()) {
-    // This needs to happen regardless of whether sandboxing is enabled.
-    if (atp_set_real_time_limit(0, 48000)) {
-      NS_WARNING("could not set real-time limit at process startup");
-    }
     InstallSoftRealTimeLimitHandler();
   } else if (sandboxEnabled) {
     // Pre-start audio before sandboxing; see bug 1443612.
