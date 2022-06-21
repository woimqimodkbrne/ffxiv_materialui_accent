using System;
using System.Runtime.InteropServices;
using System.Text;

namespace Aetherment.FFI;

// this is dumb
// cant have explicit and marshal stuff on generics
[StructLayout(LayoutKind.Explicit)]
public class Array {
	[FieldOffset(0x0)] private IntPtr ptr;
	[FieldOffset(0x8)] private ulong length;
	
	public Array(IntPtr ptr, ulong length) {
		this.ptr = ptr;
		this.length = length;
	}
	
	~Array() {
		if(ptr != IntPtr.Zero)
			Marshal.FreeHGlobal(ptr);
	}
	
	// just expand this list
	public static implicit operator Array(byte[] array) => Create(array);
	public static implicit operator Array(short[] array) => Create(array);
	public static implicit operator Array(ushort[] array) => Create(array);
	public static implicit operator Array(int[] array) => Create(array);
	public static implicit operator Array(uint[] array) => Create(array);
	public static implicit operator Array(long[] array) => Create(array);
	public static implicit operator Array(ulong[] array) => Create(array);
	public static implicit operator Array(string[] array) {
		var rarray = new Str[array.Length];
		for(var i = 0; i < array.Length; i++)
			rarray[i] = array[i];
		return Create<Str>(rarray);
	}
	
	public static Array Create<T>(T[] array) {
		var length = array.Length;
		var element_size = Marshal.SizeOf<T>();
		var ptr = Marshal.AllocHGlobal(length * element_size);
		
		for(var i = 0; i < array.Length; i++)
			if(array[i] is T e)
				Marshal.StructureToPtr<T>(e, ptr + i * element_size, false);
		
		return new Array(ptr, (ulong)length);
	}
}