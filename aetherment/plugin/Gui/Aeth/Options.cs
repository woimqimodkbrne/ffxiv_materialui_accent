using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using ImGuiNET;

namespace Aetherment.Gui;

public static partial class Aeth {
	public static class Option {
		public static bool RGB(string label, ref object val) {
			var v = (Vector3)val;
			var r = ImGui.ColorEdit3(label, ref v, ImGuiColorEditFlags.NoInputs);
			val = (object)v;
			return r;
		}
		
		public static bool RGBA(string label, ref object val) {
			var v = (Vector4)val;
			var r = ImGui.ColorEdit4(label, ref v, ImGuiColorEditFlags.NoInputs);
			val = (object)v;
			return r;
		}
		
		public static bool ByType(string label, string typ, ref object val) {
			switch(typ) {
				case "rgb":
					return RGB(label, ref val);
				case "rgba":
					return RGBA(label, ref val);
			}
			
			return false;
		}
	}
}