$NetBSD$

--- linux-user/syscall.c.orig	Thu Apr 29 17:18:59 2021
+++ linux-user/syscall.c
@@ -7393,6 +7393,10 @@ static inline abi_long host_to_target_timex64(abi_long
 }
 #endif
 
+#ifndef sigev_notify_thread_id
+#define sigev_notify_thread_id _sigev_un._tid
+#endif
+
 static inline abi_long target_to_host_sigevent(struct sigevent *host_sevp,
                                                abi_ulong target_addr)
 {
@@ -7413,7 +7417,7 @@ static inline abi_long target_to_host_sigevent(struct 
     host_sevp->sigev_signo =
         target_to_host_signal(tswap32(target_sevp->sigev_signo));
     host_sevp->sigev_notify = tswap32(target_sevp->sigev_notify);
-    host_sevp->_sigev_un._tid = tswap32(target_sevp->_sigev_un._tid);
+    host_sevp->sigev_notify_thread_id = tswap32(target_sevp->_sigev_un._tid);
 
     unlock_user_struct(target_sevp, target_addr, 1);
     return 0;
