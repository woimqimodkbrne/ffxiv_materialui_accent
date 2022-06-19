using System.Numerics;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using ImGuiNET;
using Main = Aetherment.Aetherment;
using System.Runtime.InteropServices;
using Dalamud.Interface.Colors;

namespace Aetherment.Gui.Window.Aetherment.Explorer;

public class Explorer {
	private Tree gameTree;
	private bool populatedGameTree = false;
	private bool populatingGameTree = false;
	
	private Viewer.Viewer? viewer;
	
	private bool validPath = false;
	private string curPath = "";
	
	public Explorer() {
		gameTree = new("Game Files", OpenFile);
	}
	
	private void OpenFile(string path) {
		curPath = path;
		curPath = curPath.ToLower();
		validPath = IsValidPath(curPath).Unwrap<byte>() != 0;
		if(!validPath)
			return;
		
		var ext = curPath.Split(".").Last();
		if(ext == "tex")
			viewer = new Viewer.Tex(curPath);
		else if(ext == "mtrl")
			viewer = new Viewer.Mtrl(curPath);
		else
			viewer = null;
		
		gameTree.SelectedPath = path;
	}
	
	public void Draw() {
		if(!populatedGameTree) {
			populatedGameTree = true;
			Task.Run(() => {
				populatingGameTree = true;
				foreach(var path in File.ReadLines(Main.Interface.AssemblyLocation.DirectoryName + "/assets/paths"))
					gameTree.AddNode(path.ToLowerInvariant());
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
		
		// Viewer toolbar
		ImGui.SetNextItemWidth(Aeth.WidthLeft);
		var p = validPath;
		if(!p)
			ImGui.PushStyleColor(ImGuiCol.FrameBg, ImGuiColors.DPSRed);
		if(ImGui.InputText("##path", ref curPath, 128)) {
			OpenFile(curPath);
		}
		if(!p)
			ImGui.PopStyleColor();
		
		ImGui.EndTable();
	}
	
	[DllImport("aetherment_core.dll", EntryPoint = "explorer_path_valid")]
	private static extern FFI.Result IsValidPath(FFI.Str path);
}