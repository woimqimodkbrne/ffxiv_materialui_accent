using System;
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
	
	private unsafe File* tex;
	private Aeth.Texture preview = null!; // assign preview in a function called in constructor 'Non-nullable fiel~' fuck off
	
	public unsafe Tex(string path): base(path) {
		var f = LoadFile(path);
		if(f.IsOk(out IntPtr ptr))
			tex = (File*)ptr;
		else {
			ShowError(f.Error());
			return;
		}
		
		PluginLog.Log($"{tex->header.format} {tex->header.width} {tex->header.height} {((byte[])tex->data).Length}");
		
		LoadPreview();
	}
	
	unsafe ~Tex() {
		if(tex == null)
			return;
		
		FFI.Extern.FreeObject((IntPtr)tex);
	}
	
	private unsafe void LoadPreview() {
		preview = new(tex->data.DataPtr, tex->header.width, tex->header.height, new Aeth.TextureOptions{
			Format = SharpDX.DXGI.Format.B8G8R8A8_UNorm,
		});
	}
	
	protected override void DrawViewer() {
		if(preview != null)
			Aeth.BoxedImage(ImGui.GetContentRegionAvail(), preview);
	}
	
	[DllImport("aetherment_core.dll", EntryPoint = "viewer_tex_load")]
	private static extern FFI.Result LoadFile(FFI.Str path);
	[DllImport("aetherment_core.dll", EntryPoint = "viewer_tex_save")]
	private static unsafe extern FFI.Result SaveFile(File* tex, FFI.Str filename, FFI.Str format);
}