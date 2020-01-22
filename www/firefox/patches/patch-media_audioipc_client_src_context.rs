$NetBSD$

--- media/audioipc/client/src/context.rs.orig	Fri Apr  3 19:34:57 2020
+++ media/audioipc/client/src/context.rs
@@ -6,10 +6,6 @@
 use crate::stream;
 use crate::{assert_not_in_callback, run_in_callback};
 use crate::{ClientStream, AUDIOIPC_INIT_PARAMS};
-#[cfg(target_os = "linux")]
-use audio_thread_priority::get_current_thread_info;
-#[cfg(not(target_os = "linux"))]
-use audio_thread_priority::promote_current_thread_to_real_time;
 use audioipc::codec::LengthDelimitedCodec;
 use audioipc::frame::{framed, Framed};
 use audioipc::platformhandle_passing::{framed_with_platformhandles, FramedWithPlatformHandles};
@@ -77,32 +73,7 @@ impl ClientContext {
     }
 }
 
-#[cfg(target_os = "linux")]
-fn promote_thread(rpc: &rpc::ClientProxy<ServerMessage, ClientMessage>) {
-    match get_current_thread_info() {
-        Ok(info) => {
-            let bytes = info.serialize();
-            // Don't wait for the response, this is on the callback thread, which must not block.
-            rpc.call(ServerMessage::PromoteThreadToRealTime(bytes));
-        }
-        Err(_) => {
-            warn!("Could not remotely promote thread to RT.");
-        }
-    }
-}
 
-#[cfg(not(target_os = "linux"))]
-fn promote_thread(_rpc: &rpc::ClientProxy<ServerMessage, ClientMessage>) {
-    match promote_current_thread_to_real_time(0, 48000) {
-        Ok(_) => {
-            info!("Audio thread promoted to real-time.");
-        }
-        Err(_) => {
-            warn!("Could not promote thread to real-time.");
-        }
-    }
-}
-
 fn register_thread(callback: Option<extern "C" fn(*const ::std::os::raw::c_char)>) {
     if let Some(func) = callback {
         let thr = thread::current();
@@ -117,13 +88,6 @@ fn unregister_thread(callback: Option<extern "C" fn()>
     }
 }
 
-fn promote_and_register_thread(
-    rpc: &rpc::ClientProxy<ServerMessage, ClientMessage>,
-    callback: Option<extern "C" fn(*const ::std::os::raw::c_char)>,
-) {
-    promote_thread(rpc);
-    register_thread(callback);
-}
 
 #[derive(Default)]
 struct DeviceCollectionCallback {
@@ -234,7 +198,7 @@ impl ContextOps for ClientContext {
 
         let cpu_pool = futures_cpupool::Builder::new()
             .name_prefix("AudioIPC")
-            .after_start(move || promote_and_register_thread(&rpc2, thread_create_callback))
+            .after_start(move || register_thread(thread_create_callback))
             .before_stop(move || unregister_thread(thread_destroy_callback))
             .pool_size(params.pool_size)
             .stack_size(params.stack_size)
