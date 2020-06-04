$NetBSD$

--- toolkit/library/rust/shared/lib.rs.orig	Fri May 29 06:02:46 2020
+++ toolkit/library/rust/shared/lib.rs
@@ -51,8 +51,6 @@ extern crate xpcom;
 #[cfg(feature = "new_xulstore")]
 extern crate xulstore;
 
-extern crate audio_thread_priority;
-
 #[cfg(feature = "new_webext_storage")]
 extern crate webext_storage_bridge;
 
