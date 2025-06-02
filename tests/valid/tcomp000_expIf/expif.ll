; ModuleID = './tests/valid/tcomp000_expIf/expif.agu'
source_filename = "./tests/valid/tcomp000_expIf/expif.agu"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@.str = private unnamed_addr constant [3 x i8] c"%d\00", align 1
@.str.1 = private unnamed_addr constant [5 x i8] c"true\00", align 1
@.str.2 = private unnamed_addr constant [6 x i8] c"false\00", align 1
@.str.3 = private unnamed_addr constant [5 x i8] c"unit\00", align 1
@.str.4 = private unnamed_addr constant [17 x i8] c"division by zero\00", align 1
@ola = global i1 false

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
  %ola = load i1, ptr @ola, align 1
  %if_cond = icmp ne i1 %ola, false
  br i1 %if_cond, label %then1, label %else2

then:                                             ; preds = %merge3
  %ola10 = load i1, ptr @ola, align 1
  %if_cond11 = icmp ne i1 %ola10, false
  br i1 %if_cond11, label %then7, label %else8

else:                                             ; preds = %merge3
  %ola18 = load i1, ptr @ola, align 1
  %if_cond19 = icmp ne i1 %ola18, false
  br i1 %if_cond19, label %then15, label %else16

merge:                                            ; preds = %merge17, %merge9
  %phi23 = phi i1 [ %phi14, %merge9 ], [ %phi22, %merge17 ]
  %ola30 = load i1, ptr @ola, align 1
  %if_cond31 = icmp ne i1 %ola30, false
  br i1 %if_cond31, label %then27, label %else28

then1:                                            ; preds = %entry
  %ola4 = load i1, ptr @ola, align 1
  br label %merge3

else2:                                            ; preds = %entry
  %ola5 = load i1, ptr @ola, align 1
  br label %merge3

merge3:                                           ; preds = %else2, %then1
  %phi = phi i1 [ %ola4, %then1 ], [ %ola5, %else2 ]
  %if_cond6 = icmp ne i1 %phi, false
  br i1 %if_cond6, label %then, label %else

then7:                                            ; preds = %then
  %ola12 = load i1, ptr @ola, align 1
  br label %merge9

else8:                                            ; preds = %then
  %ola13 = load i1, ptr @ola, align 1
  br label %merge9

merge9:                                           ; preds = %else8, %then7
  %phi14 = phi i1 [ %ola12, %then7 ], [ %ola13, %else8 ]
  br label %merge

then15:                                           ; preds = %else
  %ola20 = load i1, ptr @ola, align 1
  br label %merge17

else16:                                           ; preds = %else
  %ola21 = load i1, ptr @ola, align 1
  br label %merge17

merge17:                                          ; preds = %else16, %then15
  %phi22 = phi i1 [ %ola20, %then15 ], [ %ola21, %else16 ]
  br label %merge

then24:                                           ; preds = %merge29
  %ola39 = load i1, ptr @ola, align 1
  %if_cond40 = icmp ne i1 %ola39, false
  br i1 %if_cond40, label %then36, label %else37

else25:                                           ; preds = %merge29
  %ola47 = load i1, ptr @ola, align 1
  %if_cond48 = icmp ne i1 %ola47, false
  br i1 %if_cond48, label %then44, label %else45

merge26:                                          ; preds = %merge46, %merge38
  %phi52 = phi i1 [ %phi43, %merge38 ], [ %phi51, %merge46 ]
  call void @__print_bool__(i1 %phi52)
  ret i32 0

then27:                                           ; preds = %merge
  %ola32 = load i1, ptr @ola, align 1
  br label %merge29

else28:                                           ; preds = %merge
  %ola33 = load i1, ptr @ola, align 1
  br label %merge29

merge29:                                          ; preds = %else28, %then27
  %phi34 = phi i1 [ %ola32, %then27 ], [ %ola33, %else28 ]
  %if_cond35 = icmp ne i1 %phi34, false
  br i1 %if_cond35, label %then24, label %else25

then36:                                           ; preds = %then24
  %ola41 = load i1, ptr @ola, align 1
  br label %merge38

else37:                                           ; preds = %then24
  %ola42 = load i1, ptr @ola, align 1
  br label %merge38

merge38:                                          ; preds = %else37, %then36
  %phi43 = phi i1 [ %ola41, %then36 ], [ %ola42, %else37 ]
  br label %merge26

then44:                                           ; preds = %else25
  %ola49 = load i1, ptr @ola, align 1
  br label %merge46

else45:                                           ; preds = %else25
  %ola50 = load i1, ptr @ola, align 1
  br label %merge46

merge46:                                          ; preds = %else45, %then44
  %phi51 = phi i1 [ %ola49, %then44 ], [ %ola50, %else45 ]
  br label %merge26
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
