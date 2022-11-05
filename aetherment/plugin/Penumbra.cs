using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using Dalamud.Game.ClientState.Objects.Types;
using Dalamud.Plugin.Ipc;

namespace Aetherment;

public class Penumbra : IDisposable {
	private ICallGateSubscriber<string, object> postSettingsDraw;
	
	public Penumbra() {
		redraw = Redraw;
		redrawSelf = RedrawSelf;
		addTempMod = AddTempMod;
		removeTempMod = RemoveTempMod;
		addModEntry = AddModEntry;
		
		postSettingsDraw = Aetherment.Interface.GetIpcSubscriber<string, object>("Penumbra.PostSettingsDraw");
		postSettingsDraw.Subscribe(DrawSettings);
	}
	
	public void Dispose() {
		postSettingsDraw.Unsubscribe(DrawSettings);
	}
	
	private static void DrawSettings(string id) {
		if(Aetherment.state == IntPtr.Zero) return;
		
		draw_settings(Aetherment.state, id);
	}
	
	public RedrawDelegate redraw;
	public delegate void RedrawDelegate();
	public void Redraw() {
		Aetherment.Interface.GetIpcSubscriber<byte, object>("Penumbra.RedrawAll").InvokeAction(0);
	}
	
	public RedrawDelegate redrawSelf;
	public delegate void RedrawSelfDelegate();
	public void RedrawSelf() {
		Aetherment.Interface.GetIpcSubscriber<GameObject, byte, object>("Penumbra.RedrawObject").InvokeAction(Aetherment.Objects[0]!, 0);
	}
	
	public AddTempModDelegate addTempMod;
	public delegate byte AddTempModDelegate(FFI.String id, FFI.String paths, FFI.String manip, int priority);
	public byte AddTempMod(FFI.String id, FFI.String paths, FFI.String manip, int priority) {
		var pathsd = new Dictionary<string, string>();
		var pairs = ((string)paths).Split("\0\0");
		if(pairs[0].Length > 0)
			foreach(var p in pairs) {
				var v = p.Split("\0");
				pathsd[v[0]] = v[1];
			}
		return Aetherment.Interface.GetIpcSubscriber<string, Dictionary<string, string>, string, int, byte>("Penumbra.AddTemporaryModAll").InvokeFunc(id, pathsd, manip, priority);
	}
	
	public RemoveTempModDelegate removeTempMod;
	public delegate byte RemoveTempModDelegate(FFI.String id, int priority);
	public byte RemoveTempMod(FFI.String id, int priority) {
		return Aetherment.Interface.GetIpcSubscriber<string, int, byte>("Penumbra.RemoveTemporaryModAll").InvokeFunc(id, priority);
	}
	
	public AddModEntryDelegate addModEntry;
	public delegate byte AddModEntryDelegate(FFI.String id);
	public byte AddModEntry(FFI.String id) {
		Aetherment.Interface.GetIpcSubscriber<string, byte>("Penumbra.AddMod").InvokeFunc(id);
		Aetherment.Interface.GetIpcSubscriber<string, string, byte>("Penumbra.ReloadMod").InvokeFunc(id, "");
		return 0; // eh w/e, idk about what it returns
	}
	
	// Returning FFI.Str seems to not work on the rust side, idk why, cba figuring out why
	public string RootPath() {
		return Aetherment.Interface.GetIpcSubscriber<string>("Penumbra.GetModDirectory").InvokeFunc();
	}
	
	[DllImport("aetherment_core.dll")] private static extern void draw_settings(IntPtr state, FFI.Str id);
}