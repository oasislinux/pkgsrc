$NetBSD$

--- media/audioipc/audioipc/src/messages.rs.orig	Wed Jan  8 01:23:35 2020
+++ media/audioipc/audioipc/src/messages.rs
@@ -5,8 +5,6 @@
 
 use crate::PlatformHandle;
 use crate::PlatformHandleType;
-#[cfg(target_os = "linux")]
-use audio_thread_priority::RtPriorityThreadInfo;
 use cubeb::{self, ffi};
 use std::ffi::{CStr, CString};
 use std::os::raw::{c_char, c_int, c_uint};
@@ -210,8 +208,6 @@ pub enum ServerMessage {
     StreamGetCurrentDevice(usize),
     StreamRegisterDeviceChangeCallback(usize, bool),
 
-    #[cfg(target_os = "linux")]
-    PromoteThreadToRealTime([u8; std::mem::size_of::<RtPriorityThreadInfo>()]),
 }
 
 // Server -> Client messages.
@@ -241,8 +237,6 @@ pub enum ClientMessage {
     StreamCurrentDevice(Device),
     StreamRegisterDeviceChangeCallback,
 
-    #[cfg(target_os = "linux")]
-    ThreadPromoted,
 
     Error(c_int),
 }
