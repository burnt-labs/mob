# JNA and the UniFFI bindings are reached through reflection and native callbacks.
-dontwarn java.awt.**
-keep class com.sun.jna.** { *; }
-keep class * implements com.sun.jna.** { *; }
-keep class uniffi.mob.** { *; }
