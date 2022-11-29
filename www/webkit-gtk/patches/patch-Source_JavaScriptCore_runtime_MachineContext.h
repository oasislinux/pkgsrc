$NetBSD: patch-Source_JavaScriptCore_runtime_MachineContext.h,v 1.4 2022/06/08 20:56:50 leot Exp $

Add support for NetBSD.

Shared and applied upstream via <https://bugs.webkit.org/show_bug.cgi?id=241125>.

--- Source/JavaScriptCore/runtime/MachineContext.h.orig	2021-05-05 05:33:24.000000000 +0000
+++ Source/JavaScriptCore/runtime/MachineContext.h
@@ -193,6 +193,22 @@ static inline void*& stackPointerImpl(mcontext_t& mach
 #error Unknown Architecture
 #endif
 
+#elif OS(NETBSD)
+
+#if CPU(X86)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_UESP]);
+#elif CPU(X86_64)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_RSP]);
+#elif CPU(ARM)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_SP]);
+#elif CPU(ARM64)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_SP]);
+#elif CPU(MIPS)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_SP]);
+#else
+#error Unknown Architecture
+#endif
+
 #elif OS(FUCHSIA) || OS(LINUX)
 
 #if CPU(X86)
@@ -338,6 +354,22 @@ static inline void*& framePointerImpl(mcontext_t& mach
 #error Unknown Architecture
 #endif
 
+#elif OS(NETBSD)
+
+#if CPU(X86)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_EBP]);
+#elif CPU(X86_64)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_RBP]);
+#elif CPU(ARM)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_FP]);
+#elif CPU(ARM64)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_FP]);
+#elif CPU(MIPS)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_S8]);
+#else
+#error Unknown Architecture
+#endif
+
 #elif OS(FUCHSIA) || OS(LINUX)
 
 // The following sequence depends on glibc's sys/ucontext.h.
@@ -495,7 +527,23 @@ static inline void*& instructionPointerImpl(mcontext_t
 #else
 #error Unknown Architecture
 #endif
+ 
+#elif OS(NETBSD)
 
+#if CPU(X86)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_EIP]);
+#elif CPU(X86_64)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_RIP]);
+#elif CPU(ARM)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_PC]);
+#elif CPU(ARM64)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_PC]);
+#elif CPU(MIPS)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_EPC]);
+#else
+#error Unknown Architecture
+#endif
+
 #elif OS(FUCHSIA) || OS(LINUX)
 
 // The following sequence depends on glibc's sys/ucontext.h.
@@ -651,7 +699,23 @@ inline void*& argumentPointer<1>(mcontext_t& machineCo
 #else
 #error Unknown Architecture
 #endif
+ 
+#elif OS(NETBSD)
 
+#if CPU(X86)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_EDX]);
+#elif CPU(X86_64)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_RSI]);
+#elif CPU(ARM)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_R1]);
+#elif CPU(ARM64)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_X1]);
+#elif CPU(MIPS)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_A1]);
+#else
+#error Unknown Architecture
+#endif
+
 #elif OS(FUCHSIA) || OS(LINUX)
 
 // The following sequence depends on glibc's sys/ucontext.h.
@@ -771,6 +835,22 @@ inline void*& llintInstructionPointer(mcontext_t& mach
 #error Unknown Architecture
 #endif
 
+#elif OS(NETBSD)
+
+#if CPU(X86)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_ESI]);
+#elif CPU(X86_64)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_R8]);
+#elif CPU(ARM)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_R8]);
+#elif CPU(ARM64)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_X4]);
+#elif CPU(MIPS)
+    return reinterpret_cast<void*&>((uintptr_t&) machineContext.__gregs[_REG_T4]);
+#else
+#error Unknown Architecture
+#endif
+ 
 #elif OS(NETBSD)
 
 #if CPU(X86)
