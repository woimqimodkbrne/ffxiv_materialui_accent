global using Dalamud.Logging;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Numerics;
using System.Reflection;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Dalamud.Game.ClientState.Objects;
using Dalamud.Game.Command;
using Dalamud.Interface.Internal.Notifications;
using Dalamud.IoC;
using Dalamud.Plugin;

namespace Aetherment;

public class Aetherment : IDalamudPlugin {
	public string Name => "Aetherment";
	
	[PluginService][RequiredVersion("1.0")] public static DalamudPluginInterface Interface {get; private set;} = null!;
	[PluginService][RequiredVersion("1.0")] public static CommandManager         Commands  {get; private set;} = null!;
	[PluginService][RequiredVersion("1.0")] public static ObjectTable            Objects   {get; private set;} = null!;
	// [PluginService][RequiredVersion("1.0")] public static TitleScreenMenu        TitleMenu  {get; private set;} = null!;
	
	public static SharpDX.Direct3D11.Device Device => Interface.UiBuilder.Device;
	private const string maincommand = "/aetherment";
	
	private IntPtr state;
	private TextureManager textureManager;
	
	private bool isUnloading = false;
	private FileSystemWatcher? watcher;
	
	[StructLayout(LayoutKind.Sequential)]
	private unsafe struct Initializers {
		public IntPtr log;
		
		public IntPtr t_c;
		public IntPtr t_d;
		public IntPtr t_p;
		public IntPtr t_u;
	}
	
	public unsafe Aetherment() {
		log = Log;
		textureManager = new();
		
		var init = new Initializers {
			log = Marshal.GetFunctionPointerForDelegate(log),
			
			t_c = Marshal.GetFunctionPointerForDelegate(textureManager.createTexture),
			t_d = Marshal.GetFunctionPointerForDelegate(textureManager.destroyResource),
			t_p = Marshal.GetFunctionPointerForDelegate(textureManager.pinData),
			t_u = Marshal.GetFunctionPointerForDelegate(textureManager.unpinData),
		};
		
		state = initialize(init);
		
		Interface.UiBuilder.Draw += Draw;
		Commands.AddHandler(maincommand, new CommandInfo(OnCommand) {
			HelpMessage = "Open Aetherment menu"
		});
		
		// Reload if the rust part changes
		// if(Interface.IsDev) {
		// 	watcher = new FileSystemWatcher($"{Interface.AssemblyLocation.DirectoryName}", "aetherment_core.dll");
		// 	watcher.NotifyFilter = NotifyFilters.LastWrite;
		// 	watcher.Changed += (object _, FileSystemEventArgs e) => {
		// 		watcher.EnableRaisingEvents = false;
		// 		Task.Run(() => {
		// 			Task.Delay(1000);
		// 			ReloadPlugin();
		// 		});
		// 	};
		// 	watcher.EnableRaisingEvents = true;
		// }
	}
	//
	public void Dispose() {
		Interface.UiBuilder.Draw -= Draw;
		Commands.RemoveHandler(maincommand);
		FFI.Str.Drop();
		if(watcher != null)
			watcher.Dispose();
		destroy(state);
		state = IntPtr.Zero;
	}
	
	private void Draw() {
		try {
			draw(state);
		} catch {}
	}
	
	private void OnCommand(string cmd, string args) {
		if(cmd != maincommand)
			return;
		
		command(state, args);
	}
	
	#pragma warning disable CS8600,CS8602,CS8603,CS8604 // shhh
	private object GetPluginInstance() {
		var ass = typeof(Dalamud.ClientLanguage).Assembly;
		var typemanager = ass.GetType("Dalamud.Plugin.Internal.PluginManager");
		var typeplugin = ass.GetType("Dalamud.Plugin.Internal.Types.LocalPlugin");
		var manager = ass.GetType("Dalamud.Service`1").MakeGenericType(typemanager)
			.GetMethod("Get").Invoke(null, BindingFlags.Default, null, new object[] {}, null);
		var plugins = (IEnumerable)typemanager.GetProperty("InstalledPlugins").GetValue(manager);
		foreach(var p in plugins) {
			if((string)typeplugin.GetProperty("Name").GetValue(p) == "Aetherment")
				return p;
		}
		
