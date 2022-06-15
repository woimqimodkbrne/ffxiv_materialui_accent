using System.Collections.Generic;
using System.Threading.Tasks;
using System.Numerics;
using System;
using System.Linq;
using System.Runtime.InteropServices;
using ImGuiNET;
using Aetherment.Server;

using Main = Aetherment.Aetherment;

namespace Aetherment.Gui.Window.Aetherment;

public class ModBrowser {
	private string search = "";
	private List<short> tags = new();
	private int page = 0;
	
	string searchedQuery = "";
	List<short> searchedTags = new();
	int searchedPage = 0;
	
	private bool searching = false;
	
	private List<Mod> openMods = new();
	private List<Mod> mods = new();
	private Dictionary<string, (Aeth.Texture, DateTime)> previews = new(); // modid, (texture, lastaccess)
	private Aeth.TabBar? tabbar;
	
	public ModBrowser() {
		Search();
	}
	
	~ModBrowser() {
		
	}
	
	public void Draw() {
		tabbar = Aeth.BeginTabBar("tabs", false);
		
		if(Aeth.TabItem("Search")) {
			ImGui.BeginChild("search");
			DrawSearch();
			ImGui.EndChild();
		}
		
		foreach(var mod in openMods)
			if(Aeth.TabItem(mod.Name)) {
				ImGui.BeginChild(mod.Id);
				DrawModPage(mod);
				ImGui.EndChild();
			}
		
		Aeth.EndTabBar();
	}
	
	private void DrawModPage(Mod mod) {
		// TODO: make it a *bit* fancier
		ImGui.TextUnformatted(mod.Name);
		ImGui.TextUnformatted($"by {mod.Author.Name} and {string.Join(", ", mod.Contributors.Select((u) => u.Name).ToArray())}");
		ImGui.TextUnformatted($"dependencies: {string.Join(", ", mod.Dependencies.Select((u) => u.Name).ToArray())}");
		ImGui.TextUnformatted($"likes: {mod.Likes}, downloads: {mod.Downloads}, nsfw: {mod.Nsfw}");
		Aeth.WrappedText(mod.Description, Aeth.WidthLeft);
		if(ImGui.Button("Download"))
			Download(mod.Id, new DownloadSettings {
				ConfigPath = Main.Interface.GetPluginConfigDirectory(),
				PenumbraPath = Main.Interface.GetIpcSubscriber<string>("Penumbra.GetModDirectory").InvokeFunc()
			});
	}
	
	private void OpenModPage(Mod mod) {
		if(tabbar == null)
			return;
		
		for(int i = 0; i < openMods.Count; i++)
			if(openMods[i].Name == mod.Name) {
				tabbar.Tab = i + 1;
				return;
			}
		
		openMods.Add(mod);
		tabbar.Tab = openMods.Count;
	}
	
	private void DrawSearch() {
		if(ImGui.InputText("Search", ref search, 64)) {
			page = 0;
			Search();
		}
		
		var embedw = 500f;
		var spacer = Aeth.S.ItemSpacing.X;
		var totalw = Aeth.WidthLeft;
		var embedc = Math.Max(1, (int)Math.Floor((totalw + spacer) / (embedw + spacer)));
		embedw += ((totalw + spacer) - embedc * (embedw + spacer)) / embedc;
		var embeds = new Vector2(embedw, 100);
		
		ImGui.Text($"{embedc}, {embedw}");
		
		var i = 0;
		lock(mods)
			foreach(var mod in mods) {
				if(i % embedc != 0)
					ImGui.SameLine();
				i++;
				
				DrawModEmbed(mod, embeds);
			}
		
		ImGui.Dummy(new Vector2(0, 100));
		
		if(searching) {
			Aeth.Draw.AddText(ImGui.GetCursorScreenPos() + new Vector2(Aeth.WidthLeft / 2, -50) - ImGui.CalcTextSize("Searching...") / 2,
			                  ImGui.GetColorU32(ImGuiCol.Text),
			                  "Searching...");
		} else if(page != -1) {
			if(ImGui.GetScrollY() >= ImGui.GetScrollMaxY() - 50) {
				page += 1;
				Search();
			}
		} else {
			Aeth.Draw.AddText(ImGui.GetCursorScreenPos() + new Vector2(Aeth.WidthLeft / 2, -50) - ImGui.CalcTextSize("Thats it") / 2,
			                  ImGui.GetColorU32(ImGuiCol.Text),
			                  "Thats it");
		}
	}
	
