$NetBSD$

--- third_party/rust/audio_thread_priority/src/lib.rs.orig	Mon Jul 20 22:49:50 2020
+++ third_party/rust/audio_thread_priority/src/lib.rs
@@ -14,10 +14,14 @@ pub struct AudioThreadPriorityError {
 }
 
 impl AudioThreadPriorityError {
-    fn new_with_inner(message: &str, inner: Box<dyn Error>) -> AudioThreadPriorityError {
-        AudioThreadPriorityError {
-            message: message.into(),
-            inner: Some(inner),
+    cfg_if! {
+        if #[cfg(all(target_os = "linux", feature = "dbus"))] {
+            fn new_with_inner(message: &str, inner: Box<dyn Error>) -> AudioThreadPriorityError {
+                AudioThreadPriorityError {
+                    message: message.into(),
+                    inner: Some(inner),
+                }
+            }
         }
     }
     fn new(message: &str) -> AudioThreadPriorityError {
@@ -64,13 +68,13 @@ cfg_if! {
         use rt_win::promote_current_thread_to_real_time_internal;
         use rt_win::demote_current_thread_from_real_time_internal;
         use rt_win::RtPriorityHandleInternal;
-    } else if #[cfg(target_os = "linux")] {
+    } else if #[cfg(all(target_os = "linux", feature = "dbus"))] {
         mod rt_linux;
         extern crate dbus;
         extern crate libc;
-        use rt_linux::set_real_time_hard_limit_internal as set_real_time_hard_limit;
         use rt_linux::promote_current_thread_to_real_time_internal;
         use rt_linux::demote_current_thread_from_real_time_internal;
+        use rt_linux::set_real_time_hard_limit_internal as set_real_time_hard_limit;
         use rt_linux::get_current_thread_info_internal;
         use rt_linux::promote_thread_to_real_time_internal;
         use rt_linux::demote_thread_from_real_time_internal;
@@ -80,8 +84,27 @@ cfg_if! {
         /// Size of a RtPriorityThreadInfo or atp_thread_info struct, for use in FFI.
         pub static ATP_THREAD_INFO_SIZE: usize = std::mem::size_of::<RtPriorityThreadInfo>();
     } else {
-        // blanket implementations for Android and other systems.
+        // blanket implementations for Android, Linux Desktop without dbus and others
         pub struct RtPriorityHandleInternal {}
+        #[derive(Clone, Copy)]
+        pub struct RtPriorityThreadInfoInternal {
+            _dummy: u8
+        }
+
+        cfg_if! {
+            if #[cfg(not(target_os = "linux"))] {
+                pub type RtPriorityThreadInfo = RtPriorityThreadInfoInternal;
+            }
+        }
+
+        impl RtPriorityThreadInfo {
+            pub fn serialize(&self) -> [u8; 1] {
+                [0]
+            }
+            pub fn deserialize(_: [u8; 1]) -> Self {
+                RtPriorityThreadInfo{_dummy: 0}
+            }
+        }
         pub fn promote_current_thread_to_real_time_internal(_: u32, audio_samplerate_hz: u32) -> Result<RtPriorityHandle, AudioThreadPriorityError> {
             if audio_samplerate_hz == 0 {
                 return Err(AudioThreadPriorityError{message: "sample rate is zero".to_string(), inner: None});
@@ -93,6 +116,32 @@ cfg_if! {
             // no-op
             Ok(())
         }
+        pub fn set_real_time_hard_limit(
+            _: u32,
+            _: u32,
+        ) -> Result<(), AudioThreadPriorityError> {
+            Ok(())
+        }
+        pub fn get_current_thread_info_internal() -> Result<RtPriorityThreadInfo, AudioThreadPriorityError> {
+            Ok(RtPriorityThreadInfo{_dummy: 0})
+        }
+        pub fn promote_thread_to_real_time_internal(
+            _: RtPriorityThreadInfo,
+            _: u32,
+            audio_samplerate_hz: u32,
+        ) -> Result<RtPriorityHandle, AudioThreadPriorityError> {
+            if audio_samplerate_hz == 0 {
+                return Err(AudioThreadPriorityError::new("sample rate is zero"));
+            }
+            return Ok(RtPriorityHandle{});
+        }
+
+        pub fn demote_thread_from_real_time_internal(_: RtPriorityThreadInfo) -> Result<(), AudioThreadPriorityError> {
+            return Ok(());
+        }
+        #[no_mangle]
+        /// Size of a RtPriorityThreadInfo or atp_thread_info struct, for use in FFI.
+        pub static ATP_THREAD_INFO_SIZE: usize = std::mem::size_of::<RtPriorityThreadInfo>();
     }
 }
 
