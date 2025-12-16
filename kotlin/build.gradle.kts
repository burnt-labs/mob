plugins {
    kotlin("jvm") version "1.9.22"
    application
}

group = "com.burnt.mob"
version = "0.1.0"

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
    implementation("net.java.dev.jna:jna:5.14.0")

    testImplementation(kotlin("test"))
    testImplementation("org.junit.jupiter:junit-jupiter:5.10.1")
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
