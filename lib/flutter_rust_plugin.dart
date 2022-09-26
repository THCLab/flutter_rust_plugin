
import 'flutter_rust_plugin_platform_interface.dart';

class FlutterRustPlugin {
  Future<String?> getPlatformVersion() {
    return FlutterRustPluginPlatform.instance.getPlatformVersion();
  }
}
