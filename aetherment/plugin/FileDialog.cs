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
	private Dictionary<string, (Dialog?, string?)> dialogs;
	
	public FileDialog() {
		dialogs = new();
		openFileDialogDelegate = OpenFileDialog;
	}
	
	public void Draw() {
		var n = new Dictionary<string, (Dialog?, string?)>();
		foreach(var val in dialogs) {
			var dialog = val.Value.Item1;
			if(dialog != null && dialog.Draw())
				n[val.Key] = (null, dialog.GetIsOk() ? dialog.GetResults()[0] : null);
			else
				n[val.Key] = val.Value;
		}
		dialogs = n;
	}
	
	public OpenFileDialogDelegate openFileDialogDelegate;
	public delegate byte OpenFileDialogDelegate(byte mode, FFI.String title, FFI.String name, FFI.String filter, IntPtr outpath);
	public byte OpenFileDialog(byte mode, FFI.String title, FFI.String name, FFI.String filter, IntPtr outpath) {
		var id = $"{mode}\0{title}\0{filter}\0{name}"; // cba making a better way
		if(dialogs.TryGetValue(id, out var val)) {
			if(val.Item1 != null)
				return 2;
			
			if(val.Item2 == null) {
				dialogs.Remove(id);
				return 0;
			}
			
			var length = Encoding.UTF8.GetByteCount(val.Item2);
			if(length > (int)Marshal.PtrToStructure<ulong>(outpath + 8)) {
				dialogs.Remove(id);
				return 0;
			}
			
			unsafe {
				fixed(char* chars = val.Item2) {
					Encoding.UTF8.GetBytes(chars, val.Item2.Length, (byte*)Marshal.ReadIntPtr(outpath), length);
				}
				Marshal.StructureToPtr((ulong)length, outpath + 16, false);
			}
			
			dialogs.Remove(id);
			return 1;
		}
		
		var dir = Environment.GetFolderPath(Environment.SpecialFolder.Personal);
		var nameS = (string)name;
		if(mode == 0)
			dialogs[id] = (new Dialog("OpenFileDialog", title, filter, dir, nameS, nameS.Split(".").Last(), 1, false, ImGuiFileDialogFlags.None), null);
		else if(mode == 1)
			dialogs[id] = (new Dialog("SaveFileDialog", title, filter, dir, nameS, nameS.Split(".").Last(), 1, false, ImGuiFileDialogFlags.ConfirmOverwrite), null);
		else
			return 0;
		
		dialogs[id].Item1!.Show();
		return 2;
	}
}