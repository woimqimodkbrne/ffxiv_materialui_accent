namespace Aetherment.Server;

public struct Mod {
	public int Id;
	public string Name;
	public string Description;
	public IdName Author;
	public IdName[] Contributors;
	public IdName? MainMod;
	public IdName[] Dependencies;
	public long SizeInstall;
	public long SizeDownload;
	public short[] Tags;
	public string[] Previews;
	public bool nsfw;
	public int likes;
	public int downloads;
}

public struct IdName {
	public int Id;
	public string Name;
}