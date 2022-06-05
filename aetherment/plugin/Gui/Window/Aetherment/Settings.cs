using ImGuiNET;
using Main = Aetherment.Aetherment;

namespace Aetherment.Gui.Window.Aetherment;

public class Settings {
	public Settings() {
		
	}
	
	~Settings() {
		
	}
	
	public void Draw() {
		Aeth.BeginTabBar("tabs", false);
		
		Main.Config.MarkForChanges();
		
		if(Aeth.TabItem("Generic")) {
			
		}
		
		if(Aeth.TabItem("Advanced")) {
			ImGui.Checkbox("File Explorer", ref Main.Config.FileExplorer);
			ImGui.Checkbox("Mod Development", ref Main.Config.ModDev);
		}
		
		Main.Config.Save();
		
		Aeth.EndTabBar();
	}
}