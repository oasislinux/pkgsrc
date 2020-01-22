$NetBSD$

--- toolkit/library/rust/shared/lib.rs.orig	Fri Apr  3 19:35:39 2020
+++ toolkit/library/rust/shared/lib.rs
@@ -50,8 +50,6 @@ extern crate xpcom;
 #[cfg(feature = "new_xulstore")]
 extern crate xulstore;
 
-extern crate audio_thread_priority;
-
 #[cfg(feature = "webrtc")]
 extern crate mdns_service;
 extern crate neqo_glue;
