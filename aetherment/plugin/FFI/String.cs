// TODO: get rid of this and use Str instead, should work for bothway now (probably)
using System.Runtime.InteropServices;
using System.Text;

namespace Aetherment.FFI;

[StructLayout(LayoutKind.Sequential)]
public struct String {
	private unsafe byte* ptr;
	private ulong capacity;
	private ulong length;
	
	public static implicit operator string(String str) {
		unsafe {
			return Encoding.UTF8.GetString(str.ptr, (int)str.length);
		}
	}
	
	public override string ToString() => (string)this;
}