		return null;
	}
	
	private void UnloadPlugin() {
		var typeplugin = typeof(Dalamud.ClientLanguage).Assembly
			.GetType("Dalamud.Plugin.Internal.Types.LocalPlugin");
		
		typeplugin.GetMethod("Unload")
			.Invoke(GetPluginInstance(), BindingFlags.Default, null, new object[1] {false}, null);
		
		typeplugin.GetMethod("Disable")
			.Invoke(GetPluginInstance(), BindingFlags.Default, null, new object[] {}, null);
	}
	
	private async void ReloadPlugin() {
		var typeplugin = typeof(Dalamud.ClientLanguage).Assembly
			.GetType("Dalamud.Plugin.Internal.Types.LocalPlugin");
		
		try {
			await ((Task)typeplugin.GetMethod("ReloadAsync")
				.Invoke(GetPluginInstance(), BindingFlags.Default, null, new object[] {}, null)).ConfigureAwait(false);
		} catch(Exception e) {
			PluginLog.Error(e, "Failed reloading");
		}
	}
	#pragma warning restore CS8600,CS8602,CS8603,CS8604
	
	private void Kill(string content, byte depthStrip) {
		if(isUnloading)
			return;
		
		var frames = new StackTrace(true).GetFrames();
		var stack = new List<string>();
		for(int i = depthStrip; i < frames.Length; i++)
			// we dont care about the stack produced by ffi functions themselves
			// or by functions outside our own assembly
			if(frames[i].GetFileLineNumber() > 0 && frames[i].GetMethod()?.Module == typeof(Aetherment).Module)
				stack.Add($"\tat {frames[i].GetMethod()}, {frames[i].GetFileName()}:{frames[i].GetFileLineNumber()}:{frames[i].GetFileColumnNumber()}");
		PluginLog.Error($"\n\t{content}\n{string.Join("\n", stack)}");
		
		isUnloading = true;
		Interface.UiBuilder.AddNotification("Aetherment has encountered an error and has been unloaded", null, NotificationType.Error, 5000);
		UnloadPlugin();
	}
	
	private LogDelegate log;
	// private unsafe delegate void LogDelegate(byte mod, byte* ptr, ulong len, ulong cap);
	// private unsafe void Log(byte mode, byte* ptr, ulong len, ulong cap) {
	// 	if(mode == 255)
	// 		Kill(System.Text.Encoding.UTF8.GetString(ptr, (int)len), 2);
	// 	else if(mode == 1)
	// 		PluginLog.Error(System.Text.Encoding.UTF8.GetString(ptr, (int)len));
	// 	else
	// 		PluginLog.Log(System.Text.Encoding.UTF8.GetString(ptr, (int)len));
	// 	
	// 	destroy_string(ptr, len, cap);
	// }
	private unsafe delegate void LogDelegate(byte mod, byte* ptr, ulong len);
	private unsafe void Log(byte mode, byte* ptr, ulong len) {
		if(mode == 255)
			Kill(FFI.Str.StrToString(ptr, len), 2);
		else if(mode == 1)
			PluginLog.Error(FFI.Str.StrToString(ptr, len));
		else
			PluginLog.Log(FFI.Str.StrToString(ptr, len));
	}
	
	// private LogDelegate log;
	// private unsafe delegate void LogDelegate(byte mode, FFI.Str str);
	// private unsafe void Log(byte mode, FFI.Str str) {
	// 	if(mode == 255)
	// 		Kill(str, 2);
	// 	else if(mode == 1)
	// 		PluginLog.Error(str);
	// 	else
	// 		PluginLog.Log(str);
	// }
	
	[DllImport("aetherment_core.dll")] private static extern unsafe IntPtr initialize(Initializers data);
	[DllImport("aetherment_core.dll")] private static extern unsafe void destroy(IntPtr state);
	// [DllImport("aetherment_core.dll")] private static extern unsafe void destroy_string(byte* ptr, ulong len, ulong cap);
	[DllImport("aetherment_core.dll")] private static extern unsafe void command(IntPtr state, FFI.Str args);
	[DllImport("aetherment_core.dll")] private static extern unsafe void draw(IntPtr state);
}