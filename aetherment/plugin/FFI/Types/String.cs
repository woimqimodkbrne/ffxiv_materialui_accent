using System;
using System.Runtime.InteropServices;
using System.Text;
using Dalamud.Logging;

namespace Aetherment.FFI;

public class String : SafeHandle {
	// fuck off, i dont care
	public String() : base(IntPtr.Zero, true) {}
	public override bool IsInvalid { get { return false; } }
	
	protected override bool ReleaseHandle() {
		Extern.FreeObject(handle);
		return true;
	}
	
	~String() {
		this.Dispose();
	}
	
	public static implicit operator string(FFI.String str) => str.ToString();
	
	public override string ToString() {
		unsafe {
			return Encoding.UTF8.GetString((byte*)Marshal.PtrToStructure<ulong>(handle), (int)Marshal.PtrToStructure<ulong>(handle + 0x10));
		}
	}
}