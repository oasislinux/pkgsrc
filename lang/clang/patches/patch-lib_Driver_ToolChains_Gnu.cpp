$NetBSD: patch-lib_Driver_ToolChains_Gnu.cpp,v 1.4 2024/07/07 14:00:21 wiz Exp $

On SunOS always use the GCC that was used to build clang.

Fix gcc installation detection with musl libc.

Upstream merge request: https://reviews.llvm.org/D67516

--- lib/Driver/ToolChains/Gnu.cpp.orig	2024-06-15 17:21:32.000000000 +0000
+++ lib/Driver/ToolChains/Gnu.cpp
@@ -2228,7 +2228,8 @@ void Generic_GCC::GCCInstallationDetector::AddDefaultG
   static const char *const ARMHFTriples[] = {"arm-linux-gnueabihf",
                                              "armv7hl-redhat-linux-gnueabi",
                                              "armv6hl-suse-linux-gnueabi",
-                                             "armv7hl-suse-linux-gnueabi"};
+                                             "armv7hl-suse-linux-gnueabi",
+                                             "armv7l-linux-gnueabihf"};
   static const char *const ARMebLibDirs[] = {"/lib"};
   static const char *const ARMebTriples[] = {"armeb-linux-gnueabi"};
   static const char *const ARMebHFTriples[] = {
@@ -2381,6 +2382,78 @@ void Generic_GCC::GCCInstallationDetector::AddDefaultG
     default:
       break;
     }
+    return;
+  }
+
+  if (TargetTriple.isMusl()) {
+    static const char *const AArch64MuslTriples[] = {"aarch64-linux-musl"};
+    static const char *const ARMHFMuslTriples[] = {
+        "arm-linux-musleabihf", "armv7l-linux-musleabihf"
+    };
+    static const char *const ARMMuslTriples[] = {"arm-linux-musleabi"};
+    static const char *const X86_64MuslTriples[] = {"x86_64-linux-musl"};
+    static const char *const X86MuslTriples[] = {"i686-linux-musl"};
+    static const char *const MIPSMuslTriples[] = {
+        "mips-linux-musl", "mipsel-linux-musl",
+        "mipsel-linux-muslhf", "mips-linux-muslhf"
+    };
+    static const char *const PPCMuslTriples[] = {"powerpc-linux-musl"};
+    static const char *const PPC64MuslTriples[] = {"powerpc64-linux-musl"};
+    static const char *const PPC64LEMuslTriples[] = {"powerpc64le-linux-musl"};
+
+    switch (TargetTriple.getArch()) {
+    case llvm::Triple::aarch64:
+      LibDirs.append(begin(AArch64LibDirs), end(AArch64LibDirs));
+      TripleAliases.append(begin(AArch64MuslTriples), end(AArch64MuslTriples));
+      BiarchLibDirs.append(begin(AArch64LibDirs), end(AArch64LibDirs));
+      BiarchTripleAliases.append(begin(AArch64MuslTriples), end(AArch64MuslTriples));
+      break;
+    case llvm::Triple::arm:
+      LibDirs.append(begin(ARMLibDirs), end(ARMLibDirs));
+      if (TargetTriple.getEnvironment() == llvm::Triple::MuslEABIHF) {
+        TripleAliases.append(begin(ARMHFMuslTriples), end(ARMHFMuslTriples));
+      } else {
+        TripleAliases.append(begin(ARMMuslTriples), end(ARMMuslTriples));
+      }
+      break;
+    case llvm::Triple::x86_64:
+      LibDirs.append(begin(X86_64LibDirs), end(X86_64LibDirs));
+      TripleAliases.append(begin(X86_64MuslTriples), end(X86_64MuslTriples));
+      BiarchLibDirs.append(begin(X86LibDirs), end(X86LibDirs));
+      BiarchTripleAliases.append(begin(X86MuslTriples), end(X86MuslTriples));
+      break;
+    case llvm::Triple::x86:
+      LibDirs.append(begin(X86LibDirs), end(X86LibDirs));
+      TripleAliases.append(begin(X86MuslTriples), end(X86MuslTriples));
+      BiarchLibDirs.append(begin(X86_64LibDirs), end(X86_64LibDirs));
+      BiarchTripleAliases.append(begin(X86_64MuslTriples), end(X86_64MuslTriples));
+      break;
+    case llvm::Triple::mips:
+      LibDirs.append(begin(MIPSLibDirs), end(MIPSLibDirs));
+      TripleAliases.append(begin(MIPSMuslTriples), end(MIPSMuslTriples));
+      break;
+    case llvm::Triple::ppc:
+      LibDirs.append(begin(PPCLibDirs), end(PPCLibDirs));
+      TripleAliases.append(begin(PPCMuslTriples), end(PPCMuslTriples));
+      BiarchLibDirs.append(begin(PPC64LibDirs), end(PPC64LibDirs));
+      BiarchTripleAliases.append(begin(PPC64MuslTriples), end(PPC64MuslTriples));
+      break;
+    case llvm::Triple::ppc64:
+      LibDirs.append(begin(PPC64LibDirs), end(PPC64LibDirs));
+      TripleAliases.append(begin(PPC64MuslTriples), end(PPC64MuslTriples));
+      BiarchLibDirs.append(begin(PPCLibDirs), end(PPCLibDirs));
+      BiarchTripleAliases.append(begin(PPCMuslTriples), end(PPCMuslTriples));
+      break;
+    case llvm::Triple::ppc64le:
+      LibDirs.append(begin(PPC64LELibDirs), end(PPC64LELibDirs));
+      TripleAliases.append(begin(PPC64LEMuslTriples), end(PPC64LEMuslTriples));
+      break;
+    default:
+      break;
+    }
+    TripleAliases.push_back(TargetTriple.str());
+    if (TargetTriple.str() != BiarchTriple.str())
+      BiarchTripleAliases.push_back(BiarchTriple.str());
     return;
   }
 
@@ -2382,6 +2382,11 @@ void Generic_GCC::GCCInstallationDetecto
     // /usr/gcc/<version> as a prefix.
 
     SmallVector<std::pair<GCCVersion, std::string>, 8> SolarisPrefixes;
+
+    // Only use compiler as configured by pkgsrc
+    Prefixes.push_back("@GCCBASEDIR@");
+    return;
+
     std::string PrefixDir = concat(SysRoot, "/usr/gcc");
     std::error_code EC;
     for (llvm::vfs::directory_iterator LI = D.getVFS().dir_begin(PrefixDir, EC),
