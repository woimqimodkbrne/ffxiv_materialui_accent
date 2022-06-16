using System;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using Dalamud.Logging;

namespace Aetherment.FFI;

[StructLayout(LayoutKind.Explicit)]
public class Vec {
	[FieldOffset(0x00)] private IntPtr ptr;
	[FieldOffset(0x08)] private ulong capacity;
	[FieldOffset(0x10)] public ulong length;
	
	public static implicit operator sbyte[](Vec vec) => vec.Convert<sbyte>();
	public static implicit operator byte[](Vec vec) => vec.Convert<byte>();
	public static implicit operator short[](Vec vec) => vec.Convert<short>();
	public static implicit operator ushort[](Vec vec) => vec.Convert<ushort>();
	public static implicit operator int[](Vec vec) => vec.Convert<int>();
	public static implicit operator uint[](Vec vec) => vec.Convert<uint>();
	public static implicit operator long[](Vec vec) => vec.Convert<long>();
	public static implicit operator ulong[](Vec vec) => vec.Convert<ulong>();
	public static implicit operator string[](Vec vec) => vec.Convert<FFI.String>().Select((e) => (string)e).ToArray();
	// public static implicit operator string[](Vec vec) => vec.Convert<FFI.String>().Cast<string>().ToArray();
	
	public T[] Convert<T>() {
		var element_size = Marshal.SizeOf<T>();
		var array = new T[length];
		
		for(var i = 0; i < (int)length; i++)
			if(Marshal.PtrToStructure<T>(ptr + i * element_size) is T e)
				array[i] = e;
		
		return array;
	}
}