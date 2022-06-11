using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Numerics;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Dalamud.Interface;
using Dalamud.Interface.Colors;
using ImGuiNET;
using Newtonsoft.Json;
using Main = Aetherment.Aetherment;

namespace Aetherment.Gui.Window.Aetherment;

public class ModDev {
	private struct Meta {
		public string name;
		public string description;
		public int[] contributors;
		public int[] dependencies;
		public int? main_mod;
		[JsonIgnore] public string main_mod_label;
		public bool nsfw;
		[JsonIgnore] public (string, int)[] list;
		[JsonIgnore] public List<Aeth.Texture> previews;
		
		public Meta() {
			name = "";
			description = "";
			contributors = new int[0];
			dependencies = new int[0];
			main_mod = null;
			main_mod_label = "";
			nsfw = false;
			list = new (string, int)[0];
			previews = new();
		}
	}
	
	private List<string> mods;
	private string selectedMod = "";
	private string lastpath = "";
	private Meta curMod;
	
	private string importpath = "";
	
	public ModDev() {
		mods = new();
		
		// TODO: use filesystemwatcher to auto reload
		// it wont trigger if a file is renamed to be in a different folder (aka `delete`)
		
		ReloadList();
	}
	
	~ModDev() {
		
	}
	
	private void ReloadList() {
		PluginLog.Log("reload");
		mods.Clear();
		
		foreach(var d in new DirectoryInfo(Main.Config.LocalPath).EnumerateDirectories())
			mods.Add(d.Name);
		
		if(!mods.Contains(selectedMod))
			selectedMod = "";
	}
	
	private void ReloadselectedMod() {
		var path = $"{new DirectoryInfo(Main.Config.LocalPath).FullName}/{selectedMod}";
		var pathmeta = $"{path}/meta.json";
		curMod = File.Exists(pathmeta) ? JsonConvert.DeserializeObject<Meta>(File.ReadAllText(pathmeta)) : new();
		curMod.main_mod_label = "";
		curMod.list = new (string, int)[0];
		curMod.previews = new();
		
		var s = selectedMod;
		Task.Run(() => {
			if(s != selectedMod)
				return;
			
			if(curMod.main_mod != null)
				if(Server.Server.ModPage(curMod.main_mod.Value) is Server.Mod m)
					curMod.main_mod_label = m.Name;
			
			try { // who needs checks anyways
				foreach(var p in new DirectoryInfo($"{path}/previews").EnumerateFiles())
					lock(curMod.previews)
						curMod.previews.Add(FFI.Extern.ReadImage(p.FullName));
			} catch {}
		});
	}
	
