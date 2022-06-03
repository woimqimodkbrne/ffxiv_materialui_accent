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
	
	// my 'beautiful' ffi is falling apart, idk how to handle structs with rust types
	[DllImport("aetherment_core.dll", EntryPoint = "server_download_preview")]
	private static extern IntPtr download_preview(int modid, FFI.Str filename);
	public static Gui.Aeth.Texture DownloadPreview(int modid, string filename) {
		var imgptr = download_preview(modid, filename);
		var width = Marshal.PtrToStructure<uint>(imgptr);
		var height = Marshal.PtrToStructure<uint>(imgptr + 4);
		var img = FFI.Vec.Convert<byte>(imgptr + 8);
		FFI.Extern.FreeObject(imgptr);
		
		return new Gui.Aeth.Texture(img, width, height);
	}
}

// public class Server : IDisposable {
// 	private const string serverUrl = "http://localhost:8080";
	
// 	private HttpClient httpClient;
	
// 	public Server() {
// 		var handler = new HttpClientHandler();
// 		handler.Proxy = null;
// 		handler.UseProxy = false;
		
// 		httpClient = new HttpClient(handler);
// 	}
	
// 	public void Dispose() {
// 		httpClient.Dispose();
// 	}
	
// 	public async Task<Mod[]> Search(string query, short[] tags, byte page) {
// 		var url = $"{serverUrl}/search.json?query={query}&tags={string.Join(",", tags)}&page={page}";
// 		var result = await httpClient.GetStringAsync(url);
// 		return JsonConvert.DeserializeObject<Mod[]>(result) is Mod[] r ? r : new Mod[0];
// 	}
// }