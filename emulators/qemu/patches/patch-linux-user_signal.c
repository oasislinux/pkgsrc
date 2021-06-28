$NetBSD$

--- linux-user/signal.c.orig	Thu Apr 29 17:18:58 2021
+++ linux-user/signal.c
@@ -38,7 +38,9 @@ static void host_signal_handler(int host_signum, sigin
  * Signal number 0 is reserved for use as kill(pid, 0), to test whether
  * a process exists without sending it a signal.
  */
+#ifdef __SIGRTMAX
 QEMU_BUILD_BUG_ON(__SIGRTMAX + 1 != _NSIG);
+#endif
 static uint8_t host_to_target_signal_table[_NSIG] = {
     [SIGHUP] = TARGET_SIGHUP,
     [SIGINT] = TARGET_SIGINT,
