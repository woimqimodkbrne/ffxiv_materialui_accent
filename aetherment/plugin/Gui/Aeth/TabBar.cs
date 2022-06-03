using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using ImGuiNET;

namespace Aetherment.Gui;

// vanilla tabbar ugly af
// it doesnt remember what tab you had like vanilla does
// but its a price im willing to pay

public static partial class Aeth {
	public class TabBar {
		public int Tab;
		public List<string> Tabs;
		
		public TabBar() {
			Tab = 0;
			Tabs = new();
		}
	}
	
	private static Dictionary<uint, TabBar> tabbars = new();
	private static List<TabBar> tabbarstack = new();
	
	public static TabBar BeginTabBar(string sid, bool dockedBottom = true) {
		var id = ImGui.GetID(sid);
		var bar = tabbars.ContainsKey(id) ? tabbars[id] : new TabBar();
		tabbars[id] = bar;
		tabbarstack.Add(bar);
		
		var org = ImGui.GetCursorPos();
		var pos = ImGui.GetCursorScreenPos();
		var barw = ImGui.GetColumnWidth();
		var tabw = barw / Math.Max(1, bar.Tabs.Count);
		var tabh = Aeth.FrameHeight;
		var l = bar.Tabs.Count - 1;
		var hover = -1;
		
		for(int i = 0; i < bar.Tabs.Count; i++) {
			ImGui.SetCursorPos(new Vector2(org.X + i * tabw, org.Y));
			ImGui.Dummy(new Vector2(tabw, tabh));
			if(ImGui.IsItemHovered())
				hover = i;
			if(ImGui.IsItemClicked())
				bar.Tab = i;
		}
		
		var clrt = ImGui.GetColorU32(ImGuiCol.Tab);
		var clrh = ImGui.GetColorU32(ImGuiCol.TabHovered);
		var clra = ImGui.GetColorU32(ImGuiCol.TabActive);
		var draw = ImGui.GetWindowDrawList();
		for(int i = 0; i < bar.Tabs.Count; i++) {
			draw.AddRectFilled(
				new Vector2(pos.X + i * tabw        + (i != 0 ? 1 : 0), pos.Y),
				new Vector2(pos.X + i * tabw + tabw - (i != l ? 1 : 0), pos.Y + tabh),
				i == hover ? clrh : i == bar.Tab ? clra : clrt,
				Aeth.S.TabRounding,
				i == 0 ? (dockedBottom ? ImDrawFlags.RoundCornersTopLeft  : ImDrawFlags.RoundCornersBottomLeft ) :
				i == l ? (dockedBottom ? ImDrawFlags.RoundCornersTopRight : ImDrawFlags.RoundCornersBottomRight) :
				          ImDrawFlags.RoundCornersNone
			);
			
			draw.AddText(
				new Vector2(pos.X + i * tabw, pos.Y) + Aeth.S.FramePadding,
				ImGui.GetColorU32(ImGuiCol.Text),
				bar.Tabs[i]
			);
		}
		
		var y = (dockedBottom ? tabh - 1 : 0);
		draw.AddLine(
			new Vector2(pos.X, pos.Y + y),
			new Vector2(pos.X + barw, pos.Y + y),
			clra
		);
		
		bar.Tabs.Clear();
		
		return bar;
	}
	
	public static void EndTabBar() {
		tabbarstack.RemoveAt(tabbarstack.Count - 1);
	}
	
	public static bool TabItem(string label) {
		var bar = tabbarstack.Last();
		bar.Tabs.Add(label);
		
		return bar.Tab == bar.Tabs.Count - 1;
	}
}