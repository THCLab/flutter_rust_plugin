import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'flutter_rust_plugin_method_channel.dart';

abstract class FlutterRustPluginPlatform extends PlatformInterface {
  /// Constructs a FlutterRustPluginPlatform.
  FlutterRustPluginPlatform() : super(token: _token);

  static final Object _token = Object();

  static FlutterRustPluginPlatform _instance = MethodChannelFlutterRustPlugin();

  /// The default instance of [FlutterRustPluginPlatform] to use.
  ///
  /// Defaults to [MethodChannelFlutterRustPlugin].
  static FlutterRustPluginPlatform get instance => _instance;
  
  /// Platform-specific implementations should set this with their own
  /// platform-specific class that extends [FlutterRustPluginPlatform] when
  /// they register themselves.
  static set instance(FlutterRustPluginPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  Future<String?> getPlatformVersion() {
    throw UnimplementedError('platformVersion() has not been implemented.');
  }
}
