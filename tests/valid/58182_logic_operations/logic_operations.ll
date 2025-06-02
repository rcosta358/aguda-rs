; ModuleID = './tests/valid/58182_logic_operations/logic_operations.agu'
source_filename = "./tests/valid/58182_logic_operations/logic_operations.agu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@.str = private unnamed_addr constant [3 x i8] c"%d\00", align 1
@.str.1 = private unnamed_addr constant [5 x i8] c"true\00", align 1
@.str.2 = private unnamed_addr constant [6 x i8] c"false\00", align 1
@.str.3 = private unnamed_addr constant [5 x i8] c"unit\00", align 1
@.str.4 = private unnamed_addr constant [17 x i8] c"division by zero\00", align 1

; Function Attrs: noinline nounwind optnone uwtable
define dso_local void @__print_int__(i32 noundef %0) #0 {
  %2 = alloca i32, align 4
  store i32 %0, ptr %2, align 4
  %3 = load i32, ptr %2, align 4
  %4 = call i32 (ptr, ...) @printf(ptr noundef @.str, i32 noundef %3)
  ret void
}

declare i32 @printf(ptr noundef, ...) #1

; Function Attrs: noinline nounwind optnone uwtable
define dso_local void @__print_bool__(i32 noundef %0) #0 {
  %2 = alloca i32, align 4
  store i32 %0, ptr %2, align 4
  %3 = load i32, ptr %2, align 4
  %4 = icmp ne i32 %3, 0
  %5 = zext i1 %4 to i64
  %6 = select i1 %4, ptr @.str.1, ptr @.str.2
  %7 = call i32 (ptr, ...) @printf(ptr noundef %6)
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define dso_local void @__print_unit__() #0 {
  %1 = call i32 (ptr, ...) @printf(ptr noundef @.str.3)
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @__pow__(i32 noundef %0, i32 noundef %1) #0 {
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  %5 = alloca i32, align 4
  store i32 %0, ptr %3, align 4
  store i32 %1, ptr %4, align 4
  store i32 1, ptr %5, align 4
  br label %6

6:                                                ; preds = %10, %2
  %7 = load i32, ptr %4, align 4
  %8 = add nsw i32 %7, -1
  store i32 %8, ptr %4, align 4
  %9 = icmp sgt i32 %7, 0
  br i1 %9, label %10, label %14

10:                                               ; preds = %6
  %11 = load i32, ptr %3, align 4
  %12 = load i32, ptr %5, align 4
  %13 = mul nsw i32 %12, %11
  store i32 %13, ptr %5, align 4
  br label %6, !llvm.loop !6

14:                                               ; preds = %6
  %15 = load i32, ptr %5, align 4
  ret i32 %15
}

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @__div__(i32 noundef %0, i32 noundef %1) #0 {
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  store i32 %0, ptr %3, align 4
  store i32 %1, ptr %4, align 4
  %5 = load i32, ptr %4, align 4
  %6 = icmp eq i32 %5, 0
  br i1 %6, label %7, label %9

7:                                                ; preds = %2
  %8 = call i32 @write(i32 noundef 2, ptr noundef @.str.4, i64 noundef 17)
  call void @exit(i32 noundef 1) #3
  unreachable

9:                                                ; preds = %2
  %10 = load i32, ptr %3, align 4
  %11 = load i32, ptr %4, align 4
  %12 = sdiv i32 %10, %11
  ret i32 %12
}

declare i32 @write(i32 noundef, ptr noundef, i64 noundef) #1

; Function Attrs: noreturn
declare void @exit(i32 noundef) #2

