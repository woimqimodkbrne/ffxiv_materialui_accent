using System.IO;
using System.Reflection;
using Newtonsoft.Json;

namespace Aetherment;

public class Config {
	private static string configPath => $"{Aetherment.Interface.ConfigDirectory.FullName}/config.json";
	
	[JsonIgnore]
	private int hash;
	
	public bool FileExplorer = true;
	public bool ModDev = true;
	
	public override int GetHashCode() {
		int result = 0;
		foreach(var p in typeof(Config).GetFields(BindingFlags.Public | BindingFlags.Instance))
			result = (result * 7919) ^ (p.GetValue(this)?.GetHashCode() ?? 0);
		return result;
	}
	
	public void MarkForChanges() {
		hash = this.GetHashCode();
	}
	
	public void Save() {
		if(hash == this.GetHashCode())
			return;
		PluginLog.Log("save");
		File.WriteAllText(configPath, JsonConvert.SerializeObject(this));
	}
	
	public static Config Load() {
		return File.Exists(configPath) ? (JsonConvert.DeserializeObject<Config>(File.ReadAllText(configPath)) ?? new Config()) : new Config();
	}
}