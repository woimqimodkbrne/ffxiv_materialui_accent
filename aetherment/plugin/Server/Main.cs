using System;
using System.Net.Http;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Newtonsoft.Json;

namespace Aetherment.Server;

public class Server {
	[DllImport("aetherment_core.dll", EntryPoint = "server_search")]
	private static extern FFI.Result search(FFI.Str query, FFI.Array tags, int page);
	public static Mod[] Search(string query, short[] tags, int page) {
		return JsonConvert.DeserializeObject<Mod[]>(search(query, tags, page).Unwrap()) is Mod[] r ? r : new Mod[0];
	}
	
	[DllImport("aetherment_core.dll", EntryPoint = "server_mod")]
	private static extern FFI.Result mod_page(FFI.Str id);
	public static Mod? ModPage(string id) {
		return JsonConvert.DeserializeObject<Mod>(mod_page(id).Unwrap());
	}
	
	[DllImport("aetherment_core.dll", EntryPoint = "server_download_preview")]
	private static extern FFI.Result download_preview(FFI.Str modid, FFI.Str filename);
	public static Gui.Aeth.Texture DownloadPreview(string modid, string filename) {
		var img = download_preview(modid, filename).Unwrap<FFI.Extern.Img>();
		return FFI.Extern.ReadImage(img);
	}
}