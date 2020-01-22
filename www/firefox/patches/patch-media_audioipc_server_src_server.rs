$NetBSD$

--- media/audioipc/server/src/server.rs.orig	Wed Jan  8 01:23:35 2020
+++ media/audioipc/server/src/server.rs
@@ -3,8 +3,6 @@
 // This program is made available under an ISC-style license.  See the
 // accompanying file LICENSE for details
 
-#[cfg(target_os = "linux")]
-use audio_thread_priority::{promote_thread_to_real_time, RtPriorityThreadInfo};
 use audioipc;
 use audioipc::codec::LengthDelimitedCodec;
 use audioipc::frame::{framed, Framed};
@@ -523,19 +521,6 @@ impl CubebServer {
                 )
                 .unwrap_or_else(error),
 
-            #[cfg(target_os = "linux")]
-            ServerMessage::PromoteThreadToRealTime(thread_info) => {
-                let info = RtPriorityThreadInfo::deserialize(thread_info);
-                match promote_thread_to_real_time(info, 0, 48000) {
-                    Ok(_) => {
-                        info!("Promotion of content process thread to real-time OK");
-                    }
-                    Err(_) => {
-                        warn!("Promotion of content process thread to real-time error");
-                    }
-                }
-                ClientMessage::ThreadPromoted
-            }
         };
 
         trace!("process_msg: req={:?}, resp={:?}", msg, resp);
