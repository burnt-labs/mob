Pod::Spec.new do |s|
  s.name           = 'MobModule'
  s.version        = '0.1.0'
  s.summary        = 'Native mob module for React Native / Expo'
  s.description    = 'Expo module wrapping the mob Rust library for XION blockchain interaction'
  s.homepage       = 'https://github.com/burnt-labs/mob'
  s.license        = { type: 'MIT' }
  s.author         = 'Burnt Labs'
  s.source         = { git: 'https://github.com/burnt-labs/mob.git' }
  s.platform       = :ios, '16.0'
  s.swift_version  = '5.9'

  s.source_files   = '*.swift', 'generated/mob.swift', '../../swift/Sources/Mob/NativeHttpTransport.swift'

  s.preserve_paths = 'Frameworks/libmob.xcframework', 'generated/mobFFI.h', 'generated/module.modulemap'

  # Select the correct architecture slice from the XCFramework and copy it
  # to the pod's build products directory where the linker can find it.
  # This replaces vendored_frameworks which has issues with static library XCFrameworks.
  s.script_phase = {
    name: 'Select libmob architecture',
    script: <<~SCRIPT,
      if [[ "$PLATFORM_NAME" == "iphonesimulator" ]]; then
        SRC="${PODS_TARGET_SRCROOT}/Frameworks/libmob.xcframework/ios-arm64-simulator/libmob.a"
      else
        SRC="${PODS_TARGET_SRCROOT}/Frameworks/libmob.xcframework/ios-arm64/libmob.a"
      fi
      if [ ! -f "$SRC" ]; then
        echo "error: libmob.a not found at $SRC. Run scripts/build-ios.sh first."
        exit 1
      fi
      cp "$SRC" "${BUILT_PRODUCTS_DIR}/libmob.a"
    SCRIPT
    execution_position: :before_compile,
  }

  s.pod_target_xcconfig = {
    'SWIFT_INCLUDE_PATHS' => '$(inherited) "${PODS_TARGET_SRCROOT}/generated"',
    'OTHER_CFLAGS' => '$(inherited) -fmodule-map-file="${PODS_TARGET_SRCROOT}/generated/module.modulemap"',
    'OTHER_SWIFT_FLAGS' => '$(inherited) -Xcc -fmodule-map-file="${PODS_TARGET_SRCROOT}/generated/module.modulemap"',
    'HEADER_SEARCH_PATHS' => '$(inherited) "${PODS_TARGET_SRCROOT}/generated"',
  }

  # Inject -lmob into the app target's linker flags.
  # LIBRARY_SEARCH_PATHS already includes PODS_CONFIGURATION_BUILD_DIR/MobModule
  # (where the script_phase copies libmob.a) via standard CocoaPods pod dependency resolution.
  s.user_target_xcconfig = {
    'OTHER_LDFLAGS' => '$(inherited) -lmob',
  }

  s.dependency 'ExpoModulesCore'
end
