plugins {
    id("com.android.library")
    kotlin("android")
}

group = "com.burnt.mob"
version = "0.1.0"

android {
    namespace = "com.burnt.mob"
    compileSdk = rootProject.findProperty("compileSdkVersion")?.toString()?.toInt() ?: 34

    defaultConfig {
        minSdk = 24
        consumerProguardFiles("consumer-rules.pro")
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
            java.srcDirs(
                "src/main/java",
                "../../kotlin/src/main/kotlin"
            )
        }
    }
}

dependencies {
    implementation(project(":expo-modules-core"))
    implementation("net.java.dev.jna:jna:5.14.0@aar")
    implementation(kotlin("stdlib"))
}
