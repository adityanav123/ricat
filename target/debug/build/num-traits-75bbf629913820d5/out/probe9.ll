; ModuleID = 'probe9.a37e911266c57f6d-cgu.0'
source_filename = "probe9.a37e911266c57f6d-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; core::f64::<impl f64>::to_ne_bytes
; Function Attrs: inlinehint nonlazybind uwtable
define internal i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17h6f8f56a0df7d065dE"(double %self) unnamed_addr #0 {
start:
  %_0 = alloca [8 x i8], align 1
  %self1 = bitcast double %self to i64
  store i64 %self1, ptr %_0, align 1
  %0 = load i64, ptr %_0, align 1
  ret i64 %0
}

; probe9::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe95probe17hf14004aada8f1145E() unnamed_addr #1 {
start:
  %0 = alloca [8 x i8], align 8
  %_1 = alloca [8 x i8], align 1
; call core::f64::<impl f64>::to_ne_bytes
  %1 = call i64 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$11to_ne_bytes17h6f8f56a0df7d065dE"(double 3.140000e+00)
  store i64 %1, ptr %0, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %_1, ptr align 8 %0, i64 8, i1 false)
  ret void
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #2

attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #2 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!"rustc version 1.79.0 (129f3b996 2024-06-10)"}
