using System;
using System.Runtime.InteropServices;

namespace Aetherment.FFI;

public static class Extern {
	[DllImport("aetherment_core.dll", EntryPoint = "free_object")]
	public static extern void FreeObject(IntPtr obj);
	
	[StructLayout(LayoutKind.Explicit)]
	public struct Img {
		[FieldOffset(0x0)] public uint Width;
		[FieldOffset(0x4)] public uint Height;
		[FieldOffset(0x8)] public Vec Data;
	}
	
	public static Gui.Aeth.Texture ReadImage(Img img) => new Gui.Aeth.Texture(img.Data, img.Width, img.Height);
	
	[DllImport("aetherment_core.dll", EntryPoint = "read_image")]
	private static extern Result read_image(Str path);
	public static Gui.Aeth.Texture ReadImage(string path) => ReadImage(read_image(path).Unwrap<Img>());
}