using System.Numerics;
using ImGuiNET;

namespace Aetherment.Gui.Window.Aetherment;

public class AethermentWindow {
	private Settings settings;
	private ModManager manager;
	private ModBrowser browser;
	
	public AethermentWindow() {
		settings = new();
		manager = new();
		browser = new();
	}
	
	~AethermentWindow() {
		
	}
	
	public void Draw() {
		ImGui.SetNextWindowSize(new Vector2(1070, 600));
		// ImGui.SetNextWindowSize(new Vector2(1070, 600), ImGuiCond.FirstUseEver);
		ImGui.Begin("Aetherment");
		
		ImGui.PushStyleVar(ImGuiStyleVar.ItemSpacing, new Vector2(0, 0));
		Aeth.BeginTabBar("tabs");
		ImGui.PopStyleVar();
		
		if(Aeth.TabItem("Settings")) {
			ImGui.BeginChild("settings");
			settings.Draw();
			ImGui.EndChild();
		}
		
		if(Aeth.TabItem("Mod Manager")) {
			ImGui.BeginChild("manager");
			manager.Draw();
			ImGui.EndChild();
		}
		
		if(Aeth.TabItem("Mod Browser")) {
			ImGui.BeginChild("browser");
			browser.Draw();
			ImGui.EndChild();
		}
		
		Aeth.EndTabBar();
		ImGui.End();
	}
}