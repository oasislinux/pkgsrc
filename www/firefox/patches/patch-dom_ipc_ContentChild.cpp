$NetBSD$

--- dom/ipc/ContentChild.cpp.orig	Fri Apr  3 19:34:35 2020
+++ dom/ipc/ContentChild.cpp
@@ -111,9 +111,7 @@
 #include "GMPServiceChild.h"
 #include "nsIStringBundle.h"
 #include "Geolocation.h"
-#include "audio_thread_priority.h"
 #include "nsIConsoleService.h"
-#include "audio_thread_priority.h"
 #include "nsIURIMutator.h"
 #include "nsIInputStreamChannel.h"
 #include "nsFocusManager.h"
@@ -1804,9 +1802,6 @@ mozilla::ipc::IPCResult ContentChild::RecvSetProcessSa
   } else {
     // Pre-start audio before sandboxing; see bug 1443612.
     if (StaticPrefs::media_cubeb_sandbox()) {
-      if (atp_set_real_time_limit(0, 48000)) {
-        NS_WARNING("could not set real-time limit at process startup");
-      }
       InstallSoftRealTimeLimitHandler();
     } else {
       Unused << CubebUtils::GetCubebContext();
