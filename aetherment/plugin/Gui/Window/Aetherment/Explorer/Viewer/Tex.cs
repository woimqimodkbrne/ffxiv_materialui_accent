using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Runtime.InteropServices;
using ImGuiNET;

namespace Aetherment.Gui.Window.Aetherment.Explorer.Viewer;

public class Tex: Viewer {
	private enum Format: uint {
		Unknown = 0x0,
		
		L8 = 0x1130,
		A8 = 0x1131,
		
		Argb4 = 0x1440,
		A1rgb5 = 0x1441,
		Argb8 = 0x1450,
		Xrgb8 = 0x1451,
		Argb82 = 0x1452,
		
		R32F = 0x2150,
		Rg16F = 0x2250,
		Argb16 = 0x2460,
		Rgba32F = 0x2470,
		
		Dxt1 = 0x3420,
		Dxt3 = 0x3430,
		Dxt5 = 0x3431,
		
		D16 = 0x4140,
		D24S8 = 0x4250,
		Rgba8 = 0x4401,
		
		Null = 0x5100,
		Shadow16 = 0x5140,
		Shadow24 = 0x5150,
	}
	
	[StructLayout(LayoutKind.Sequential)]
	private struct Header {
		public uint flags;
		public Format format;
		public ushort width;
		public ushort height;
		public ushort depths;
		public ushort mip_levels;
		public unsafe fixed uint lod_offsets[3];
		public unsafe fixed uint mip_offsets[13];
	}
	
	[StructLayout(LayoutKind.Sequential)]
	private struct File {
		public Header header;
		public FFI.Vec data; // Vec<u8>
	}
	
	private static Aeth.Texture preview;
	static Tex() {
		preview = new(1024, 1024, new Aeth.TextureOptions{
			Format = SharpDX.DXGI.Format.B8G8R8A8_UNorm,
			Usage = SharpDX.Direct3D11.ResourceUsage.Dynamic,
			CpuAccessFlags = SharpDX.Direct3D11.CpuAccessFlags.Write,
		});
	}
	
	private static bool rChannel = true;
	private static bool gChannel = true;
	private static bool bChannel = true;
	private static bool aChannel = true;
	
	private unsafe File* tex;
	private FFI.Vec previewData;
	public string[][] paths;
	public Dictionary<string, object> settings;
	public Dictionary<string, (string, string)> infos;
	
	public unsafe Tex(string path): base(path) {
		var ext = "." + path.Split(".").Last();
		validImports = new string[3]{ext, ".dds", ".png"};
		validExports = new string[3]{ext, ".dds", ".png"};
		
		paths = new string[1][]{new string[2]{"", path}};
		
		settings = new();
		infos = new();
		
		ReloadPreview();
	}
	
	unsafe ~Tex() {
		FFI.Extern.FreeObject((IntPtr)tex);
	}
	
	public unsafe void ReloadPreview() {
		var layers = new FFI.Array[paths.Length];
		for(var i = 0; i < paths.Length; i++) {
			var l = new FFI.Str[paths[i].Length];
			for(var j = 0; j < paths[i].Length; j++)
				l[j] = paths[i][j];
			layers[i] = FFI.Array.Create(l);
		}
		
		var v = new List<string>();
		foreach(var s in settings)
			v.Add($"\"{s.Key}\":{Util.Settings.ByType(infos[s.Key].Item2, s.Value)}");
		
		var f = LoadFile(FFI.Array.Create(layers), $"{{{(string.Join(',', v))}}}");
		if(f.IsOk(out IntPtr ptr))
			tex = (File*)ptr;
		else {
			ShowError(f.Error());
			return;
		}
		
		if(tex->header.format == Format.A8 || tex->header.format == Format.L8) {
			rChannel = true;
			gChannel = true;
			bChannel = true;
			aChannel = true;
		}
		
		previewData = GetPreview(tex, (ushort)preview.Width, (ushort)preview.Height).Unwrap<FFI.Vec>();
		preview.WriteData(previewData.DataPtr);
	}
	
