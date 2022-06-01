using System.Collections.Generic;
using System.Threading.Tasks;
using System.Numerics;

using ImGuiNET;
using Main = Aetherment.Aetherment;
using Aetherment.Server;

namespace Aetherment.Gui.Window.Aetherment;

public class ModBrowser {
	private string search = "";
	private List<short> tags = new();
	private byte page = 0;
	
	private Mod[] mods = new Mod[0];
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
		
		if(ImGui.Button("<") && page > 0) {
			page -= 1;
			Search();
		}
		ImGui.SameLine();
		ImGui.Text($"{page}");
		ImGui.SameLine();
		if(ImGui.Button(">")) {
			page += 1;
			Search();
		}
		
		foreach(var mod in mods)
			ImGui.Text($"{mod.Name} - {mod.Author.Name} - {mod.Description}");
	}
	
	private void Search() {
		if(searching)
			return;
		
		searching = true;
		
		Task.Run(async() => {
			string searchedQuery = "";
			List<short> searchedTags = new();
			byte searchedPage = 0;
			
			while(search != searchedQuery || tags != searchedTags || page != searchedPage) {
				searchedQuery = search;
				searchedTags = tags; // probably doesnt copy, TODO: check that
				searchedPage = page;
				
				PluginLog.Log($"search {searchedQuery}");
				
				mods = await Main.Server.Search(searchedQuery, searchedTags.ToArray(), searchedPage);
			}
			
			searching = false;
		});
	}
}