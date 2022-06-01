global using Dalamud.Logging;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Collections.Immutable;
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
	
	public static Server.Server Server {get; private set;} = null!;
	
	private const string command = "/aetherment";
	
	private bool aethermentGuiVisible = true;
	private Gui.Window.Aetherment.AethermentWindow aethermentGui;
	
	private bool isUnloading = false;
	private FileSystemWatcher? watcher;
	
	public Aetherment() {
		logDelegate = Log;
		initialize(Marshal.GetFunctionPointerForDelegate(logDelegate));
		
		PluginLog.Log(cool_test("hello there c:"));
		PluginLog.Log(string.Join(" - ", (string[])cool_test2(new string[5]{"1", "2", "3", "4", "5"})));
		
		aethermentGui = new();
		
		Interface.UiBuilder.Draw += Draw;
		Commands.AddHandler(command, new CommandInfo(OnCommand) {
			HelpMessage = "Open Aetherment menu"
		});
		
		// Reload if the rust part changes
		if(Interface.IsDev) {
			watcher = new FileSystemWatcher($"{Interface.AssemblyLocation.DirectoryName}", "aetherment_core.dll");
			watcher.NotifyFilter = NotifyFilters.LastWrite;
			watcher.Changed += (object _, FileSystemEventArgs e) => {
				watcher.EnableRaisingEvents = false;
				Task.Run(()=> {
					Task.Delay(100);
					ReloadPlugin();
				});
			};
			watcher.EnableRaisingEvents = true;
		}
	}
	
	public void Dispose() {
		Interface.UiBuilder.Draw -= Draw;
		Commands.RemoveHandler(command);
		if(watcher != null)
			watcher.Dispose();
	}
	
	private void Draw() {
		try {
			if(aethermentGuiVisible)
				aethermentGui.Draw(ref aethermentGuiVisible);
		} catch(Exception e) {
			Kill(e.Message, 1);
		}
	}
	
	private void OnCommand(string cmd, string args) {
		try {
			if(cmd != command)
				return;
			
			if(args == "texfinder")
				return; //todo
			else if(args == "panic")
				panic("you asked for it");
			else if(args == "panic2")
				throw new Exception("ohoh");
			else
				aethermentGuiVisible = !aethermentGuiVisible;
		} catch(Exception e) {
			Kill(e.Message, 1);
		}
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
		Interface.UiBuilder.AddNotification("Aetherment as encountered an error and has been unloaded", null, NotificationType.Error);
		UnloadPlugin();
	}
	
	private LogDelegate logDelegate;
	private delegate void LogDelegate(byte mod, IntPtr contentptr);
	private void Log(byte mode, IntPtr contentptr) {
		var content = new FFI.String(contentptr); // have i told you yet how much i hate c#?
		if(mode == 255) {
			Kill(content, 2);
		} else
			PluginLog.Log(content);
	}
	
	[DllImport("aetherment_core.dll")] private static extern void initialize(IntPtr log);
	[DllImport("aetherment_core.dll")] private static extern FFI.String cool_test(FFI.Str str);
	[DllImport("aetherment_core.dll")] private static extern FFI.Vec cool_test2(FFI.Array str);
	[DllImport("aetherment_core.dll")] private static extern void panic(FFI.Str str);
}