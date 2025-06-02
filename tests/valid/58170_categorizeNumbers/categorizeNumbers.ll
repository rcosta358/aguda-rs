; ModuleID = './tests/valid/58170_categorizeNumbers/categorizeNumbers.agu'
source_filename = "./tests/valid/58170_categorizeNumbers/categorizeNumbers.agu"
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

define i32 @categorizeNumber(i32 %0) {
entry:
  %n = alloca i32, align 4
  store i32 %0, ptr %n, align 4
  %result = alloca i32, align 4
  store i32 0, ptr %result, align 4
  %n1 = load i32, ptr %n, align 4
  %mod = srem i32 %n1, 2
  %eq = icmp eq i32 %mod, 0
  %isEven = alloca i1, align 1
  store i1 %eq, ptr %isEven, align 1
  %n2 = load i32, ptr %n, align 4
  %gt = icmp sgt i32 %n2, 0
  %isPositive = alloca i1, align 1
  store i1 %gt, ptr %isPositive, align 1
  %n3 = load i32, ptr %n, align 4
  %gt4 = icmp sgt i32 %n3, 100
  %isBig = alloca i1, align 1
  store i1 %gt4, ptr %isBig, align 1
  %isEven5 = load i1, ptr %isEven, align 1
  %if_cond = icmp ne i1 %isEven5, false
  br i1 %if_cond, label %then, label %else

then:                                             ; preds = %entry
  %isPositive9 = load i1, ptr %isPositive, align 1
  %if_cond10 = icmp ne i1 %isPositive9, false
  br i1 %if_cond10, label %then6, label %else7

else:                                             ; preds = %entry
  %isPositive27 = load i1, ptr %isPositive, align 1
  %if_cond28 = icmp ne i1 %isPositive27, false
  br i1 %if_cond28, label %then24, label %else25

merge:                                            ; preds = %merge26, %merge8
  %phi30 = phi {} [ %phi23, %merge8 ], [ %phi29, %merge26 ]
  %n34 = load i32, ptr %n, align 4
  %mod35 = srem i32 %n34, 5
  %eq36 = icmp eq i32 %mod35, 0
  %if_cond37 = icmp ne i1 %eq36, false
  br i1 %if_cond37, label %then31, label %else32

then6:                                            ; preds = %then
  %isBig14 = load i1, ptr %isBig, align 1
  %if_cond15 = icmp ne i1 %isBig14, false
  br i1 %if_cond15, label %then11, label %else12

else7:                                            ; preds = %then
  %n19 = load i32, ptr %n, align 4
  %eq20 = icmp eq i32 %n19, 0
  %if_cond21 = icmp ne i1 %eq20, false
  br i1 %if_cond21, label %then16, label %else17

merge8:                                           ; preds = %merge18, %merge13
  %phi23 = phi {} [ %phi, %merge13 ], [ %phi22, %merge18 ]
  br label %merge

then11:                                           ; preds = %then6
  store i32 1, ptr %result, align 4
  br label %merge13

else12:                                           ; preds = %then6
  store i32 2, ptr %result, align 4
  br label %merge13

merge13:                                          ; preds = %else12, %then11
  %phi = phi {} [ zeroinitializer, %then11 ], [ zeroinitializer, %else12 ]
  br label %merge8

then16:                                           ; preds = %else7
  store i32 5, ptr %result, align 4
  br label %merge18

else17:                                           ; preds = %else7
  store i32 3, ptr %result, align 4
  br label %merge18

merge18:                                          ; preds = %else17, %then16
  %phi22 = phi {} [ zeroinitializer, %then16 ], [ zeroinitializer, %else17 ]
  br label %merge8

then24:                                           ; preds = %else
  store i32 4, ptr %result, align 4
  br label %merge26

else25:                                           ; preds = %else
  br label %merge26

merge26:                                          ; preds = %else25, %then24
  %phi29 = phi {} [ zeroinitializer, %then24 ], [ zeroinitializer, %else25 ]
  br label %merge

then31:                                           ; preds = %merge
  store i32 6, ptr %result, align 4
  br label %merge33

else32:                                           ; preds = %merge
  store i32 7, ptr %result, align 4
  br label %merge33

merge33:                                          ; preds = %else32, %then31
  %phi38 = phi {} [ zeroinitializer, %then31 ], [ zeroinitializer, %else32 ]
  %n42 = load i32, ptr %n, align 4
  %lt = icmp slt i32 %n42, -50
  %if_cond43 = icmp ne i1 %lt, false
  br i1 %if_cond43, label %then39, label %else40

then39:                                           ; preds = %merge33
  store i32 8, ptr %result, align 4
  br label %merge41

else40:                                           ; preds = %merge33
  br label %merge41

merge41:                                          ; preds = %else40, %then39
  %phi44 = phi {} [ zeroinitializer, %then39 ], [ zeroinitializer, %else40 ]
  %result45 = load i32, ptr %result, align 4
  ret i32 %result45
}

define i32 @main({} %0) {
entry:
  %call = call i32 @categorizeNumber(i32 150)
  %call1 = call i32 @categorizeNumber(i32 150)
  call void @__print_int__(i32 %call1)
  %call2 = call i32 @categorizeNumber(i32 24)
  %call3 = call i32 @categorizeNumber(i32 24)
  call void @__print_int__(i32 %call3)
  %call4 = call i32 @categorizeNumber(i32 0)
  %call5 = call i32 @categorizeNumber(i32 0)
  call void @__print_int__(i32 %call5)
  %call6 = call i32 @categorizeNumber(i32 -12)
  %call7 = call i32 @categorizeNumber(i32 -12)
  call void @__print_int__(i32 %call7)
  %call8 = call i32 @categorizeNumber(i32 15)
  %call9 = call i32 @categorizeNumber(i32 15)
  call void @__print_int__(i32 %call9)
  %call10 = call i32 @categorizeNumber(i32 7)
  %call11 = call i32 @categorizeNumber(i32 7)
  call void @__print_int__(i32 %call11)
  %call12 = call i32 @categorizeNumber(i32 -3)
  %call13 = call i32 @categorizeNumber(i32 -3)
  call void @__print_int__(i32 %call13)
  %call14 = call i32 @categorizeNumber(i32 -75)
  %call15 = call i32 @categorizeNumber(i32 -75)
  call void @__print_int__(i32 %call15)
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
