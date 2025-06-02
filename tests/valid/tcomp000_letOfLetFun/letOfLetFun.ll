; ModuleID = './tests/valid/tcomp000_letOfLetFun/letOfLetFun.agu'
source_filename = "./tests/valid/tcomp000_letOfLetFun/letOfLetFun.agu"
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
  %x = alloca i32, align 4
  store i32 5, ptr %x, align 4
  %x1 = alloca {}, align 8
  store {} zeroinitializer, ptr %x1, align 1
  %x2 = alloca {}, align 8
  store {} zeroinitializer, ptr %x2, align 1
  %x3 = alloca {}, align 8
  store {} zeroinitializer, ptr %x3, align 1
  %x4 = alloca {}, align 8
  store {} zeroinitializer, ptr %x4, align 1
  %x5 = alloca {}, align 8
  store {} zeroinitializer, ptr %x5, align 1
  %x6 = alloca {}, align 8
  store {} zeroinitializer, ptr %x6, align 1
  %x7 = alloca {}, align 8
  store {} zeroinitializer, ptr %x7, align 1
  %x8 = alloca {}, align 8
  store {} zeroinitializer, ptr %x8, align 1
  %x9 = alloca {}, align 8
  store {} zeroinitializer, ptr %x9, align 1
  %x10 = alloca {}, align 8
  store {} zeroinitializer, ptr %x10, align 1
  %x11 = alloca {}, align 8
  store {} zeroinitializer, ptr %x11, align 1
  %x12 = alloca {}, align 8
  store {} zeroinitializer, ptr %x12, align 1
  %x13 = alloca {}, align 8
  store {} zeroinitializer, ptr %x13, align 1
  %x14 = alloca {}, align 8
  store {} zeroinitializer, ptr %x14, align 1
  %x15 = alloca {}, align 8
  store {} zeroinitializer, ptr %x15, align 1
  %x16 = alloca {}, align 8
  store {} zeroinitializer, ptr %x16, align 1
  %x17 = alloca {}, align 8
  store {} zeroinitializer, ptr %x17, align 1
  %x18 = alloca {}, align 8
  store {} zeroinitializer, ptr %x18, align 1
  %x19 = alloca {}, align 8
  store {} zeroinitializer, ptr %x19, align 1
  %x20 = alloca {}, align 8
  store {} zeroinitializer, ptr %x20, align 1
  %x21 = alloca {}, align 8
  store {} zeroinitializer, ptr %x21, align 1
  %x22 = alloca {}, align 8
  store {} zeroinitializer, ptr %x22, align 1
  %x23 = alloca {}, align 8
  store {} zeroinitializer, ptr %x23, align 1
  %x24 = alloca {}, align 8
  store {} zeroinitializer, ptr %x24, align 1
  %x25 = alloca {}, align 8
  store {} zeroinitializer, ptr %x25, align 1
  %x26 = alloca {}, align 8
  store {} zeroinitializer, ptr %x26, align 1
  %x27 = alloca {}, align 8
  store {} zeroinitializer, ptr %x27, align 1
  %x28 = alloca {}, align 8
  store {} zeroinitializer, ptr %x28, align 1
  %x29 = alloca {}, align 8
  store {} zeroinitializer, ptr %x29, align 1
  %x30 = alloca {}, align 8
  store {} zeroinitializer, ptr %x30, align 1
  %x31 = alloca {}, align 8
  store {} zeroinitializer, ptr %x31, align 1
  %x32 = alloca {}, align 8
  store {} zeroinitializer, ptr %x32, align 1
  %x33 = alloca {}, align 8
  store {} zeroinitializer, ptr %x33, align 1
  %x34 = alloca {}, align 8
  store {} zeroinitializer, ptr %x34, align 1
  %x35 = alloca {}, align 8
  store {} zeroinitializer, ptr %x35, align 1
  %x36 = alloca {}, align 8
  store {} zeroinitializer, ptr %x36, align 1
  %x37 = alloca {}, align 8
  store {} zeroinitializer, ptr %x37, align 1
  %x38 = alloca {}, align 8
  store {} zeroinitializer, ptr %x38, align 1
  %x39 = alloca {}, align 8
  store {} zeroinitializer, ptr %x39, align 1
  %x40 = alloca {}, align 8
  store {} zeroinitializer, ptr %x40, align 1
  %x41 = alloca {}, align 8
  store {} zeroinitializer, ptr %x41, align 1
  %x42 = alloca {}, align 8
  store {} zeroinitializer, ptr %x42, align 1
  %x43 = alloca {}, align 8
  store {} zeroinitializer, ptr %x43, align 1
  %x44 = alloca {}, align 8
  store {} zeroinitializer, ptr %x44, align 1
  %x45 = alloca {}, align 8
  store {} zeroinitializer, ptr %x45, align 1
  %x46 = alloca {}, align 8
  store {} zeroinitializer, ptr %x46, align 1
  %x47 = alloca {}, align 8
  store {} zeroinitializer, ptr %x47, align 1
  %x48 = alloca {}, align 8
  store {} zeroinitializer, ptr %x48, align 1
  %x49 = alloca {}, align 8
  store {} zeroinitializer, ptr %x49, align 1
  %x50 = alloca {}, align 8
  store {} zeroinitializer, ptr %x50, align 1
  %x51 = alloca {}, align 8
  store {} zeroinitializer, ptr %x51, align 1
  %x52 = alloca {}, align 8
  store {} zeroinitializer, ptr %x52, align 1
  %x53 = alloca {}, align 8
  store {} zeroinitializer, ptr %x53, align 1
  %x54 = alloca {}, align 8
  store {} zeroinitializer, ptr %x54, align 1
  %x55 = alloca {}, align 8
  store {} zeroinitializer, ptr %x55, align 1
  %x56 = alloca {}, align 8
  store {} zeroinitializer, ptr %x56, align 1
  %x57 = alloca {}, align 8
  store {} zeroinitializer, ptr %x57, align 1
  %x58 = alloca {}, align 8
  store {} zeroinitializer, ptr %x58, align 1
  %x59 = alloca {}, align 8
  store {} zeroinitializer, ptr %x59, align 1
  %x60 = alloca {}, align 8
  store {} zeroinitializer, ptr %x60, align 1
  %x61 = alloca {}, align 8
  store {} zeroinitializer, ptr %x61, align 1
  %x62 = alloca {}, align 8
  store {} zeroinitializer, ptr %x62, align 1
  %x63 = alloca {}, align 8
  store {} zeroinitializer, ptr %x63, align 1
  %x64 = alloca {}, align 8
  store {} zeroinitializer, ptr %x64, align 1
  %x65 = alloca {}, align 8
  store {} zeroinitializer, ptr %x65, align 1
  %x66 = alloca {}, align 8
  store {} zeroinitializer, ptr %x66, align 1
  %x67 = load {}, ptr %x66, align 1
  %x68 = load {}, ptr %x66, align 1
  call void @__print_unit__({} %x68)
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
