namespace Aetherment.Server;

public struct Mod {
	public string Id;
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
	public bool Nsfw;
	public int Likes;
	public int Downloads;
}

public struct IdName {
	public string Id;
	public string Name;
}