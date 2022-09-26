import 'dart:ffi';
import 'dart:io';

import 'package:flutter_rust_plugin/bridge_generated.dart';

class FlutterRustPlugin {
  static const base = 'rust_part';
  static final path = Platform.isWindows? '$base.dll' : 'lib$base.so';
  static final dylib = Platform.isIOS
      ? DynamicLibrary.process()
      : Platform.isMacOS
      ? DynamicLibrary.executable()
      : DynamicLibrary.open(path);
  static final api = RustPartImpl(dylib);

  static Future<String> hello() async{
    var hello = await api.hello();
    return hello;
  }
}
