using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using Dalamud.Game.ClientState.Objects.Types;
using Dalamud.Plugin.Ipc;

namespace Aetherment;

// https://github.com/Ottermandias/Penumbra.Api/blob/9472b6e327109216368c3dc1720159f5295bdb13/IPenumbraApi.cs
public unsafe class Penumbra : IDisposable {
	// private ICallGateSubscriber<string, object> postSettingsDraw;
	
	public Penumbra() {
		redraw = Redraw;
		redrawSelf = RedrawSelf;
		rootPath = RootPath;
		modList = ModList;
		addModEntry = AddModEntry;
		reloadMod = ReloadMod;
		setModEnabled = SetModEnabled;
		setModPriority = SetModPriority;
		setModInherit = SetModInherit;
		setModSettings = SetModSettings;
		defaultCollection = DefaultCollection;
		
		// postSettingsDraw = Aetherment.Interface.GetIpcSubscriber<string, object>("Penumbra.PostSettingsDraw");
		// postSettingsDraw.Subscribe(DrawSettings);
		
		// mare doesnt seem to sync other temp mods
		// var paths = new Dictionary<string, string> {
		// 	["chara/human/c1401/obj/face/f0001/texture/--c1401f0001_fac_d.tex"] = "D:/ffxiv/aetherment/test/test_face.tex",
		// };
		// Aetherment.Interface.GetIpcSubscriber<string, string, Dictionary<string, string>, string, int, byte>("Penumbra.AddTemporaryMod").InvokeFunc("aeth_test", "Me", paths, "", 50);
		// Aetherment.Interface.GetIpcSubscriber<string, string, int, byte>("Penumbra.RemoveTemporaryMod").InvokeFunc("aeth_test", "Me", 50);
	}
	
	public void Dispose() {
		// postSettingsDraw.Unsubscribe(DrawSettings);
	}
	
	// private static void DrawSettings(string id) {
	// 	if(Aetherment.state == IntPtr.Zero) return;
	// 	
	// 	try {
	// 		draw_settings(Aetherment.state, id);
	// 	} catch(Exception e) {
	// 		PluginLog.Error("draw_settings somehow paniced, even tho it's supposed to catch those, wtf", e);
	// 	}
	// }
	
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
	
	public RootPathDelegate rootPath;
	public delegate FFI.Str RootPathDelegate();
	public FFI.Str RootPath() {
		return Aetherment.Interface.GetIpcSubscriber<string>("Penumbra.GetModDirectory").InvokeFunc();
	}
	
	// TODO: this might return a string thats longer than the ffi.str allocated buffer if the user has an insane amount of mods. look into it
	public ModListDelegate modList;
	public delegate FFI.Str ModListDelegate();
	public FFI.Str ModList() {
		var mods = Aetherment.Interface.GetIpcSubscriber<IList<(string, string)>>("Penumbra.GetMods").InvokeFunc();
		var mods_str = ""; // should use a mutable string but idc, fuck c#
		for(int i = 0; i < mods.Count; i++) {
			if(i > 0)
				mods_str += "\0";
			mods_str += mods[i].Item1;
		}
		
		return mods_str;
	}
	
	public AddModEntryDelegate addModEntry;
	public delegate byte AddModEntryDelegate(FFI.Str id);
	public byte AddModEntry(FFI.Str id) {
		return Aetherment.Interface.GetIpcSubscriber<string, byte>("Penumbra.AddMod").InvokeFunc(id);
	}
	
	public ReloadModDelegate reloadMod;
	public delegate byte ReloadModDelegate(FFI.Str id);
	public byte ReloadMod(FFI.Str id) {
		return Aetherment.Interface.GetIpcSubscriber<string, string, byte>("Penumbra.ReloadMod").InvokeFunc(id, "");
	}
	
	public SetModEnabledDelegate setModEnabled;
	public delegate byte SetModEnabledDelegate(FFI.Str collection, FFI.Str mod, byte enabled);
	public byte SetModEnabled(FFI.Str collection, FFI.Str mod, byte enabled) {
		return Aetherment.Interface.GetIpcSubscriber<string, string, string, bool, byte>("Penumbra.TrySetMod").InvokeFunc(collection, mod, "", enabled != 0);
	}
	
	public SetModPriorityDelegate setModPriority;
	public delegate byte SetModPriorityDelegate(FFI.Str collection, FFI.Str mod, int priority);
	public byte SetModPriority(FFI.Str collection, FFI.Str mod, int priority) {
		return Aetherment.Interface.GetIpcSubscriber<string, string, string, int, byte>("Penumbra.TrySetModPriority").InvokeFunc(collection, mod, "", priority);
	}
	
	public SetModInheritDelegate setModInherit;
	public delegate byte SetModInheritDelegate(FFI.Str collection, FFI.Str mod, byte inherit);
	public byte SetModInherit(FFI.Str collection, FFI.Str mod, byte inherit) {
		return Aetherment.Interface.GetIpcSubscriber<string, string, string, int, byte>("Penumbra.TryInheritMod").InvokeFunc(collection, mod, "", inherit);
	}
	
	public SetModSettingsDelegate setModSettings;
	public delegate byte SetModSettingsDelegate(FFI.Str collection, FFI.Str mod, FFI.Str option, FFI.Str sub_options_str);
	public byte SetModSettings(FFI.Str collection, FFI.Str mod, FFI.Str option, FFI.Str sub_options_str) {
		var sub_options = new List<string>();
		foreach(var sub_option in sub_options_str.ToString().Split('\0'))
			if(sub_option.Length > 0)
				sub_options.Add(sub_option);
		
		if(sub_options.Count == 1)
			return Aetherment.Interface.GetIpcSubscriber<string, string, string, string, string, byte>("Penumbra.TrySetModSetting").InvokeFunc(collection, mod, "", option, sub_options[0]);
		else
			return Aetherment.Interface.GetIpcSubscriber<string, string, string, string, IReadOnlyList<string>, byte>("Penumbra.TrySetModSettings").InvokeFunc(collection, mod, "", option, sub_options);
	}
	
	public DefaultCollectionDelegate defaultCollection;
	public delegate FFI.Str DefaultCollectionDelegate();
	public FFI.Str DefaultCollection() {
		return Aetherment.Interface.GetIpcSubscriber<string>("Penumbra.GetDefaultCollectionName").InvokeFunc();
	}
}