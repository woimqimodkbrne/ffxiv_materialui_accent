using System;
using System.Runtime.InteropServices;
using System.Text;
using Dalamud.Logging;

namespace Aetherment.FFI;

[StructLayout(LayoutKind.Explicit)]
public class String {
	[FieldOffset(0x00)] private unsafe byte* ptr;
	[FieldOffset(0x08)] private ulong capacity;
	[FieldOffset(0x10)] private ulong length;
	
	public static implicit operator string(String str) {
		unsafe {
			return Encoding.UTF8.GetString(str.ptr, (int)str.length);
		}
	}
	
	public override string ToString() => (string)this;
}

// public class String : SafeHandle {
// 	// fuck off, i dont care
// 	public String() : base(IntPtr.Zero, true) {}
// 	public String(IntPtr ptr) : base(ptr, true) {}
// 	public override bool IsInvalid { get { return false; } }
	
// 	protected override bool ReleaseHandle() {
// 		Extern.FreeObject(handle);
// 		return true;
// 	}
	
// 	~String() {
// 		this.Dispose();
// 	}
	
// 	public static implicit operator string(String str) {
// 		unsafe {
// 			return Encoding.UTF8.GetString((byte*)Marshal.PtrToStructure<ulong>(str.handle), (int)Marshal.PtrToStructure<ulong>(str.handle + 0x10));
// 		}
// 	}

// 	public override string ToString() => (string)this;
// }