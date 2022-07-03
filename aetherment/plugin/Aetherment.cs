global using Dalamud.Logging;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Reflection;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Dalamud.Game.Command;
using Dalamud.Interface.Internal.Notifications;
using Dalamud.IoC;
using Dalamud.Plugin;

namespace Aetherment;

public class Aetherment : IDalamudPlugin {
	public string Name => "Aetherment";
	
	[PluginService][RequiredVersion("1.0")] public static DalamudPluginInterface Interface {get; private set;} = null!;
	[PluginService][RequiredVersion("1.0")] public static CommandManager         Commands  {get; private set;} = null!;
	// [PluginService][RequiredVersion("1.0")] public static TitleScreenMenu        TitleMenu  {get; private set;} = null!;
	
	public static SharpDX.Direct3D11.Device Device => Interface.UiBuilder.Device;
	
	private const string maincommand = "/aetherment";
	
	private IntPtr state;
	
	private TextureManager textureManager;
	
	private bool isUnloading = false;
	private FileSystemWatcher? watcher;
	
	[StructLayout(LayoutKind.Sequential)]
	private unsafe struct Initializers {
		public FFI.Str binary_path;
		public FFI.Str config_path;
		public IntPtr log;
		public IntPtr t1;
		public IntPtr t2;
		public IntPtr t3;
		public IntPtr t4;
		public IntPtr t5;
	}
	
	public unsafe Aetherment() {
		logDelegate = Log;
		textureManager = new();
		
		var init = new Initializers {
			binary_path = Interface.AssemblyLocation.DirectoryName!,
			config_path = Interface.ConfigDirectory.FullName,
			log = Marshal.GetFunctionPointerForDelegate(logDelegate),
			t1 = Marshal.GetFunctionPointerForDelegate(textureManager.createTexture),
			t2 = Marshal.GetFunctionPointerForDelegate(textureManager.createTextureData),
			t3 = Marshal.GetFunctionPointerForDelegate(textureManager.destroyResource),
			t4 = Marshal.GetFunctionPointerForDelegate(textureManager.pinData),
			t5 = Marshal.GetFunctionPointerForDelegate(textureManager.unpinData),
		};
		
		state = initialize(init);
		
		Interface.UiBuilder.Draw += Draw;
		Commands.AddHandler(maincommand, new CommandInfo(OnCommand) {
			HelpMessage = "Open Aetherment menu"
		});
		
		// Reload if the rust part changes
		if(Interface.IsDev) {
			watcher = new FileSystemWatcher($"{Interface.AssemblyLocation.DirectoryName}", "aetherment_core.dll");
			watcher.NotifyFilter = NotifyFilters.LastWrite;
			watcher.Changed += (object _, FileSystemEventArgs e) => {
				watcher.EnableRaisingEvents = false;
				Task.Run(()=> {
					Task.Delay(500);
					ReloadPlugin();
				});
			};
			watcher.EnableRaisingEvents = true;
		}
	}
	
	public void Dispose() {
		destroy(state);
		Interface.UiBuilder.Draw -= Draw;
		Commands.RemoveHandler(maincommand);
		if(watcher != null)
			watcher.Dispose();
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
	
	private void ReloadPlugin() {
		var typeplugin = typeof(Dalamud.ClientLanguage).Assembly
			.GetType("Dalamud.Plugin.Internal.Types.LocalPlugin");
		
		typeplugin.GetMethod("Reload")
			.Invoke(GetPluginInstance(), BindingFlags.Default, null, new object[] {}, null);
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
	
	private LogDelegate logDelegate;
	private delegate void LogDelegate(byte mod, FFI.String content);
	private void Log(byte mode, FFI.String content) {
		if(mode == 255)
			Kill(content, 2);
		else if(mode == 1)
			PluginLog.Error(content);
		else
			PluginLog.Log(content);
	}
	
	[DllImport("aetherment_core.dll")] private static extern IntPtr initialize(Initializers data);
	[DllImport("aetherment_core.dll")] private static extern void destroy(IntPtr state);
	[DllImport("aetherment_core.dll")] private static extern void draw(IntPtr state);
	[DllImport("aetherment_core.dll")] private static extern void command(IntPtr state, FFI.Str args);
}