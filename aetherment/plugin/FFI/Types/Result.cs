using System;
using System.Runtime.InteropServices;
using System.Text;
using Dalamud.Logging;

namespace Aetherment.FFI;

public class Result : SafeHandle {
	public Result() : base(IntPtr.Zero, true) {}
	public override bool IsInvalid { get { return false; } }
	
	protected override bool ReleaseHandle() {
		Extern.FreeObject(handle);
		return true;
	}
	
	~Result() {
		this.Dispose();
	}
	
	public bool IsError(out string err) {
		var is_err = Marshal.PtrToStructure<byte>(handle) == 1;
		if(is_err)
			err = Marshal.PtrToStructure<FFI.String>(handle + 0x8)!;
		else
			err = "";
		
		return is_err;
	}
	
	// i wish the worst upon this language
	public bool IsOk<T>(out T obj) {
		var is_ok = Marshal.PtrToStructure<byte>(handle) == 0;
		if(is_ok)
			obj = Marshal.PtrToStructure<T>(handle + 0x8)!;
		else
			obj = default(T)!;
		
		return is_ok;
	}
	
	public bool IsOk<T>(out T[] obj) {
		var is_ok = Marshal.PtrToStructure<byte>(handle) == 0;
		if(is_ok)
			obj = Marshal.PtrToStructure<Vec>(handle + 0x8)!.Convert<T>();
		else
			obj = default(T[])!;
		
		return is_ok;
	}
	
	public bool IsOk(out string obj) {
		var is_ok = Marshal.PtrToStructure<byte>(handle) == 0;
		if(is_ok)
			obj = Marshal.PtrToStructure<FFI.String>(handle + 0x8)!;
		else
			obj = default(string)!;
		
		return is_ok;
	}
	
	public bool IsOk(out string[] obj) {
		var is_ok = Marshal.PtrToStructure<byte>(handle) == 0;
		if(is_ok)
			obj = Marshal.PtrToStructure<Vec>(handle + 0x8)!;
		else
			obj = default(string[])!;
		
		return is_ok;
	}
	
	public T Unwrap<T>() {
		if(IsError(out var err))
			throw new Exception(err);
		
		return Marshal.PtrToStructure<T>(handle + 0x8)!;
	}
	
	public string Unwrap() {
		if(IsError(out var err))
			throw new Exception(err);
		
		return Marshal.PtrToStructure<FFI.String>(handle + 0x8)!;
	}
}