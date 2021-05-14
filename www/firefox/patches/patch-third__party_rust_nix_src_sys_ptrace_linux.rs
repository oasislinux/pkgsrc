$NetBSD$

Fix build on linux-musl.

Upstream: https://github.com/nix-rust/nix/pull/1198

--- third_party/rust/nix/src/sys/ptrace/linux.rs.orig	Tue May  4 19:00:06 2021
+++ third_party/rust/nix/src/sys/ptrace/linux.rs
@@ -9,10 +9,12 @@ use sys::signal::Signal;
 
 pub type AddressType = *mut ::libc::c_void;
 
-#[cfg(all(target_os = "linux",
-          any(target_arch = "x86_64",
-              target_arch = "x86"),
-          target_env = "gnu"))]
+#[cfg(all(
+    target_os = "linux",
+    any(all(target_arch = "x86_64",
+            any(target_env = "gnu", target_env = "musl")),
+        all(target_arch = "x86", target_env = "gnu"))
+))]
 use libc::user_regs_struct;
 
 cfg_if! {
@@ -193,19 +195,23 @@ fn ptrace_peek(request: Request, pid: Pid, addr: Addre
 }
 
 /// Get user registers, as with `ptrace(PTRACE_GETREGS, ...)`
-#[cfg(all(target_os = "linux",
-          any(target_arch = "x86_64",
-              target_arch = "x86"),
-          target_env = "gnu"))]
+#[cfg(all(
+    target_os = "linux",
+    any(all(target_arch = "x86_64",
+            any(target_env = "gnu", target_env = "musl")),
+        all(target_arch = "x86", target_env = "gnu"))
+))]
 pub fn getregs(pid: Pid) -> Result<user_regs_struct> {
     ptrace_get_data::<user_regs_struct>(Request::PTRACE_GETREGS, pid)
 }
 
 /// Set user registers, as with `ptrace(PTRACE_SETREGS, ...)`
-#[cfg(all(target_os = "linux",
-          any(target_arch = "x86_64",
-              target_arch = "x86"),
-          target_env = "gnu"))]
+#[cfg(all(
+    target_os = "linux",
+    any(all(target_arch = "x86_64",
+            any(target_env = "gnu", target_env = "musl")),
+        all(target_arch = "x86", target_env = "gnu"))
+))]
 pub fn setregs(pid: Pid, regs: user_regs_struct) -> Result<()> {
     let res = unsafe {
         libc::ptrace(Request::PTRACE_SETREGS as RequestType,
