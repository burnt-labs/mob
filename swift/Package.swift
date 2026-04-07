// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "Mob",
    platforms: [
        .iOS(.v16)
    ],
    products: [
        .library(
            name: "Mob",
            targets: ["Mob"]
        ),
    ],
    targets: [
        .binaryTarget(
            name: "libmob",
            path: "lib/libmob.xcframework"
        ),
        .target(
            name: "Mob",
            dependencies: ["libmob"],
            path: "Sources/Mob",
            sources: ["mob.swift", "NativeHttpTransport.swift", "Compat.swift"]
        ),
        .testTarget(
            name: "MobTests",
            dependencies: ["Mob"],
            path: "Tests/MobTests"
        ),
    ]
)
