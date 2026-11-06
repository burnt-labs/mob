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

  s.source_files   = '*.swift', 'generated/mob.swift', 'generated/NativeHttpTransport.swift'

  s.preserve_paths = 'Frameworks/static', 'generated/mobFFI.h', 'generated/module.modulemap'

  # Copy the static library built by scripts/build-ios.sh for this platform into the
  # pod's build products directory, where the linker can find it, along with the mobFFI
  # module that the app target needs to import this pod's Swift interface.
  s.script_phase = {
    name: 'Select libmob architecture',
    script: <<~SCRIPT,
      if [[ "$PLATFORM_NAME" == "iphonesimulator" ]]; then
        SRC="${PODS_TARGET_SRCROOT}/Frameworks/static/simulator/libmob.a"
      else
        SRC="${PODS_TARGET_SRCROOT}/Frameworks/static/device/libmob.a"
      fi
      if [ ! -f "$SRC" ]; then
        echo "error: libmob.a not found at $SRC. Run scripts/build-ios.sh first."
        exit 1
      fi
      cp "$SRC" "${BUILT_PRODUCTS_DIR}/libmob.a"
      mkdir -p "${BUILT_PRODUCTS_DIR}/mobFFI"
      cp "${PODS_TARGET_SRCROOT}/generated/mobFFI.h" "${PODS_TARGET_SRCROOT}/generated/module.modulemap" "${BUILT_PRODUCTS_DIR}/mobFFI/"
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
  # The include path lets the app resolve the mobFFI clang module this pod's Swift module imports.
  s.user_target_xcconfig = {
    'OTHER_LDFLAGS' => '$(inherited) -lmob',
    'SWIFT_INCLUDE_PATHS' => '$(inherited) "${PODS_CONFIGURATION_BUILD_DIR}/MobModule/mobFFI"',
  }

  s.dependency 'ExpoModulesCore'
end
