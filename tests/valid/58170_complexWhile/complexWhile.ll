; ModuleID = './tests/valid/58170_complexWhile/complexWhile.agu'
source_filename = "./tests/valid/58170_complexWhile/complexWhile.agu"
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
  %a = alloca i32, align 4
  store i32 1, ptr %a, align 4
  %b = alloca i32, align 4
  store i32 10, ptr %b, align 4
  %c = alloca i32, align 4
  store i32 0, ptr %c, align 4
  br label %cond

cond:                                             ; preds = %body, %entry
  %a1 = load i32, ptr %a, align 4
  %b2 = load i32, ptr %b, align 4
  %lt = icmp slt i32 %a1, %b2
  br i1 %lt, label %rhs, label %merge

body:                                             ; preds = %merge
  %a10 = load i32, ptr %a, align 4
  %a11 = load i32, ptr %a, align 4
  call void @__print_int__(i32 %a11)
  %b12 = load i32, ptr %b, align 4
  %b13 = load i32, ptr %b, align 4
  call void @__print_int__(i32 %b13)
  %a14 = load i32, ptr %a, align 4
  %add = add i32 %a14, 1
  store i32 %add, ptr %a, align 4
  %b15 = load i32, ptr %b, align 4
  %sub = sub i32 %b15, 1
  store i32 %sub, ptr %b, align 4
  %c16 = load i32, ptr %c, align 4
  %add17 = add i32 %c16, 1
  store i32 %add17, ptr %c, align 4
  br label %cond

after:                                            ; preds = %merge
  %c18 = load i32, ptr %c, align 4
  %c19 = load i32, ptr %c, align 4
  call void @__print_int__(i32 %c19)
  %a20 = load i32, ptr %a, align 4
  %a21 = load i32, ptr %a, align 4
  call void @__print_int__(i32 %a21)
  %b22 = load i32, ptr %b, align 4
  %b23 = load i32, ptr %b, align 4
  call void @__print_int__(i32 %b23)
  ret i32 0

rhs:                                              ; preds = %cond
  %a3 = load i32, ptr %a, align 4
  %mod = srem i32 %a3, 3
  %neq = icmp ne i32 %mod, 0
  %not = xor i1 %neq, true
  br i1 %not, label %rhs4, label %merge5

merge:                                            ; preds = %merge5, %cond
  %logical_op9 = phi i1 [ %lt, %cond ], [ %logical_op, %merge5 ]
  %while_cond = icmp ne i1 %logical_op9, false
  br i1 %while_cond, label %body, label %after

rhs4:                                             ; preds = %rhs
  %b6 = load i32, ptr %b, align 4
  %mod7 = srem i32 %b6, 5
  %neq8 = icmp ne i32 %mod7, 0
  br label %merge5

merge5:                                           ; preds = %rhs4, %rhs
  %logical_op = phi i1 [ %neq, %rhs ], [ %neq8, %rhs4 ]
  br label %merge
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
