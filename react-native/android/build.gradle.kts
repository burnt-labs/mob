plugins {
    kotlin("android") version "1.9.22"
    id("com.android.library")
}

group = "com.burnt.mob"
version = "0.1.0"

android {
    namespace = "com.burnt.mob"
    compileSdk = 34

    defaultConfig {
        minSdk = 24
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

    sourceSets {
        getByName("main") {
            java.srcDirs("src/main/java")
        }
    }
}

dependencies {
    implementation("expo:expo-modules-core:+")
    implementation("net.java.dev.jna:jna:5.14.0@aar")
    implementation(kotlin("stdlib"))
}
