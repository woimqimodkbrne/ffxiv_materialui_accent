global using Dalamud.Logging;

using System.Runtime.InteropServices;
using Dalamud.Game.Command;
using Dalamud.IoC;
using Dalamud.Plugin;

namespace Aetherment;

public class Aetherment : IDalamudPlugin {
	public string Name => "Aetherment";
	
	[PluginService][RequiredVersion("1.0")] public static DalamudPluginInterface Interface  {get; private set;} = null!;
	[PluginService][RequiredVersion("1.0")] public static CommandManager         Commands   {get; private set;} = null!;
	// [PluginService][RequiredVersion("1.0")] public static TitleScreenMenu        TitleMenu  {get; private set;} = null!;
	
	public static Server.Server Server {get; private set;} = null!;
	
	private const string command = "/aetherment";
	
	private bool aethermentGuiVisible = true;
	private Gui.Window.Aetherment.AethermentWindow aethermentGui;
	
	public Aetherment() {
		PluginLog.Log(cool_test("hello there c:"));
		
		Server = new();
		
		aethermentGui = new();
		
		Interface.UiBuilder.Draw += Draw;
		Commands.AddHandler(command, new CommandInfo(OnCommand) {
			HelpMessage = "Open Aetherment menu"
		});
	}
	
	public void Dispose() {
		Server.Dispose();
		
		Interface.UiBuilder.Draw -= Draw;
		Commands.RemoveHandler(command);
	}
	
	private void Draw() {
		if(aethermentGuiVisible)
			aethermentGui.Draw();
	}
	
	private void OnCommand(string cmd, string args) {
		if(cmd != command)
			return;
		
		if(args == "texfinder")
			return; //todo
		else
			aethermentGuiVisible = !aethermentGuiVisible;
	}
	
	[DllImport("aetherment_core.dll")] public static extern FFI.String cool_test(FFI.Str str);
}