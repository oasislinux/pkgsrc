$NetBSD$

--- media/audioipc/server/src/lib.rs.orig	Wed Apr 29 21:00:35 2020
+++ media/audioipc/server/src/lib.rs
@@ -9,7 +9,6 @@ extern crate error_chain;
 #[macro_use]
 extern crate log;
 
-use audio_thread_priority::promote_current_thread_to_real_time;
 use audioipc::core;
 use audioipc::platformhandle_passing::framed_with_platformhandles;
 use audioipc::rpc;
@@ -64,12 +63,6 @@ fn run() -> Result<ServerWrapper> {
     let callback_thread = core::spawn_thread(
         "AudioIPC Callback RPC",
         || {
-            match promote_current_thread_to_real_time(0, 48000) {
-                Ok(_) => {}
-                Err(_) => {
-                    debug!("Failed to promote audio callback thread to real-time.");
-                }
-            }
             trace!("Starting up cubeb audio callback event loop thread...");
             Ok(())
         },
