; ModuleID = 'probe1.3a1fbbbh-cgu.0'
source_filename = "probe1.3a1fbbbh-cgu.0"
target datalayout = "e-m:e-p:32:32-i64:64-n32:64-S128"
target triple = "wasm32-unknown-emscripten"

%"std::iter::Rev<std::iter::StepBy<std::ops::Range<i32>>>" = type { [0 x i32], %"std::iter::StepBy<std::ops::Range<i32>>", [0 x i32] }
%"std::iter::StepBy<std::ops::Range<i32>>" = type { [0 x i32], { i32, i32 }, [0 x i32], i32, [0 x i8], i8, [3 x i8] }
%"std::panic::Location" = type { [0 x i32], { [0 x i8]*, i32 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }

@alloc1 = private unnamed_addr constant <{ [27 x i8] }> <{ [27 x i8] c"assertion failed: step != 0" }>, align 1
@alloc2 = private unnamed_addr constant <{ [124 x i8] }> <{ [124 x i8] c"/home/oleg/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/iter/adapters/step_by.rs" }>, align 1
@alloc3 = private unnamed_addr constant <{ i8*, [12 x i8] }> <{ i8* getelementptr inbounds (<{ [124 x i8] }>, <{ [124 x i8] }>* @alloc2, i32 0, i32 0, i32 0), [12 x i8] c"|\00\00\00\15\00\00\00\09\00\00\00" }>, align 4

; core::iter::traits::iterator::Iterator::rev
; Function Attrs: inlinehint uwtable
define hidden void @_ZN4core4iter6traits8iterator8Iterator3rev17h0914bc7bef47470eE(%"std::iter::Rev<std::iter::StepBy<std::ops::Range<i32>>>"* noalias nocapture sret dereferenceable(16) %0, %"std::iter::StepBy<std::ops::Range<i32>>"* noalias nocapture dereferenceable(16) %self) unnamed_addr #0 {
start:
  %_2 = alloca %"std::iter::StepBy<std::ops::Range<i32>>", align 4
  %1 = bitcast %"std::iter::StepBy<std::ops::Range<i32>>"* %_2 to i8*
  %2 = bitcast %"std::iter::StepBy<std::ops::Range<i32>>"* %self to i8*
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 4 %1, i8* align 4 %2, i32 16, i1 false)
; call core::iter::adapters::rev::Rev<T>::new
  call void @"_ZN4core4iter8adapters3rev12Rev$LT$T$GT$3new17h03851e491507f48fE"(%"std::iter::Rev<std::iter::StepBy<std::ops::Range<i32>>>"* noalias nocapture sret dereferenceable(16) %0, %"std::iter::StepBy<std::ops::Range<i32>>"* noalias nocapture dereferenceable(16) %_2)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::iter::traits::iterator::Iterator::step_by
; Function Attrs: inlinehint uwtable
define hidden void @_ZN4core4iter6traits8iterator8Iterator7step_by17hec34a1d49b974754E(%"std::iter::StepBy<std::ops::Range<i32>>"* noalias nocapture sret dereferenceable(16) %0, i32 %self.0, i32 %self.1, i32 %step) unnamed_addr #0 {
start:
; call core::iter::adapters::step_by::StepBy<I>::new
  call void @"_ZN4core4iter8adapters7step_by15StepBy$LT$I$GT$3new17h6cbcae2055a08543E"(%"std::iter::StepBy<std::ops::Range<i32>>"* noalias nocapture sret dereferenceable(16) %0, i32 %self.0, i32 %self.1, i32 %step)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; core::iter::adapters::rev::Rev<T>::new
; Function Attrs: uwtable
define hidden void @"_ZN4core4iter8adapters3rev12Rev$LT$T$GT$3new17h03851e491507f48fE"(%"std::iter::Rev<std::iter::StepBy<std::ops::Range<i32>>>"* noalias nocapture sret dereferenceable(16) %0, %"std::iter::StepBy<std::ops::Range<i32>>"* noalias nocapture dereferenceable(16) %iter) unnamed_addr #1 {
start:
  %_2 = alloca %"std::iter::StepBy<std::ops::Range<i32>>", align 4
  %1 = bitcast %"std::iter::StepBy<std::ops::Range<i32>>"* %_2 to i8*
  %2 = bitcast %"std::iter::StepBy<std::ops::Range<i32>>"* %iter to i8*
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 4 %1, i8* align 4 %2, i32 16, i1 false)
  %3 = bitcast %"std::iter::Rev<std::iter::StepBy<std::ops::Range<i32>>>"* %0 to %"std::iter::StepBy<std::ops::Range<i32>>"*
  %4 = bitcast %"std::iter::StepBy<std::ops::Range<i32>>"* %3 to i8*
  %5 = bitcast %"std::iter::StepBy<std::ops::Range<i32>>"* %_2 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i32(i8* align 4 %4, i8* align 4 %5, i32 16, i1 false)
  ret void
}

