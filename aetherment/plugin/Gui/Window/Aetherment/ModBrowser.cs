using System.Collections.Generic;
using System.Threading.Tasks;
using System.Numerics;

using ImGuiNET;
using Aetherment.Server;
using System;

namespace Aetherment.Gui.Window.Aetherment;

public class ModBrowser {
	private string search = "";
	private List<short> tags = new();
	private byte page = 0;
	
	private List<Mod> mods = new();
	private Dictionary<int, (Aeth.Texture, DateTime)> previews = new(); // modid, (texture, lastaccess)
	private bool searching = false;
	
	public ModBrowser() {
		Search();
	}
	
	~ModBrowser() {
		
	}
	
	public void Draw() {
		Aeth.BeginTabBar("tabs", false);
		
		if(Aeth.TabItem("Search")) {
			ImGui.BeginChild("search");
			DrawSearch();
			ImGui.EndChild();
			ImGui.EndTabItem();
		}
		
		Aeth.TabItem("Blah1");
		Aeth.TabItem("Blah2");
		Aeth.TabItem("Blah3");
		Aeth.TabItem("Blah4");
		
		Aeth.EndTabBar();
	}
	
	private void DrawSearch() {
		if(ImGui.InputText("Search", ref search, 64))
			Search();
		
		var embedw = 500f;
		var spacer = Aeth.S.ItemSpacing.X;
		var totalw = Aeth.WidthLeft;
		var embedc = Math.Max(1, (int)Math.Floor((totalw + spacer) / (embedw + spacer)));
		embedw += ((totalw + spacer) - embedc * (embedw + spacer)) / embedc;
		var embeds = new Vector2(embedw, 100);
		
		ImGui.Text($"{embedc}, {embedw}");
		
		var i = 0;
		foreach(var mod in mods) {
			if(i % embedc != 0)
				ImGui.SameLine();
			i++;
			
			DrawModEmbed(mod, embeds);
		}
		
		// if(ImGui.GetScrollY())
	}
	
	private void DrawModEmbed(Mod mod, Vector2 size) {
		var pos = ImGui.GetCursorScreenPos();
		ImGui.Dummy(size);
		
		if(!ImGui.IsItemVisible()) {
			if(previews.TryGetValue(mod.Id, out var val) && (DateTime.UtcNow - val.Item2).TotalMilliseconds > 10000)
				previews.Remove(mod.Id);
			
			return;
		}
		
		// Embed bg
		var rounding = Aeth.S.FrameRounding;
		Aeth.Draw.AddRectFilled(pos, pos + size, ImGui.GetColorU32(ImGuiCol.FrameBg), rounding);
		
		// Preview bg
		var previewPos = pos + new Vector2(2, 2);
		var previewSize = new Vector2((size.Y - 4) * 1.5f, size.Y - 4);
		Aeth.Draw.AddRectFilled(previewPos, previewPos + previewSize, 0xFF101010, rounding);
		
		// Preview
		var tex = GetPreview(mod);
		var scale = Math.Min(previewSize.X / tex.Width, previewSize.Y / tex.Height);
		var w = tex.Width * scale;
		var h = tex.Height * scale;
		previewPos.X += (previewSize.X - w) / 2;
		previewPos.Y += (previewSize.Y - h) / 2;
		rounding -= Math.Min(rounding, Math.Max(previewSize.X - w, previewSize.Y - h) / 2);
		Aeth.Draw.AddImageRounded(tex, previewPos, previewPos + new Vector2(w, h), Vector2.Zero, Vector2.One, 0xFFFFFFFF, rounding);
		
		// Name
		Aeth.WrappedText(mod.Name,
		                 new Vector2(pos.X + previewSize.X + 8, pos.Y + 2),
		                 new Vector2(size.X - previewSize.X - 10, Aeth.TextHeight),
		                 "");
		
		// Description
		Aeth.WrappedText(mod.Description,
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
		
		searching = true;
		
		Task.Run(() => {
			string searchedQuery = "";
			List<short> searchedTags = new();
			byte searchedPage = 0;
			
			while(search != searchedQuery || tags != searchedTags || page != searchedPage) {
				searchedQuery = search;
				searchedTags = tags; // probably doesnt copy, TODO: check that
				searchedPage = page;
				
				PluginLog.Log($"search {searchedQuery}");
				
				if(search != searchedQuery || tags != searchedTags) {
					mods.Clear();
					previews.Clear();
				}
				
				// ofc c# refers to the namespace before the class inside the namespace we are importing
				mods.AddRange(Server.Server.Search(searchedQuery, searchedTags.ToArray(), searchedPage));
			}
			
			searching = false;
		});
	}
}