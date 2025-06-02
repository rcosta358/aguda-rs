; ModuleID = 'lib.c'
source_filename = "lib.c"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc19.39.33521"

$"??_C@_02DPKJAMEF@?$CFd?$AA@" = comdat any

$"??_C@_04LOAJBDKD@true?$AA@" = comdat any

$"??_C@_05LAPONLG@false?$AA@" = comdat any

$"??_C@_04NPEDKLDA@unit?$AA@" = comdat any

$"??_C@_0BB@OMEBCKB@division?5by?5zero?$AA@" = comdat any

@"??_C@_02DPKJAMEF@?$CFd?$AA@" = linkonce_odr dso_local unnamed_addr constant [3 x i8] c"%d\00", comdat, align 1
@"??_C@_04LOAJBDKD@true?$AA@" = linkonce_odr dso_local unnamed_addr constant [5 x i8] c"true\00", comdat, align 1
@"??_C@_05LAPONLG@false?$AA@" = linkonce_odr dso_local unnamed_addr constant [6 x i8] c"false\00", comdat, align 1
@"??_C@_04NPEDKLDA@unit?$AA@" = linkonce_odr dso_local unnamed_addr constant [5 x i8] c"unit\00", comdat, align 1
@"??_C@_0BB@OMEBCKB@division?5by?5zero?$AA@" = linkonce_odr dso_local unnamed_addr constant [17 x i8] c"division by zero\00", comdat, align 1

; Function Attrs: noinline nounwind optnone uwtable
define dso_local void @__print_int__(i32 noundef %0) #0 {
  %2 = alloca i32, align 4
  store i32 %0, ptr %2, align 4
  %3 = load i32, ptr %2, align 4
  %4 = call i32 (ptr, ...) @printf(ptr noundef @"??_C@_02DPKJAMEF@?$CFd?$AA@", i32 noundef %3)
  ret void
}

declare dso_local i32 @printf(ptr noundef, ...) #1

; Function Attrs: noinline nounwind optnone uwtable
define dso_local void @__print_bool__(i32 noundef %0) #0 {
  %2 = alloca i32, align 4
  store i32 %0, ptr %2, align 4
  %3 = load i32, ptr %2, align 4
  %4 = icmp ne i32 %3, 0
  %5 = zext i1 %4 to i64
  %6 = select i1 %4, ptr @"??_C@_04LOAJBDKD@true?$AA@", ptr @"??_C@_05LAPONLG@false?$AA@"
  %7 = call i32 (ptr, ...) @printf(ptr noundef %6)
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define dso_local void @__print_unit__() #0 {
  %1 = call i32 (ptr, ...) @printf(ptr noundef @"??_C@_04NPEDKLDA@unit?$AA@")
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @__pow__(i32 noundef %0, i32 noundef %1) #0 {
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  %5 = alloca i32, align 4
  store i32 %1, ptr %3, align 4
  store i32 %0, ptr %4, align 4
  store i32 1, ptr %5, align 4
  br label %6

6:                                                ; preds = %10, %2
  %7 = load i32, ptr %3, align 4
  %8 = add nsw i32 %7, -1
  store i32 %8, ptr %3, align 4
  %9 = icmp sgt i32 %7, 0
  br i1 %9, label %10, label %14

10:                                               ; preds = %6
  %11 = load i32, ptr %4, align 4
  %12 = load i32, ptr %5, align 4
  %13 = mul nsw i32 %12, %11
  store i32 %13, ptr %5, align 4
  br label %6, !llvm.loop !5

14:                                               ; preds = %6
  %15 = load i32, ptr %5, align 4
  ret i32 %15
}

; Function Attrs: noinline nounwind optnone uwtable
define dso_local i32 @__div__(i32 noundef %0, i32 noundef %1) #0 {
  %3 = alloca i32, align 4
  %4 = alloca i32, align 4
  store i32 %1, ptr %3, align 4
  store i32 %0, ptr %4, align 4
  %5 = load i32, ptr %3, align 4
  %6 = icmp eq i32 %5, 0
  br i1 %6, label %7, label %9

7:                                                ; preds = %2
  %8 = call i32 @write(i32 noundef 2, ptr noundef @"??_C@_0BB@OMEBCKB@division?5by?5zero?$AA@", i32 noundef 17)
  call void @exit(i32 noundef 1) #3
  unreachable

9:                                                ; preds = %2
  %10 = load i32, ptr %4, align 4
  %11 = load i32, ptr %3, align 4
  %12 = sdiv i32 %10, %11
  ret i32 %12
}

declare dso_local i32 @write(i32 noundef, ptr noundef, i32 noundef) #1

; Function Attrs: noreturn
declare dso_local void @exit(i32 noundef) #2

attributes #0 = { noinline nounwind optnone uwtable "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { noreturn "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cmov,+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}

!0 = !{i32 1, !"wchar_size", i32 2}
!1 = !{i32 8, !"PIC Level", i32 2}
!2 = !{i32 7, !"uwtable", i32 2}
!3 = !{i32 1, !"MaxTLSAlign", i32 65536}
!4 = !{!"clang version 17.0.6"}
!5 = distinct !{!5, !6}
!6 = !{!"llvm.loop.mustprogress"}
