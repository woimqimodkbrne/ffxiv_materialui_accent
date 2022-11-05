using System;
using System.Runtime.InteropServices;
using System.Text;

namespace Aetherment.FFI;

[StructLayout(LayoutKind.Explicit)]
public class Str {
	[FieldOffset(0x0)] public IntPtr ptr;
	[FieldOffset(0x8)] public ulong length;
	
	public Str(string str) {
		var length = Encoding.UTF8.GetByteCount(str);
		ptr = Marshal.AllocHGlobal(length);
		
		unsafe {
			var p = (byte*)ptr;
			fixed(char* chars = str) {
				Encoding.UTF8.GetBytes(chars, str.Length, p, length);
			}
		}
		
		this.length = (ulong)length;
	}
	
	~Str() {
		// unsafe{PluginLog.Log($"Free str {Encoding.UTF8.GetString((byte*)ptr, (int)length)}");}
		Marshal.FreeHGlobal(ptr);
	}
	
	public static implicit operator Str(string str) => new Str(str);
}