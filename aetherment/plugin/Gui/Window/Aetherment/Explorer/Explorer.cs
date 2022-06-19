using System.Numerics;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using ImGuiNET;
using Main = Aetherment.Aetherment;
using System.Runtime.InteropServices;
using Dalamud.Interface.Colors;
using System;
using Dalamud.Interface.ImGuiFileDialog;

namespace Aetherment.Gui.Window.Aetherment.Explorer;

public class Explorer {
	private Tree gameTree;
	private bool populatedGameTree = false;
	private bool populatingGameTree = false;
	
	private Viewer.Viewer? viewer;
	
	private bool validPath = false;
	private string curPath = "";
	
	private FileDialog? dialog;
	private Action<bool, string> dialogCallback = null!;
	private string dialogFilters = null!;
	
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
	
	private void OpenDialog(string id, string title, string filters, string name, Action<bool, string> callback) {
		var ext = string.Empty;
		if(id == "SaveFileDialog") {
			ext = Main.Config.ExplorerExportExt.ContainsKey(filters) ? Main.Config.ExplorerExportExt[filters] : filters.Split(",")[0];
			dialogFilters = filters;
		} else
			dialogFilters = null!;
		
		dialog = new FileDialog(id, title, filters, Main.Config.ExplorerExportPath, name, ext, 1, false, ImGuiFileDialogFlags.None);
		dialog.Show();
		dialogCallback = callback;
	}
	
	private void Export(bool success, string path) {
		if(!success)
			return;
		
		viewer!.SaveFile(path, path.Split(".").Last());
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
		if(viewer != null)
			viewer.Draw();
		ImGui.EndChild();
		
		// Viewer toolbar
		if(viewer != null) {
			if(ImGui.Button("Import (TODO)", new Vector2(100, Aeth.FrameHeight)))
				{}
			
			ImGui.SameLine();
			if(ImGui.Button("Export", new Vector2(100, Aeth.FrameHeight))) {
				var name = curPath.Split("/").Last().Split(".")[0];
				OpenDialog("SaveFileDialog", "Export " + name, string.Join(",", viewer.validExports), name, Export);
			}
		}
		
		var p = validPath;
		if(!p)
			ImGui.PushStyleColor(ImGuiCol.FrameBg, ImGuiColors.DPSRed);
		ImGui.SameLine();
		ImGui.SetNextItemWidth(Aeth.WidthLeft);
		if(ImGui.InputText("##path", ref curPath, 128)) {
			OpenFile(curPath);
		}
		if(!p)
			ImGui.PopStyleColor();
		
		ImGui.EndTable();
		
		
		if(dialog != null && dialog.Draw()) {
			var result = dialog.GetResults()[0];
			dialogCallback(dialog.GetIsOk(), result);
			Main.Config.MarkForChanges();
			Main.Config.ExplorerExportPath = dialog.GetCurrentPath();
			if(dialogFilters != null)
				Main.Config.ExplorerExportExt[dialogFilters] = "." + result.Split(".").Last();
			Main.Config.Save();
			dialog = null;
		}
	}
	
	[DllImport("aetherment_core.dll", EntryPoint = "explorer_path_valid")]
	private static extern FFI.Result IsValidPath(FFI.Str path);
}