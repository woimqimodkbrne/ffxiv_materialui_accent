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
using System.Collections.Generic;

namespace Aetherment.Gui.Window.Aetherment.Explorer;

public class Explorer {
	private Tree gameTree;
	private bool populatedGameTree = false;
	private bool populatingGameTree = false;
	
	private Tree modTree = null!;
	private IntPtr modDatas = IntPtr.Zero;
	// private Dictionary<string, string[][]> modPaths = null!;
	// private Dictionary<string, List<string>> modOptions = null!;
	
	private string selectedMod = null!;
	private string selectedOption = null!;
	private string selectedSubOption = null!;
	private string[] modSelectorEntries = null!;
	private bool populatedModSelector = false;
	
	private Viewer.Viewer? viewer;
	
	private bool validPath = false;
	private string curPath = "";
	
	private FileDialog? dialog;
	private Action<bool, string> dialogCallback = null!;
	private string dialogFilters = null!;
	
	public Explorer() {
		gameTree = new("Game Files", (string path) => {OpenFile(path);});
	}
	
	private void LoadMod(string mod) {
		if(modDatas != IntPtr.Zero)
			FFI.Extern.FreeObject(modDatas);
		
		var path = $"{Main.Config.LocalPath}/{mod}";
		selectedMod = mod;
		modDatas = GetModDatas(path).Unwrap<IntPtr>();
		modTree = new(mod, (string path) => {OpenFileMod(path);});
		modTree.AddNode("Options");
		
		foreach(var p in (string[])GetDatasGamepaths(modDatas).Unwrap<FFI.Vec>())
			modTree.AddNode(p);
		
		
		SetModOption("", "");
	}
	
	private void SetModOption(string option, string suboption) {
		selectedOption = option;
		selectedSubOption = suboption;
		modTree.SetNodeState(false);
		modTree.SetNodeState("Options", true); // never disable basic stuff
		foreach(var p in (string[])GetDatasGamepaths(modDatas, option, suboption).Unwrap<FFI.Vec>()) {
			modTree.SetNodeState(p, true);
		}
		
		// Reload the viewer
		if(!OpenFileMod(curPath))
			OpenFile(curPath);
	}
	
	private bool OpenFileMod(string path) {
		curPath = path;
		if(path == "Options")
			viewer = new Viewer.Options($"{Main.Config.LocalPath}/{selectedMod}/datas.json");
		
		validPath = false;
		if(!GetDatasFilePaths(modDatas, curPath, selectedOption, selectedSubOption).IsOk(out FFI.Vec[] p))
			if(!GetDatasFilePaths(modDatas, curPath, "", "").IsOk(out p))
				return false;
		
		validPath = true;
		
		var paths = new string[p.Length][];
		for(var i = 0; i < p.Length; i++)
			paths[i] = (string[])p[i];
		
		var simplePath = $"{Main.Config.LocalPath}/{selectedMod}/{(paths.Length == 1 && paths[0].Length == 1 ? paths[0][0] : paths[0][1])}";
		// TODO: tex viewer multi layer support
		OpenViewer(simplePath, curPath.Split(".").Last());
		
		return true;
	}
	
	private bool OpenFile(string path) {
		curPath = path.ToLowerInvariant();
		validPath = IsValidPath(curPath).Unwrap<byte>() != 0;
		if(!validPath)
			return false;
		
		OpenViewer(curPath, curPath.Split(".").Last());
		
		gameTree.SelectedPath = path;
		
		return true;
	}
	
