using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using Dalamud.Interface.ImGuiFileDialog;
using Dialog = Dalamud.Interface.ImGuiFileDialog.FileDialog;

namespace Aetherment;

public class FileDialog {
	private Dialog? dialog;
	private string? result;
	
	public FileDialog() {
		openFileDialogDelegate = OpenFileDialog;
	}
	
	public void Draw() {
		// PluginLog.Log($"{dialog != null}");
		if(dialog != null && dialog.Draw()) {
			result = dialog.GetIsOk() ? dialog.GetResults()[0] : null;
			dialog = null;
		}
	}
	
	// this will break if its called with one still active, too bad!
	public OpenFileDialogDelegate openFileDialogDelegate;
	public delegate byte OpenFileDialogDelegate(byte mode, FFI.String title, FFI.String filter, FFI.String name, IntPtr outpath);
	public byte OpenFileDialog(byte mode, FFI.String title, FFI.String filter, FFI.String name, IntPtr outpath) {
		// TOOD: save path and selected extension somewhere
		var dir = Environment.GetFolderPath(Environment.SpecialFolder.Personal);
		var nameS = (string)name;
		if(mode == 0)
			dialog = new Dialog("OpenFileDialog", title, filter, dir, nameS, nameS.Split(".").Last(), 1, false, ImGuiFileDialogFlags.None);
		else if(mode == 1)
			dialog = new Dialog("SaveFileDialog", title, filter, dir, nameS, nameS.Split(".").Last(), 1, false, ImGuiFileDialogFlags.ConfirmOverwrite);
		else
			return 0;
		
		dialog.Show();
		
		while(dialog != null)
			Thread.Sleep(100);
		
		if(result == null)
			return 0;
		
		var length = Encoding.UTF8.GetByteCount(result);
		if(length > (int)Marshal.PtrToStructure<ulong>(outpath + 8))
			return 0;
		
		unsafe {
			fixed(char* chars = result) {
				Encoding.UTF8.GetBytes(chars, result.Length, (byte*)Marshal.ReadIntPtr(outpath), length);
			}
			Marshal.StructureToPtr((ulong)length, outpath + 16, false);
		}
		
		return 1;
	}
}