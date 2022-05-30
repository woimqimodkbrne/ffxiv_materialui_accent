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
		ImGui.BeginTabBar("tabs");
		
		if(ImGui.BeginTabItem("Settings")) {
			ImGui.BeginChild("settings");
			settings.Draw();
			ImGui.EndChild();
			ImGui.EndTabItem();
		}
		
		if(ImGui.BeginTabItem("Mod Manager")) {
			ImGui.BeginChild("manager");
			manager.Draw();
			ImGui.EndChild();
			ImGui.EndTabItem();
		}
		
		if(ImGui.BeginTabItem("Mod Browser")) {
			ImGui.BeginChild("browser");
			browser.Draw();
			ImGui.EndChild();
			ImGui.EndTabItem();
		}
		
		ImGui.EndTabBar();
		ImGui.End();
	}
}