plugins {
    kotlin("jvm") version "2.1.0"
    application
}

group = "com.burnt.mob"
version = "0.1.0"

repositories {
    mavenCentral()
}

val xionTypesJar = file("../../xion-types/kotlin/build/libs/xion-types-kotlin-0.0.1.jar")

dependencies {
    implementation(kotlin("stdlib"))
    implementation("net.java.dev.jna:jna:5.14.0")

    testImplementation(kotlin("test"))
    testImplementation("org.junit.jupiter:junit-jupiter:5.10.1")
    // Typed tests use xion-types built from the sibling checkout.
    testImplementation(files(xionTypesJar))
    testImplementation("com.google.protobuf:protobuf-java:4.31.1")
    testImplementation("com.google.protobuf:protobuf-kotlin:4.31.1")
    testImplementation("com.google.protobuf:protobuf-java-util:4.31.1")
}

sourceSets {
    main {
        kotlin {
            srcDir("lib/uniffi")
        }
        resources {
            srcDir("lib")
        }
    }
    test {
        kotlin {
            srcDir("src/test/kotlin")
        }
    }
}

tasks.test {
    doFirst {
        if (!xionTypesJar.exists()) {
            throw GradleException("Build ../xion-types/kotlin before running Mob typed tests")
        }
    }

    useJUnitPlatform()

    // Add library path for native library
    systemProperty("java.library.path", "${projectDir}/lib")

    // Only run integration tests if INTEGRATION env var is set
    val runIntegration = System.getenv("INTEGRATION") == "1"
    if (!runIntegration) {
        exclude("**/IntegrationTest*")
    }
}

kotlin {
    jvmToolchain(17)
}
