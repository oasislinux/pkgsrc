$NetBSD$

--- toolkit/library/rust/shared/lib.rs.orig	Thu Jun 25 17:05:12 2020
+++ toolkit/library/rust/shared/lib.rs
@@ -51,8 +51,6 @@ extern crate xpcom;
 #[cfg(feature = "new_xulstore")]
 extern crate xulstore;
 
-extern crate audio_thread_priority;
-
 #[cfg(not(target_os = "android"))]
 extern crate webext_storage_bridge;
 
