using System;
using System.Runtime.InteropServices;

namespace Aetherment.FFI;

public static class Extern {
	[DllImport("aetherment_core.dll", EntryPoint = "free_object")]
	public static extern void FreeObject(IntPtr obj);
}