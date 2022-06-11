using System;
using System.Runtime.InteropServices;

namespace Aetherment.FFI;

public static class Extern {
	[DllImport("aetherment_core.dll", EntryPoint = "free_object")]
	public static extern void FreeObject(IntPtr obj);
	
	public static Gui.Aeth.Texture ReadImage(IntPtr imgptr) {
		var width = Marshal.PtrToStructure<uint>(imgptr);
		var height = Marshal.PtrToStructure<uint>(imgptr + 4);
		var img = FFI.Vec.Convert<byte>(imgptr + 8);
		FFI.Extern.FreeObject(imgptr);
		
		return new Gui.Aeth.Texture(img, width, height);
	}
	
	[DllImport("aetherment_core.dll", EntryPoint = "read_image")]
	private static extern IntPtr read_image(Str path);
	public static Gui.Aeth.Texture ReadImage(string path) => ReadImage(read_image(path));
}