$NetBSD$

Link musl targets dynamically by default, like all other targets.

--- compiler/rustc_target/src/spec/linux_musl_base.rs.orig	Tue Jan 12 06:26:46 2021
+++ compiler/rustc_target/src/spec/linux_musl_base.rs
@@ -8,8 +8,6 @@ pub fn opts() -> TargetOptions {
     base.post_link_objects_fallback = crt_objects::post_musl_fallback();
     base.crt_objects_fallback = Some(CrtObjectsFallback::Musl);
 
-    // These targets statically link libc by default
-    base.crt_static_default = true;
     // These targets allow the user to choose between static and dynamic linking.
     base.crt_static_respected = true;
 
