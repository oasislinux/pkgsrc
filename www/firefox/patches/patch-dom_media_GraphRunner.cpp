$NetBSD$

--- dom/media/GraphRunner.cpp.orig	Fri Apr  3 19:34:35 2020
+++ dom/media/GraphRunner.cpp
@@ -13,7 +13,6 @@
 #include "nsISupportsImpl.h"
 #include "prthread.h"
 #include "Tracing.h"
-#include "audio_thread_priority.h"
 
 namespace mozilla {
 
@@ -95,9 +94,6 @@ auto GraphRunner::OneIteration(GraphTime aStateEnd, Gr
 }
 
 NS_IMETHODIMP GraphRunner::Run() {
-  atp_handle* handle =
-      atp_promote_current_thread_to_real_time(0, mGraph->GraphRate());
-
   nsCOMPtr<nsIThreadInternal> threadInternal = do_QueryInterface(mThread);
   threadInternal->SetObserver(mGraph);
 
@@ -119,9 +115,6 @@ NS_IMETHODIMP GraphRunner::Run() {
     mMonitor.Notify();
   }
 
-  if (handle) {
-    atp_demote_current_thread_from_real_time(handle);
-  }
 
   dom::WorkletThread::DeleteCycleCollectedJSContext();
 
