using System.Numerics;
using ImGuiNET;
using Main = Aetherment.Aetherment;

namespace Aetherment.Gui.Window.Aetherment;

public class AethermentWindow {
	private Settings settings;
	private ModManager manager;
	private ModBrowser browser;
	private Explorer.Explorer explorer;
	private ModDev dev;
	
	public AethermentWindow() {
		settings = new();
		manager = new();
		browser = new();
		explorer = new();
		dev = new();
	}
	
	~AethermentWindow() {
		
	}
	
	public void Draw(ref bool enabled) {
		// ImGui.SetNextWindowSize(new Vector2(1070, 600));
		ImGui.SetNextWindowSize(new Vector2(1070, 600), ImGuiCond.FirstUseEver);
		ImGui.Begin("Aetherment", ref enabled);
		
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
		
		if(Main.Config.FileExplorer && Aeth.TabItem("File Explorer")) {
			ImGui.BeginChild("explorer");
			explorer.Draw();
			ImGui.EndChild();
		}
		
		if(Main.Config.ModDev && Aeth.TabItem("Mod Development")) {
			ImGui.BeginChild("dev", Vector2.Zero, false, ImGuiWindowFlags.NoScrollbar | ImGuiWindowFlags.NoScrollWithMouse);
			dev.Draw();
			ImGui.EndChild();
		}
		
		Aeth.EndTabBar();
		ImGui.End();
	}
}