	public void Draw() {
		if(lastpath != Main.Config.LocalPath) {
			lastpath = Main.Config.LocalPath;
			ReloadList();
		}
		
		// ImGui.PushStyleVar(ImGuiStyleVar.CellPadding, new Vector2(ImGui.GetStyle().CellPadding.X, 0));
		ImGui.BeginTable("##divider", 2, ImGuiTableFlags.Resizable);
		ImGui.TableSetupColumn("##list", ImGuiTableColumnFlags.WidthFixed, 200);
		ImGui.TableSetupColumn("##editor", ImGuiTableColumnFlags.WidthStretch);
		ImGui.TableNextRow();
		
		// List
		ImGui.TableNextColumn();
		// ImGui.BeginChildFrame(ImGui.GetID("list"), new Vector2(0, -Aeth.FrameHeight - Aeth.S.ItemSpacing.Y));
		ImGui.BeginChild("list", new Vector2(0, -Aeth.FrameHeight - Aeth.S.ItemSpacing.Y));
		foreach(var mod in mods)
			if(ImGui.Selectable(mod, selectedMod == mod)) {
				selectedMod = mod;
				ReloadselectedMod();
			}
		// ImGui.EndChildFrame();
		ImGui.EndChild();
		
		// Buttons
		var o = (Aeth.WidthLeft + Aeth.FrameHeight) / 4;
		var pos = ImGui.GetCursorPos() - new Vector2(Aeth.FrameHeight, 0);
		
		ImGui.SetCursorPos(new Vector2(pos.X + o, pos.Y));
		Aeth.ButtonIcon(FontAwesomeIcon.Plus);
		Aeth.HoverTooltip("New Mod");
		
		ImGui.SetCursorPos(new Vector2(pos.X + o * 2, pos.Y));
		if(Aeth.ButtonIcon(FontAwesomeIcon.FileImport))
			ImGui.OpenPopup("import");
		Aeth.HoverTooltip("Import");
		
		// TODO: have fancy window showing all penumbra mods, too lazy atm
		if(ImGui.BeginPopupContextItem("import")) {
			ImGui.InputTextWithHint("##input", "Penumbra Mod Path", ref importpath, 128);
			if(ImGui.Button("Import")) {
				PluginLog.Log("import");
				var origin = importpath;
				var target = $"{new DirectoryInfo(Main.Config.LocalPath).FullName}/{new DirectoryInfo(importpath).Name}";
				importpath = "";
				Task.Run(() => {
					ImportPenumbra(origin, target);
					ReloadList();
				});
				ImGui.CloseCurrentPopup();
			}
			ImGui.EndPopup();
		}
		
		ImGui.SetCursorPos(new Vector2(pos.X + o * 3, pos.Y));
		if(Aeth.ButtonIcon("")) // why isnt this in the enum? (fa-arrows-rotate)
			ReloadList();
		Aeth.HoverTooltip("Refresh list");
		
		// Editor
		ImGui.TableNextColumn();
		if(selectedMod != "") {
			ImGui.BeginChild("selected", new Vector2(0, -Aeth.FrameHeight - Aeth.S.ItemSpacing.Y));
			if(ImGui.CollapsingHeader("Meta")) {
				var w = Aeth.WidthLeft - 100;
				ImGui.SetNextItemWidth(w);
				ImGui.InputText("Name", ref curMod.name, 256);
				var limit = $"{curMod.name.Length}/64";
				Aeth.Offset(w - ImGui.CalcTextSize(limit).X - 4, -Aeth.S.ItemSpacing.Y, false);
				ImGui.TextColored(curMod.name.Length > 64 ? ImGuiColors.DPSRed : Aeth.S.Colors[(int)ImGuiCol.Text], limit);
				
				ImGui.InputTextMultiline("Description", ref curMod.description, 40000, new Vector2(w, 200));
				limit = $"{curMod.description.Length}/10000";
				Aeth.Offset(w - ImGui.CalcTextSize(limit).X - 4, -Aeth.S.ItemSpacing.Y, false);
				ImGui.TextColored(curMod.description.Length > 10000 ? ImGuiColors.DPSRed : Aeth.S.Colors[(int)ImGuiCol.Text], limit);
				
				ImGui.Text("Previews");
				ImGui.BeginChild("previews", new Vector2(0, 150 + Aeth.S.ScrollbarSize), false, ImGuiWindowFlags.AlwaysHorizontalScrollbar);
				// previews
				var ps = new Vector2(225, 150);
				foreach(var img in curMod.previews) {
					Aeth.BoxedImage(ps, img);
					ImGui.SameLine();
				}
				
				// new preview
				pos = ImGui.GetCursorScreenPos();
				ImGui.Dummy(ps);
				Aeth.Draw.AddRectFilled(pos, pos + ps, 0xFF101010, Aeth.S.FrameRounding);
				pos += ps / 2;
				Aeth.Draw.AddRectFilled(pos + new Vector2(-5, -40),
				                        pos + new Vector2( 5,  40),
				                        0xFFFFFFFF, Aeth.S.FrameRounding);
				Aeth.Draw.AddRectFilled(pos + new Vector2(-40, -5),
				                        pos + new Vector2( 40,  5),
				                        0xFFFFFFFF, Aeth.S.FrameRounding);
				// TODO: dalamud file selector
				// TODO: better name, this wont take into account non last preview deletion
				// TODO: name also always assumes there is a extension, this might not be the case
				if(ImGui.IsItemClicked())
					ImGui.OpenPopup("previewadd");
				Aeth.HoverTooltip("Add Preview");
				if(ImGui.BeginPopupContextItem("previewadd")) {
					ImGui.InputTextWithHint("##input", "Image path", ref importpath, 128);
					if(ImGui.Button("Add")) {
						var path = importpath;
						var previewspath = $"{new DirectoryInfo(Main.Config.LocalPath).FullName}/{selectedMod}/previews";
						importpath = "";
						Task.Run(() => {
							var dir = Directory.CreateDirectory(previewspath);
							var destpath = $"{previewspath}/{dir.GetFiles().Length + 1}.{path.Split(".").Last()}";
							File.Copy(path, destpath);
							lock(curMod.previews)
								curMod.previews.Add(FFI.Extern.ReadImage(destpath));
						});
						ImGui.CloseCurrentPopup();
					}
					ImGui.EndPopup();
				}
				ImGui.EndChild();
				
				ImGui.Text("TODO: contributor selection");
				
				ImGui.Text("TODO: dependency selection");
				
				// Main mod
				ImGui.SetNextItemWidth(300);
				if(ImGui.InputText("Main Mod", ref curMod.main_mod_label, 256)) {
					// TODO: dont search while already doing so, aka copy what we do in modbrowser
					curMod.main_mod = null;
					Task.Run(() => {curMod.list = Server.Server.Search(curMod.main_mod_label, new short[0], 0).Select((e) => (e.Name, e.Id)).ToArray();});
				}
				var active = ImGui.IsItemActive();
				if(active)
					ImGui.OpenPopup("mainmod");
				
				ImGui.SetNextWindowPos(new Vector2(ImGui.GetItemRectMin().X, ImGui.GetItemRectMax().Y));
				ImGui.SetNextWindowSize(new Vector2(300, 200));
				if(ImGui.BeginPopup("mainmod", ImGuiWindowFlags.NoFocusOnAppearing | ImGuiWindowFlags.ChildWindow)) {
					foreach(var s in curMod.list)
						if(ImGui.Selectable(s.Item1)) {
							curMod.main_mod = s.Item2;
							curMod.main_mod_label = s.Item1;
						}
					
					active |= ImGui.IsWindowFocused();
					
					if(!active)
						ImGui.CloseCurrentPopup();
					
					ImGui.EndPopup();
				}
				
				// nsfw
				ImGui.Checkbox("NSFW", ref curMod.nsfw);
			}
			ImGui.EndChild();
			
			if(Aeth.ButtonIcon("")) // fa-floppy-disk
				File.WriteAllText($"{new DirectoryInfo(Main.Config.LocalPath).FullName}/{selectedMod}/meta.json", JsonConvert.SerializeObject(curMod));
			Aeth.HoverTooltip("Save Changes");
			
			ImGui.SameLine();
			Aeth.ButtonIcon(""); // fa-cloud-upload-alt
			Aeth.HoverTooltip("Upload");
		}
		
		ImGui.EndTable();
	}
	
	[DllImport("aetherment_core.dll", EntryPoint = "import_penumbra")]
	public static extern void ImportPenumbra(FFI.Str penumbra_path, FFI.Str target_path);
}