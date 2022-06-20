using System;
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
		preview = new(2048, 2048, new Aeth.TextureOptions{
			Format = SharpDX.DXGI.Format.B8G8R8A8_UNorm,
			Usage = SharpDX.Direct3D11.ResourceUsage.Dynamic,
			CpuAccessFlags = SharpDX.Direct3D11.CpuAccessFlags.Write,
		});
	}
	
	private unsafe File* tex;
	
	public unsafe Tex(string path): base(path) {
		var ext = "." + path.Split(".").Last();
		validImports = new string[3]{ext, ".dds", ".png"};
		validExports = new string[3]{ext, ".dds", ".png"};
		
		var f = LoadFile(path);
		if(f.IsOk(out IntPtr ptr))
			tex = (File*)ptr;
		else {
			ShowError(f.Error());
			return;
		}
		
		preview.WriteData(GetPreview(tex, 2048, 2048).Unwrap<FFI.Vec>().DataPtr);
	}
	
	unsafe ~Tex() {
		if(tex == null)
			return;
		
		FFI.Extern.FreeObject((IntPtr)tex);
	}
	
	protected override unsafe void DrawViewer() {
		if(preview != null) {
			var rounding = Aeth.S.FrameRounding;
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
			rounding -= Math.Min(rounding, Math.Max(size.X - w, size.Y - h) / 2);
			Aeth.Draw.AddImageRounded(preview, pos, pos + new Vector2(w, h), Vector2.Zero, Vector2.One, 0xFFFFFFFF, rounding);
		}
	}
	
	public unsafe override void SaveFile(string filename, string format) {
		SaveFile(tex, filename, format);
	}
	
	[DllImport("aetherment_core.dll", EntryPoint = "viewer_tex_load")]
	private static extern FFI.Result LoadFile(FFI.Str path);
	[DllImport("aetherment_core.dll", EntryPoint = "viewer_tex_preview")]
	private static unsafe extern FFI.Result GetPreview(File* tex, ushort width, ushort height);
	[DllImport("aetherment_core.dll", EntryPoint = "viewer_tex_save")]
	private static unsafe extern FFI.Result SaveFile(File* tex, FFI.Str filename, FFI.Str format);
}