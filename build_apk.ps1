$env:ANDROID_HOME = "D:\Android\Sdk"
$env:ANDROID_NDK_ROOT = "D:\Android\Sdk\ndk\25.2.9519653"
$env:JAVA_HOME = "C:\Program Files\Java\jdk-17"
Set-Location "D:\GITHUB\ALVR\alvr\client_openxr"
cargo apk build --release
