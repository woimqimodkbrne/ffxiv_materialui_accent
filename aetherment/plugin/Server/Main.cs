using System;
using System.Net.Http;
using System.Runtime.InteropServices;
using System.Threading.Tasks;
using Newtonsoft.Json;

namespace Aetherment.Server;

public class Server {
	public static Mod[] Search(string query, short[] tags, int page) {
		return JsonConvert.DeserializeObject<Mod[]>(server_search(query, tags, page)) is Mod[] r ? r : new Mod[0];
	}
	
	[DllImport("aetherment_core.dll")]
	private static extern FFI.String server_search(FFI.Str query, FFI.Array tags, int page);
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