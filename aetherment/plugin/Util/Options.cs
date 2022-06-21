using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;

namespace Aetherment.Util;

public static class Settings {
	public static string RGB(object val) {
		var v = (Vector3)val;
		return $"[{v.X}, {v.Y}, {v.Z}]";
	}
	
	public static string RGBA(object val) {
		var v = (Vector4)val;
		return $"[{v.X}, {v.Y}, {v.Z}, {v.W}]";
	}
	
	public static string ByType(string typ, object val) {
		switch(typ) {
			case "rgb":
				return RGB(val);
			case "rgba":
				return RGBA(val);
		}
		
		return "";
	}
}