// TODO: continue this cba atm

using System;
using System.Runtime.InteropServices;
using ImGuiNET;

namespace Aetherment.Gui.Window.Aetherment.Explorer.Viewer;

public class Mtrl: Viewer {
	[StructLayout(LayoutKind.Sequential)]
	private struct File {
		public FFI.String shader;
		// public FFI.Vec textures;
		public FFI.Vec uvsets;
		public FFI.Vec colorsets;
		public FFI.Vec unk;
		public FFI.Vec colorset_datas;
		public FFI.Vec samplers;
		public FFI.Vec constants;
	}
	
	[StructLayout(LayoutKind.Sequential)]
	private struct ColorsetRow {
		public Half diffuse_r;
		public Half diffuse_g;
		public Half diffuse_b;
		public Half specular_strength;
		public Half specular_r;
		public Half specular_g;
		public Half specular_b;
		public Half gloss_strength;
		public Half emissive_r;
		public Half emissive_g;
		public Half emissive_b;
		public Half material;
		public Half material_repeat_x;
		public Half material_skew_x;
		public Half material_skew_y;
		public Half material_repeat_y;
	}
	
	[StructLayout(LayoutKind.Sequential)]
	struct Sampler {
		public uint typ;
		public uint flags;
		public FFI.String path;
	}
	
	[StructLayout(LayoutKind.Sequential)]
	struct Constant {
		public uint typ; // this changes between loads on the 2nd iteration and idfk why
		public ushort offset;
		public ushort size;
	}
	
	private unsafe File* file;
	
	public unsafe Mtrl(string path): base(path) {
		var f = LoadFile(path);
		if(f.IsOk(out IntPtr ptr))
			file = (File*)ptr;
		else {
			ShowError(f.Error());
			return;
		}
	}
	
	unsafe ~Mtrl() {
		FFI.Extern.FreeObject((IntPtr)file);
	}
	
	protected unsafe override void DrawViewer() {
		ImGui.Text("- Shader:");
		ImGui.TextUnformatted(file->shader);
		
		// ImGui.Text("- Textures:");
		// foreach(var path in (string[])file->textures)
		// 	ImGui.TextUnformatted(path);
		
		ImGui.Text("- UvSets:");
		foreach(var path in (string[])file->uvsets)
			ImGui.TextUnformatted(path);
		
		ImGui.Text("- Colorsets:");
		foreach(var path in (string[])file->colorsets)
			ImGui.TextUnformatted(path);
		
		ImGui.Text("- Unknown:");
		ImGui.TextUnformatted(string.Join(',', file->unk.Convert<byte>()));
		
		ImGui.Text("- Samplers:");
		foreach(var sampler in file->samplers.Convert<Sampler>())
			ImGui.TextUnformatted($"{sampler.typ}, {sampler.flags}, {sampler.path}");
		
		ImGui.Text("- Constants:");
		foreach(var sampler in file->samplers.Convert<Constant>())
			ImGui.TextUnformatted($"{sampler.typ}, {sampler.offset}, {sampler.size}");
		
		ImGui.Text("- Colorsets data:");
		var rows = file->colorset_datas.Convert<ColorsetRow>();
		for(var rowi = 0; rowi < 16; rowi++) {
			var row = rows[rowi];
			ImGui.TextUnformatted($"[{rowi}] diffuse({row.diffuse_r},{row.diffuse_g},{row.diffuse_b})\n" +
			                      $"         specular({row.specular_r},{row.specular_g},{row.specular_b}) {row.specular_strength}x\n" +
			                      $"         emissive({row.emissive_r},{row.emissive_g},{row.emissive_b})\n" +
			                      $"         gloss({row.gloss_strength})\n" +
			                      $"         material(index: {(ushort)((float)row.material * 64)}, repeat x: {row.material_repeat_x}, repeat y: {row.material_repeat_y}, " +
			                                        $"skew x: {row.material_skew_x}, skew y: {row.material_skew_y})");
		}
	}
	
	[DllImport("aetherment_core.dll", EntryPoint = "viewer_mtrl_load")]
	private static extern FFI.Result LoadFile(FFI.Str path);
}