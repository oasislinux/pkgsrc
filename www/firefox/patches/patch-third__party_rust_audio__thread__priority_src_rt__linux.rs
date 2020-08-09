$NetBSD$

--- third_party/rust/audio_thread_priority/src/rt_linux.rs.orig	Mon Jul 20 22:49:50 2020
+++ third_party/rust/audio_thread_priority/src/rt_linux.rs
@@ -33,7 +33,7 @@ impl From<dbus::Error> for AudioThreadPriorityError {
 
 impl From<Box<dyn Error>> for AudioThreadPriorityError {
     fn from(error: Box<dyn Error>) -> Self {
-        AudioThreadPriorityError::new(&format!("{}", error.description()))
+        AudioThreadPriorityError::new(&format!("{}", error.to_string()))
     }
 }
 