	private void DrawModEmbed(Mod mod, Vector2 size) {
		var pos = ImGui.GetCursorScreenPos();
		ImGui.Dummy(size);
		
		if(!ImGui.IsItemVisible()) {
			if(previews.TryGetValue(mod.Id, out var val) && (DateTime.UtcNow - val.Item2).TotalMilliseconds > 10000)
				previews.Remove(mod.Id);
			
			return;
		}
		
		if(ImGui.IsItemClicked())
			OpenModPage(mod);
		
		// Embed bg
		var rounding = Aeth.S.FrameRounding;
		Aeth.Draw.AddRectFilled(pos, pos + size, ImGui.GetColorU32(ImGuiCol.FrameBg), rounding);
		
		// Preview
		var previewPos = pos + new Vector2(2, 2);
		var previewSize = new Vector2((size.Y - 4) * 1.5f, size.Y - 4);
		var tex = GetPreview(mod);
		Aeth.BoxedImage(previewPos, previewSize, tex);
		
		// Name
		Aeth.WrappedText(mod.Name,
		                 new Vector2(pos.X + previewSize.X + 8, pos.Y + 2),
		                 new Vector2(size.X - previewSize.X - 10, Aeth.TextHeight),
		                 "");
		
		// Description
		Aeth.WrappedText(mod.Description.Replace("\n", " "),
		                 new Vector2(pos.X + previewSize.X + 8, pos.Y + 6 + Aeth.TextHeight),
		                 new Vector2(size.X - previewSize.X - 10, size.Y - (Aeth.TextHeight + 6) * 2));
		
		// Tags
		Aeth.WrappedText("Tags",
		                 new Vector2(pos.X + previewSize.X + 8, pos.Y + previewSize.Y - Aeth.TextHeight - 2),
		                 new Vector2(size.X - previewSize.X - 10, Aeth.TextHeight),
		                 "",
		                 ImGui.GetColorU32(ImGuiCol.TextDisabled));
	}
	
	private Aeth.Texture GetPreview(Mod mod) {
		if(previews.TryGetValue(mod.Id, out var val)) {
			previews[mod.Id] = (val.Item1, DateTime.UtcNow);
			return val.Item1;
		}
		
		var tex = new Aeth.Texture();
		previews[mod.Id] = (tex, DateTime.UtcNow);
		
		if(mod.Previews.Length > 0) {
			var id = mod.Id;
			var preview = mod.Previews[0];
			Task.Run(() => {
				if(previews.TryGetValue(id, out var val))
					previews[id] = (Server.Server.DownloadPreview(id, preview), val.Item2);
			});
		}
		
		return tex;
	}
	
	private void Search() {
		if(searching)
			return;
		
		if(page == -1)
			return;
		
		searching = true;
		
		Task.Run(() => {
			while(search != searchedQuery || tags != searchedTags || page != searchedPage) {
				if(search != searchedQuery || tags != searchedTags) {
					lock(mods)
						mods.Clear();
					previews.Clear();
				}
				
				searchedQuery = search;
				searchedTags = tags; // probably doesnt copy, TODO: check that
				searchedPage = page;
				
				PluginLog.Log($"search {searchedQuery}");
				
				// ofc c# refers to the namespace before the class inside the namespace we are importing
				var m = Server.Server.Search(searchedQuery, searchedTags.ToArray(), searchedPage);
				if(m.Length == 0) {
					page = -1;
					break;
				}
				
				lock(mods)
					mods.AddRange(m);
			}
			
			searching = false;
		});
	}
	
	private struct DownloadSettings {
		public FFI.Str ConfigPath;
		public FFI.Str PenumbraPath;
	}
	
	[DllImport("aetherment_core.dll", EntryPoint = "download_mod")]
	private static extern void Download(FFI.Str id, DownloadSettings settings);
}