	protected override unsafe void DrawViewer() {
		{
			var changed = false;
			
			var h = tex->header;
			if(h.format != Format.A8 && h.format != Format.L8) {
				changed |= ImGui.Checkbox("R", ref rChannel);
				ImGui.SameLine();
				changed |= ImGui.Checkbox("G", ref gChannel);
				ImGui.SameLine();
				changed |= ImGui.Checkbox("B", ref bChannel);
				ImGui.SameLine();
				changed |= ImGui.Checkbox("A", ref aChannel);
				ImGui.SameLine();
				
				if(changed) {
					var data = (byte*)previewData.DataPtr;
					var target = preview.PinData();
					if(!rChannel && !gChannel && !bChannel && aChannel)
						for(int i = 0; i < preview.Width * preview.Height * 4; i += 4) {
							var a = data[i + 3];
							target[i    ] = a;
							target[i + 1] = a;
							target[i + 2] = a;
							target[i + 3] = (byte)255;
						}
					else
						for(int i = 0; i < preview.Width * preview.Height * 4; i += 4) {
							target[i    ] = bChannel ? data[i    ] : (byte)0;
							target[i + 1] = gChannel ? data[i + 1] : (byte)0;
							target[i + 2] = rChannel ? data[i + 2] : (byte)0;
							target[i + 3] = aChannel ? data[i + 3] : (byte)255;
						}
					preview.FreeData();
				}
				
				changed = false;
			}
			
			ImGui.SetNextItemWidth(200);
			if(ImGui.BeginCombo("##layers", "Layers", ImGuiComboFlags.HeightRegular)) {
				foreach(var s in settings) {
					var val = s.Value;
					changed |= Aeth.Option.ByType(infos[s.Key].Item1, infos[s.Key].Item2, ref val);
					settings[s.Key] = val;
				}
				
				ImGui.EndCombo();
			}
			
			if(changed)
				ReloadPreview();
		}
		
		if(tex != null) {
			// var rounding = Aeth.S.FrameRounding;
			var pos = ImGui.GetCursorScreenPos();
			var size = ImGui.GetContentRegionAvail();
			
			// This is dumb, TODO: use a shader or smth
			Aeth.Draw.AddRectFilled(pos, pos + size, 0xFF303030);
			for(int x = 0; x < Math.Ceiling(size.X / 32f); x += 2)
				for(int y = 0; y < Math.Ceiling(size.Y / 32f); y++) {
					var p = new Vector2(pos.X + x * 32 + (y % 2 == 0 ? 0 : 32), pos.Y + y * 32);
					Aeth.Draw.AddRectFilled(p, p + new Vector2(32, 32), 0xFFCFCFCF);
				}
			
			var scale = Math.Min(size.X / tex->header.width, size.Y / tex->header.height);
			var w = tex->header.width * scale;
			var h = tex->header.height * scale;
			pos.X += (size.X - w) / 2;
			pos.Y += (size.Y - h) / 2;
			// rounding -= Math.Min(rounding, Math.Max(size.X - w, size.Y - h) / 2);
			Aeth.Draw.AddImage(preview, pos, pos + new Vector2(w, h), Vector2.Zero, Vector2.One, 0xFFFFFFFF);
			// Aeth.Draw.AddImageRounded(preview, pos, pos + new Vector2(w, h), Vector2.Zero, Vector2.One, 0xFFFFFFFF, rounding);
		}
	}
	
	public unsafe override void SaveFile(string filename, string format) {
		// This saves it with all layers, TODO: change that? idk
		SaveFile(tex, filename, format);
	}
	
	[DllImport("aetherment_core.dll", EntryPoint = "viewer_tex_load")]
	private static extern FFI.Result LoadFile(FFI.Array path, FFI.Str settings);
	[DllImport("aetherment_core.dll", EntryPoint = "viewer_tex_preview")]
	private static unsafe extern FFI.Result GetPreview(File* tex, ushort width, ushort height);
	[DllImport("aetherment_core.dll", EntryPoint = "viewer_tex_save")]
	private static unsafe extern FFI.Result SaveFile(File* tex, FFI.Str filename, FFI.Str format);
}