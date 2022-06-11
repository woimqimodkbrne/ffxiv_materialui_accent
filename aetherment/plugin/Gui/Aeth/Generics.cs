using System;
using System.Collections.Generic;
using System.Numerics;
using System.Text.RegularExpressions;
using Dalamud.Interface;
using ImGuiNET;

namespace Aetherment.Gui;

public static partial class Aeth {
	public static ImDrawListPtr Draw => ImGui.GetWindowDrawList();
	public static ImGuiStylePtr S => ImGui.GetStyle();
	public static float TextHeight => ImGui.GetFontSize();
	public static float FrameHeight => S.FramePadding.Y * 2 + TextHeight;
	public static float WidthLeft => ImGui.GetContentRegionAvail().X;
	public static float HeightLeft => ImGui.GetContentRegionAvail().Y;
	
	public static void Offset(Vector2 xy, bool globalScale = true) {
		ImGui.SetCursorPos(ImGui.GetCursorPos() + xy * (globalScale ? ImGuiHelpers.GlobalScale : 1));
	}
	
	public static void Offset(float x, float y, bool globalScale = true) {
		if(globalScale) {
			ImGui.SetCursorPosX(ImGui.GetCursorPosX() + x * ImGuiHelpers.GlobalScale);
			ImGui.SetCursorPosY(ImGui.GetCursorPosY() + y * ImGuiHelpers.GlobalScale);
		} else {
			ImGui.SetCursorPosX(ImGui.GetCursorPosX() + x);
			ImGui.SetCursorPosY(ImGui.GetCursorPosY() + y);
		}
	}
	
	// TODO: mby rewrite it to constantly half until it reaches the perfect size, idk the fancy name for it
	// TODO: regex based splitting and cutting away useless chars like trailing commas
	public static void WrappedText(string text, Vector2 pos, Vector2 size, string split = " ", uint? clr = null) {
		if(clr == null)
			clr = ImGui.GetColorU32(ImGuiCol.Text);
		
		var lines = Math.Max(1, (int)Math.Floor(size.Y / TextHeight));
		// var content = text.Split(split); // lmao, a empty string doesnt split on every char, dumb shit mfg
		var content = Regex.Split(text, split);
		var line = 0;
		var curline = content[0];
		
		for(var i = 1; i < content.Length; i++) {
			var seg = content[i];
			var add = line == lines - 1 ? "..." : "";
			
			if(ImGui.CalcTextSize(curline + split + seg + add).X > size.X) {
				Draw.AddText(new Vector2(pos.X, pos.Y + line * TextHeight), clr.Value, curline + add);
				line++;
				curline = seg;
				if(line == lines)
					return;
			} else
				curline += split + seg;
		}
		
		Draw.AddText(new Vector2(pos.X, pos.Y + line * TextHeight), clr.Value, curline);
	}
	
	public static void HoverTooltip(string label) {
		if(ImGui.IsItemHovered()) {
			ImGui.BeginTooltip();
			ImGui.TextUnformatted(label);
			ImGui.EndTooltip();
		}
			
	}
	
	public static bool ButtonIcon(FontAwesomeIcon icon) => ButtonIcon(icon.ToIconString());
	
	public static bool ButtonIcon(string icon) {
		ImGui.PushFont(UiBuilder.IconFont);
		
		var size = new Vector2(FrameHeight);
		var pos = ImGui.GetCursorPos();
		var hover = false;
		ImGui.PushStyleVar(ImGuiStyleVar.FramePadding, new Vector2(0, S.FramePadding.Y));
		ImGui.Dummy(size);
		if(ImGui.IsItemHovered()) {
			ImGui.PushStyleColor(ImGuiCol.Text, S.Colors[1]);
			hover = true;
		}
		
		ImGui.SetCursorPos(pos);
		var a = ImGui.Button(icon, size);
		ImGui.PopStyleVar();
		
		if(hover)
			ImGui.PopStyleColor();
		ImGui.PopFont();
		
		return a;
	}
	
	public static void BoxedImage(Vector2 size, Texture tex) {
		BoxedImage(ImGui.GetCursorScreenPos(), size, tex);
		ImGui.Dummy(size);
	}
	
	public static void BoxedImage(Vector2 pos, Vector2 size, Texture tex) {
		var rounding = S.FrameRounding;
		Aeth.Draw.AddRectFilled(pos, pos + size, 0xFF101010, rounding);
		var scale = Math.Min(size.X / tex.Width, size.Y / tex.Height);
		var w = tex.Width * scale;
		var h = tex.Height * scale;
		pos.X += (size.X - w) / 2;
		pos.Y += (size.Y - h) / 2;
		rounding -= Math.Min(rounding, Math.Max(size.X - w, size.Y - h) / 2);
		Draw.AddImageRounded(tex, pos, pos + new Vector2(w, h), Vector2.Zero, Vector2.One, 0xFFFFFFFF, rounding);
	}
}