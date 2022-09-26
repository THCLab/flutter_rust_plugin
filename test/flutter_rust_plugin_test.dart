import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_rust_plugin/flutter_rust_plugin.dart';
import 'package:flutter_rust_plugin/flutter_rust_plugin_platform_interface.dart';
import 'package:flutter_rust_plugin/flutter_rust_plugin_method_channel.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';

class MockFlutterRustPluginPlatform 
    with MockPlatformInterfaceMixin
    implements FlutterRustPluginPlatform {

  @override
  Future<String?> getPlatformVersion() => Future.value('42');
}

void main() {
  final FlutterRustPluginPlatform initialPlatform = FlutterRustPluginPlatform.instance;

  test('$MethodChannelFlutterRustPlugin is the default instance', () {
    expect(initialPlatform, isInstanceOf<MethodChannelFlutterRustPlugin>());
  });

  test('getPlatformVersion', () async {
    FlutterRustPlugin flutterRustPlugin = FlutterRustPlugin();
    MockFlutterRustPluginPlatform fakePlatform = MockFlutterRustPluginPlatform();
    FlutterRustPluginPlatform.instance = fakePlatform;
  
    expect(await flutterRustPlugin.getPlatformVersion(), '42');
  });
}
