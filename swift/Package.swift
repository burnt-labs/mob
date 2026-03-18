// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "Mob",
    platforms: [
        .macOS(.v13),
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
            sources: ["mob.swift"],
            publicHeadersPath: "include"
        ),
        .testTarget(
            name: "MobTests",
            dependencies: ["Mob"],
            path: "Tests/MobTests"
        ),
    ]
)
