using System.Numerics;
using Dalamud.Interface;
using ImGuiNET;

namespace Aetherment.Gui;

public static partial class Aeth {
	public static ImGuiNET.ImGuiStylePtr S => ImGui.GetStyle();
	public static float FrameHeight => S.FramePadding.Y * 2 + ImGui.GetFontSize();
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
}