define i32 @main({} %0) {
entry:
  %a = alloca i1, align 1
  store i1 true, ptr %a, align 1
  %b = alloca i1, align 1
  store i1 false, ptr %b, align 1
  %c = alloca i1, align 1
  store i1 true, ptr %c, align 1
  %opRes = alloca i1, align 1
  store i1 false, ptr %opRes, align 1
  %a1 = load i1, ptr %a, align 1
  br i1 %a1, label %rhs, label %merge

rhs:                                              ; preds = %entry
  %b2 = load i1, ptr %b, align 1
  %not = xor i1 %b2, true
  br i1 %not, label %rhs3, label %merge4

merge:                                            ; preds = %merge4, %entry
  %logical_op6 = phi i1 [ %a1, %entry ], [ %logical_op, %merge4 ]
  store i1 %logical_op6, ptr %opRes, align 1
  %opRes7 = load i1, ptr %opRes, align 1
  %opRes8 = load i1, ptr %opRes, align 1
  call void @__print_bool__(i1 %opRes8)
  %a9 = load i1, ptr %a, align 1
  br i1 %a9, label %rhs10, label %merge11

rhs3:                                             ; preds = %rhs
  %c5 = load i1, ptr %c, align 1
  br label %merge4

merge4:                                           ; preds = %rhs3, %rhs
  %logical_op = phi i1 [ %b2, %rhs ], [ %c5, %rhs3 ]
  br label %merge

rhs10:                                            ; preds = %merge
  %b12 = load i1, ptr %b, align 1
  br label %merge11

merge11:                                          ; preds = %rhs10, %merge
  %logical_op13 = phi i1 [ %a9, %merge ], [ %b12, %rhs10 ]
  %not16 = xor i1 %logical_op13, true
  br i1 %not16, label %rhs14, label %merge15

rhs14:                                            ; preds = %merge11
  %c17 = load i1, ptr %c, align 1
  br label %merge15

merge15:                                          ; preds = %rhs14, %merge11
  %logical_op18 = phi i1 [ %logical_op13, %merge11 ], [ %c17, %rhs14 ]
  store i1 %logical_op18, ptr %opRes, align 1
  %opRes19 = load i1, ptr %opRes, align 1
  %opRes20 = load i1, ptr %opRes, align 1
  call void @__print_bool__(i1 %opRes20)
  %a21 = load i1, ptr %a, align 1
  br i1 %a21, label %rhs22, label %merge23

rhs22:                                            ; preds = %merge15
  %b24 = load i1, ptr %b, align 1
  br label %merge23

merge23:                                          ; preds = %rhs22, %merge15
  %logical_op25 = phi i1 [ %a21, %merge15 ], [ %b24, %rhs22 ]
  %not26 = xor i1 %logical_op25, true
  %not29 = xor i1 %not26, true
  br i1 %not29, label %rhs27, label %merge28

rhs27:                                            ; preds = %merge23
  %b30 = load i1, ptr %b, align 1
  br i1 %b30, label %rhs31, label %merge32

merge28:                                          ; preds = %merge32, %merge23
  %logical_op35 = phi i1 [ %not26, %merge23 ], [ %logical_op34, %merge32 ]
  store i1 %logical_op35, ptr %opRes, align 1
  %opRes36 = load i1, ptr %opRes, align 1
  %opRes37 = load i1, ptr %opRes, align 1
  call void @__print_bool__(i1 %opRes37)
  %a38 = load i1, ptr %a, align 1
  %not39 = xor i1 %a38, true
  br i1 %not39, label %rhs40, label %merge41

rhs31:                                            ; preds = %rhs27
  %c33 = load i1, ptr %c, align 1
  br label %merge32

merge32:                                          ; preds = %rhs31, %rhs27
  %logical_op34 = phi i1 [ %b30, %rhs27 ], [ %c33, %rhs31 ]
  br label %merge28

rhs40:                                            ; preds = %merge28
  %b42 = load i1, ptr %b, align 1
  br label %merge41

merge41:                                          ; preds = %rhs40, %merge28
  %logical_op43 = phi i1 [ %not39, %merge28 ], [ %b42, %rhs40 ]
  store i1 %logical_op43, ptr %opRes, align 1
  %opRes44 = load i1, ptr %opRes, align 1
  %opRes45 = load i1, ptr %opRes, align 1
  call void @__print_bool__(i1 %opRes45)
  %a46 = load i1, ptr %a, align 1
  %not49 = xor i1 %a46, true
  br i1 %not49, label %rhs47, label %merge48

rhs47:                                            ; preds = %merge41
  %b50 = load i1, ptr %b, align 1
  br label %merge48

merge48:                                          ; preds = %rhs47, %merge41
  %logical_op51 = phi i1 [ %a46, %merge41 ], [ %b50, %rhs47 ]
  br i1 %logical_op51, label %rhs52, label %merge53

rhs52:                                            ; preds = %merge48
  %c54 = load i1, ptr %c, align 1
  br label %merge53

merge53:                                          ; preds = %rhs52, %merge48
  %logical_op55 = phi i1 [ %logical_op51, %merge48 ], [ %c54, %rhs52 ]
  %not56 = xor i1 %logical_op55, true
  store i1 %not56, ptr %opRes, align 1
  %opRes57 = load i1, ptr %opRes, align 1
  %opRes58 = load i1, ptr %opRes, align 1
  call void @__print_bool__(i1 %opRes58)
  ret i32 0
}

attributes #0 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { noreturn "frame-pointer"="all" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { noreturn }

!llvm.ident = !{!0}
!llvm.module.flags = !{!1, !2, !3, !4, !5}

!0 = !{!"clang version 17.0.6 (https://github.com/llvm/llvm-project 6009708b4367171ccdbf4b5905cb6a803753fe18)"}
!1 = !{i32 1, !"wchar_size", i32 4}
!2 = !{i32 8, !"PIC Level", i32 2}
!3 = !{i32 7, !"PIE Level", i32 2}
!4 = !{i32 7, !"uwtable", i32 2}
!5 = !{i32 7, !"frame-pointer", i32 2}
!6 = distinct !{!6, !7}
!7 = !{!"llvm.loop.mustprogress"}
