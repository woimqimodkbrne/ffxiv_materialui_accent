using System;
using System.Runtime.InteropServices;
using System.Text;
using Dalamud.Logging;

namespace Aetherment.FFI;

// same as array, dumb shit
public class Vec : SafeHandle {
	public Vec() : base(IntPtr.Zero, true) {}
	public override bool IsInvalid { get { return false; } }
	
	protected override bool ReleaseHandle() {
		Extern.FreeObject(handle);
		return true;
	}
	
	~Vec() {
		this.Dispose();
	}
	
	// just add to the list cuz ofc
	public static implicit operator byte[](Vec vec) => Convert<byte>(vec.handle);
	public static implicit operator short[](Vec vec) => Convert<short>(vec.handle);
	public static implicit operator ushort[](Vec vec) => Convert<ushort>(vec.handle);
	public static implicit operator int[](Vec vec) => Convert<int>(vec.handle);
	public static implicit operator uint[](Vec vec) => Convert<uint>(vec.handle);
	public static implicit operator long[](Vec vec) => Convert<long>(vec.handle);
	public static implicit operator ulong[](Vec vec) => Convert<ulong>(vec.handle);
	public static implicit operator string[](Vec vec) {
		// cant use convert since String isnt explicit and cant make
		// it explicit since it inherits safehandle (yay c#)
		var ptr = Marshal.ReadIntPtr(vec.handle);
		var length = (int)Marshal.PtrToStructure<ulong>(vec.handle + 0x10);
		var array = new string[length];
		
		for(var i = 0; i < length; i++)
			array[i] = new FFI.String(ptr + i * 0x18);
		
		return array;
	}
	
	public static T[] Convert<T>(IntPtr handle) {
		var element_size = Marshal.SizeOf<T>();
		var ptr = Marshal.ReadIntPtr(handle);
		var length = (int)Marshal.PtrToStructure<ulong>(handle + 0x10);
		var array = new T[length];
		
		for(var i = 0; i < length; i++)
			if(Marshal.PtrToStructure<T>(ptr + i * element_size) is T e)
				array[i] = e;
		
		return array;
	}
}

// public class Vec<T> : SafeHandle {
// 	public Vec() : base(IntPtr.Zero, true) {}
// 	public override bool IsInvalid { get { return false; } }
	
// 	protected override bool ReleaseHandle() {
// 		Extern.FreeObject(handle);
// 		return true;
// 	}
	
// 	~Vec() {
// 		this.Dispose();
// 	}
	
// 	public static implicit operator T[](Vec<T> vec) {
// 		var element_size = Marshal.SizeOf<T>();
// 		var ptr = Marshal.ReadIntPtr(vec.handle);
// 		var length = (int)Marshal.PtrToStructure<ulong>(vec.handle + 0x10);
// 		var array = new T[length];
		
// 		for(var i = 0; i < length; i++)
// 			if(Marshal.PtrToStructure<T>(ptr + i * element_size) is T e)
// 				array[i] = e;
		
// 		return array;
// 	}
// }