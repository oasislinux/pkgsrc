$NetBSD$

Link musl targets dynamically by default, like all other targets.

--- src/librustc_target/spec/linux_musl_base.rs.orig	Wed Jan 22 06:09:56 2020
+++ src/librustc_target/spec/linux_musl_base.rs
@@ -25,8 +25,6 @@ pub fn opts() -> TargetOptions {
     base.pre_link_objects_exe_crt.push("crti.o".to_string());
     base.post_link_objects_crt.push("crtn.o".to_string());
 
-    // These targets statically link libc by default
-    base.crt_static_default = true;
     // These targets allow the user to choose between static and dynamic linking.
     base.crt_static_respected = true;
 