; core::iter::adapters::step_by::StepBy<I>::new
; Function Attrs: uwtable
define hidden void @"_ZN4core4iter8adapters7step_by15StepBy$LT$I$GT$3new17h6cbcae2055a08543E"(%"std::iter::StepBy<std::ops::Range<i32>>"* noalias nocapture sret dereferenceable(16) %0, i32 %iter.0, i32 %iter.1, i32 %step) unnamed_addr #1 personality i32 (...)* @rust_eh_personality {
start:
  %1 = alloca { i8*, i32 }, align 4
  %_4 = icmp ne i32 %step, 0
  %_3 = xor i1 %_4, true
  br i1 %_3, label %bb2, label %bb1

bb1:                                              ; preds = %start
  %_7 = sub i32 %step, 1
  %2 = bitcast %"std::iter::StepBy<std::ops::Range<i32>>"* %0 to { i32, i32 }*
  %3 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %2, i32 0, i32 0
  store i32 %iter.0, i32* %3, align 4
  %4 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %2, i32 0, i32 1
  store i32 %iter.1, i32* %4, align 4
  %5 = getelementptr inbounds %"std::iter::StepBy<std::ops::Range<i32>>", %"std::iter::StepBy<std::ops::Range<i32>>"* %0, i32 0, i32 3
  store i32 %_7, i32* %5, align 4
  %6 = getelementptr inbounds %"std::iter::StepBy<std::ops::Range<i32>>", %"std::iter::StepBy<std::ops::Range<i32>>"* %0, i32 0, i32 5
  store i8 1, i8* %6, align 4
  ret void

bb2:                                              ; preds = %start
; invoke core::panicking::panic
  invoke void @_ZN4core9panicking5panic17h75891ed8f5b3bc8fE([0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [27 x i8] }>* @alloc1 to [0 x i8]*), i32 27, %"std::panic::Location"* noalias readonly align 4 dereferenceable(16) bitcast (<{ i8*, [12 x i8] }>* @alloc3 to %"std::panic::Location"*))
          to label %unreachable unwind label %cleanup

bb3:                                              ; preds = %cleanup
  br label %bb4

bb4:                                              ; preds = %bb3
  %7 = bitcast { i8*, i32 }* %1 to i8**
  %8 = load i8*, i8** %7, align 4
  %9 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  %10 = load i32, i32* %9, align 4
  %11 = insertvalue { i8*, i32 } undef, i8* %8, 0
  %12 = insertvalue { i8*, i32 } %11, i32 %10, 1
  resume { i8*, i32 } %12

unreachable:                                      ; preds = %bb2
  unreachable

cleanup:                                          ; preds = %bb2
  %13 = landingpad { i8*, i32 }
          cleanup
  %14 = extractvalue { i8*, i32 } %13, 0
  %15 = extractvalue { i8*, i32 } %13, 1
  %16 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 0
  store i8* %14, i8** %16, align 4
  %17 = getelementptr inbounds { i8*, i32 }, { i8*, i32 }* %1, i32 0, i32 1
  store i32 %15, i32* %17, align 4
  br label %bb3
}

; probe1::probe
; Function Attrs: uwtable
define hidden void @_ZN6probe15probe17hcc631d850158b26eE() unnamed_addr #1 {
start:
  %_3 = alloca { i32, i32 }, align 4
  %_2 = alloca %"std::iter::StepBy<std::ops::Range<i32>>", align 4
  %_1 = alloca %"std::iter::Rev<std::iter::StepBy<std::ops::Range<i32>>>", align 4
  %0 = bitcast { i32, i32 }* %_3 to i32*
  store i32 0, i32* %0, align 4
  %1 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_3, i32 0, i32 1
  store i32 10, i32* %1, align 4
  %2 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_3, i32 0, i32 0
  %3 = load i32, i32* %2, align 4
  %4 = getelementptr inbounds { i32, i32 }, { i32, i32 }* %_3, i32 0, i32 1
  %5 = load i32, i32* %4, align 4
; call core::iter::traits::iterator::Iterator::step_by
  call void @_ZN4core4iter6traits8iterator8Iterator7step_by17hec34a1d49b974754E(%"std::iter::StepBy<std::ops::Range<i32>>"* noalias nocapture sret dereferenceable(16) %_2, i32 %3, i32 %5, i32 2)
  br label %bb1

bb1:                                              ; preds = %start
; call core::iter::traits::iterator::Iterator::rev
  call void @_ZN4core4iter6traits8iterator8Iterator3rev17h0914bc7bef47470eE(%"std::iter::Rev<std::iter::StepBy<std::ops::Range<i32>>>"* noalias nocapture sret dereferenceable(16) %_1, %"std::iter::StepBy<std::ops::Range<i32>>"* noalias nocapture dereferenceable(16) %_2)
  br label %bb2

bb2:                                              ; preds = %bb1
  ret void
}

; Function Attrs: argmemonly nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i32(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i32, i1 immarg) #2

declare i32 @rust_eh_personality(...) unnamed_addr #3

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17h75891ed8f5b3bc8fE([0 x i8]* noalias nonnull readonly align 1, i32, %"std::panic::Location"* noalias readonly align 4 dereferenceable(16)) unnamed_addr #4

attributes #0 = { inlinehint uwtable "target-cpu"="generic" }
attributes #1 = { uwtable "target-cpu"="generic" }
attributes #2 = { argmemonly nounwind willreturn }
attributes #3 = { "target-cpu"="generic" }
attributes #4 = { cold noinline noreturn uwtable "target-cpu"="generic" }
