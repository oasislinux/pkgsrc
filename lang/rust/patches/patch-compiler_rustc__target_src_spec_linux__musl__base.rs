$NetBSD$

Link musl targets dynamically by default, like all other targets.

--- compiler/rustc_target/src/spec/linux_musl_base.rs.orig	Wed Feb 10 17:36:44 2021
+++ compiler/rustc_target/src/spec/linux_musl_base.rs
@@ -9,8 +9,5 @@ pub fn opts() -> TargetOptions {
     base.post_link_objects_fallback = crt_objects::post_musl_fallback();
     base.crt_objects_fallback = Some(CrtObjectsFallback::Musl);
 
-    // These targets statically link libc by default
-    base.crt_static_default = true;
-
     base
 }
