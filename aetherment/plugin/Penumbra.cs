using System.Collections.Generic;
using Dalamud.Game.ClientState.Objects.Types;

namespace Aetherment;

public class Penumbra {
	public Penumbra() {
		redraw = Redraw;
		redrawSelf = RedrawSelf;
		addTempMod = AddTempMod;
		removeTempMod = RemoveTempMod;
	}
	
	public RedrawDelegate redraw;
	public delegate void RedrawDelegate();
	public void Redraw() {
		Aetherment.Interface.GetIpcSubscriber<byte, object>("Penumbra.RedrawAll").InvokeAction(0);
	}
	
	public RedrawDelegate redrawSelf;
	public delegate void RedrawSelfDelegate();
	public void RedrawSelf() {
		// This doesnt redraw the mount!!!, TODO: make pr to penumbra to fix that. or bother otter, idk
		// Aetherment.Interface.GetIpcSubscriber<string, byte, object>("Penumbra.RedrawObject").InvokeAction("self", 0);
		
		Aetherment.Interface.GetIpcSubscriber<byte, object>("Penumbra.RedrawAll").InvokeAction(0);
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
}