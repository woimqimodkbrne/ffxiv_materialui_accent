using System.Numerics;
using System.Runtime.InteropServices;
using Dalamud.Interface.Colors;
using ImGuiNET;

namespace Aetherment.Gui.Window.Aetherment.Explorer.Viewer;

public class Viewer {
	public string[] validImports;
	public string[] validExports;
	
	private string path;
	private string? error;
	
	public Viewer(string path) {
		this.path = path;
		validImports = new string[0];
		validExports = new string[0];
	}
	
	public void ShowError(string error) {
		this.error = error;
	}
	
	public void Draw() {
		if(error != null) {
			ImGui.Dummy(Vector2.Zero);
			ImGui.PushStyleColor(ImGuiCol.Text, ImGuiColors.DPSRed);
			Aeth.WrappedText(error, ImGui.GetContentRegionAvail());
			ImGui.PopStyleColor();
			
			return;
		}
		
		DrawViewer();
	}
	
	protected virtual void DrawViewer() {}
	public virtual void SaveFile(string filename, string format) {}
}