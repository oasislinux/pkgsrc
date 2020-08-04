$NetBSD$

--- dom/media/UnderrunHandlerLinux.cpp.orig	Mon Jul 20 20:53:19 2020
+++ dom/media/UnderrunHandlerLinux.cpp
@@ -9,7 +9,6 @@
 
 #include <mozilla/Sprintf.h>
 #include <mozilla/Atomics.h>
-#include "audio_thread_priority.h"
 #include "nsDebug.h"
 
 namespace mozilla {
@@ -58,20 +57,6 @@ void InstallSoftRealTimeLimitHandler() {
 }
 
 void DemoteThreadFromRealTime() {
-  atp_thread_info* info = atp_get_current_thread_info();
-  if (!info) {
-    NS_WARNING("Could not get current thread info when demoting thread.");
-    return;
-  }
-  int rv = atp_demote_thread_from_real_time(info);
-  if (rv) {
-    NS_WARNING("Could not demote thread from real-time.");
-    return;
-  }
-  rv = atp_free_thread_info(info);
-  if (rv) {
-    NS_WARNING("Could not free atp_thread_info struct");
-  }
   gRealtimeLimitReached = false;
 }
 
