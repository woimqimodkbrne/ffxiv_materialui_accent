using System;
using System.Net.Http;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Newtonsoft.Json;

namespace Aetherment.Server;

public class Server {
	[DllImport("aetherment_core.dll", EntryPoint = "server_search")]
	private static extern FFI.String search(FFI.Str query, FFI.Array tags, int page);
	public static Mod[] Search(string query, short[] tags, int page) {
		return JsonConvert.DeserializeObject<Mod[]>(search(query, tags, page)) is Mod[] r ? r : new Mod[0];
	}
	
	[DllImport("aetherment_core.dll", EntryPoint = "server_mod")]
	private static extern FFI.String mod_page(FFI.Str id);
	public static Mod? ModPage(string id) {
		return JsonConvert.DeserializeObject<Mod>(mod_page(id));
	}
	
	// my 'beautiful' ffi is falling apart, idk how to handle structs with rust types
	[DllImport("aetherment_core.dll", EntryPoint = "server_download_preview")]
	private static extern IntPtr download_preview(FFI.Str modid, FFI.Str filename);
	public static Gui.Aeth.Texture DownloadPreview(string modid, string filename) {
		var imgptr = download_preview(modid, filename);
		return FFI.Extern.ReadImage(imgptr);
	}
}