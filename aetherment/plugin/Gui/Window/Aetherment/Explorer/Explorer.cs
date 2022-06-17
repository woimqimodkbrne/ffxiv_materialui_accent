using System.Numerics;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using ImGuiNET;
using Main = Aetherment.Aetherment;

namespace Aetherment.Gui.Window.Aetherment.Explorer;

public class Explorer {
	private Tree gameTree;
	private bool populatedGameTree = false;
	private bool populatingGameTree = false;
	
	private Viewer.Viewer? viewer;
	
	public Explorer() {
		gameTree = new("Game Files", OpenFile);
	}
	
	private void OpenFile(string path) {
		PluginLog.Log(path);
		var ext = path.Split(".").Last();
		if(ext == "tex")
			viewer = new Viewer.Tex(path);
		else
			viewer = null;
	}
	
	public void Draw() {
		if(!populatedGameTree) {
			populatedGameTree = true;
			Task.Run(() => {
				populatingGameTree = true;
				foreach(var path in File.ReadLines(Main.Interface.AssemblyLocation.DirectoryName + "/assets/paths"))
					gameTree.AddNode(path);
				populatingGameTree = false;
			});
		}
		
		ImGui.BeginTable("##divider", 2, ImGuiTableFlags.Resizable);
		ImGui.TableSetupColumn("##tree", ImGuiTableColumnFlags.WidthFixed, 200);
		ImGui.TableSetupColumn("##viewer", ImGuiTableColumnFlags.WidthStretch);
		ImGui.TableNextRow();
		
		// Trees
		ImGui.TableNextColumn();
		ImGui.BeginChild("tree", new Vector2(0, -Aeth.FrameHeight - Aeth.S.ItemSpacing.Y));
		gameTree.Draw();
		if(populatingGameTree)
			ImGui.Text("Populating Tree..."); // TODO: fancy loading icon
		ImGui.EndChild();
		
		// Viewer
		ImGui.TableNextColumn();
		ImGui.BeginChild("viewer", new Vector2(0, -Aeth.FrameHeight - Aeth.S.ItemSpacing.Y));
		if(viewer != null) {
			viewer.Draw();
		}
		ImGui.EndChild();
		
		ImGui.EndTable();
	}
}