	private void OpenViewer(string path, string ext) {
		if(ext == "tex")
			viewer = new Viewer.Tex(path);
		else if(ext == "mtrl")
			viewer = new Viewer.Mtrl(path);
		else
			viewer = null;
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
		ImGui.BeginChild("tree", new Vector2(0, -(Aeth.FrameHeight + Aeth.S.ItemSpacing.Y) * 2));
		if(modTree != null)
			modTree.Draw();
		gameTree.Draw();
		if(populatingGameTree)
			ImGui.Text("Populating Tree..."); // TODO: fancy loading icon
		ImGui.EndChild();
		
		// Option Selector
		if(selectedMod != null) {
			ImGui.SetNextItemWidth(Aeth.WidthLeft);
			if(ImGui.BeginCombo("##optionselector", $"{selectedOption}/{selectedSubOption}", ImGuiComboFlags.HeightRegular)) {
				foreach(var o in (string[])GetDatasOptions(modDatas).Unwrap<FFI.Vec>())
					if(ImGui.TreeNode(o)) {
						foreach(var s in (string[])GetDatasOptions(modDatas, o).Unwrap<FFI.Vec>())
							if(ImGui.Selectable(s, o == selectedOption && s == selectedSubOption))
								SetModOption(o, s);
						ImGui.TreePop();
					}
				
				ImGui.EndCombo();
			}
		} else {
			ImGui.Dummy(new Vector2(Aeth.WidthLeft, Aeth.FrameHeight));
		}
		
		// Mod Selector
		ImGui.SetNextItemWidth(Aeth.WidthLeft);
		if(ImGui.BeginCombo("##modselector", selectedMod, ImGuiComboFlags.HeightRegular)) {
			if(!populatedModSelector) {
				if(Main.Config.LocalPath != "")
					modSelectorEntries = new DirectoryInfo(Main.Config.LocalPath).GetDirectories().Select(m => m.Name).ToArray();
				populatedModSelector = true;
			}
			
			foreach(var mod in modSelectorEntries)
				if(ImGui.Selectable(mod, mod == selectedMod))
					LoadMod(mod);
			
			ImGui.EndCombo();
		} else {
			populatedModSelector = false;
		}
		
		// Viewer
		ImGui.TableNextColumn();
		ImGui.BeginChild("viewer", new Vector2(0, -Aeth.FrameHeight - Aeth.S.ItemSpacing.Y));
		if(viewer != null)
			viewer.Draw();
		ImGui.EndChild();
		
		// Viewer toolbar
		if(viewer != null && curPath != "Options") {
			if(ImGui.Button("Import (TODO)", new Vector2(100, Aeth.FrameHeight)))
				{}
			
			ImGui.SameLine();
			if(ImGui.Button("Export", new Vector2(100, Aeth.FrameHeight))) {
				var name = curPath.Split("/").Last().Split(".")[0];
				OpenDialog("SaveFileDialog", "Export " + name, string.Join(",", viewer.validExports), name, Export);
			}
			
			ImGui.SameLine();
		}
		
		var p = validPath;
		if(!p)
			ImGui.PushStyleColor(ImGuiCol.FrameBg, ImGuiColors.DPSRed);
		ImGui.SetNextItemWidth(Aeth.WidthLeft);
		if(ImGui.InputText("##path", ref curPath, 256))
			if(!OpenFileMod(curPath))
				OpenFile(curPath);
		if(!p)
			ImGui.PopStyleColor();
		
		ImGui.EndTable();
		
		// Open/Save dialog
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
	[DllImport("aetherment_core.dll", EntryPoint = "explorer_datas_load")]
	private static extern FFI.Result GetModDatas(FFI.Str path);
	[DllImport("aetherment_core.dll", EntryPoint = "explorer_datas_gamepaths")]
	private static extern FFI.Result GetDatasGamepaths(IntPtr datas);
	[DllImport("aetherment_core.dll", EntryPoint = "explorer_datas_option_gamepaths")]
	private static extern FFI.Result GetDatasGamepaths(IntPtr datas, FFI.Str option, FFI.Str suboption);
	[DllImport("aetherment_core.dll", EntryPoint = "explorer_datas_paths")]
	private static extern FFI.Result GetDatasFilePaths(IntPtr datas, FFI.Str gamepath, FFI.Str option, FFI.Str suboption);
	[DllImport("aetherment_core.dll", EntryPoint = "explorer_datas_options")]
	private static extern FFI.Result GetDatasOptions(IntPtr datas);
	[DllImport("aetherment_core.dll", EntryPoint = "explorer_datas_suboptions")]
	private static extern FFI.Result GetDatasOptions(IntPtr datas, FFI.Str option);
}