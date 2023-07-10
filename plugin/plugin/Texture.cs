// This is dumb
// TODO: figure out a way to get wgpu working properly, probably wrong device or smth
// or perhabs just use raw windows d3d11 api

using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using SharpDX.Direct3D;
using SharpDX.Direct3D11;

namespace Aetherment;

public class TextureManager {
	[StructLayout(LayoutKind.Explicit)]
	public struct TextureOptions {
		[FieldOffset(0x00)] public int width;
		[FieldOffset(0x04)] public int height;
		[FieldOffset(0x08)] public SharpDX.DXGI.Format format;
		[FieldOffset(0x0C)] public ResourceUsage usage;
		[FieldOffset(0x10)] public CpuAccessFlags cpuflags;
	}
	
	public TextureManager() {
		createTexture = CreateTexture;
		pinData = PinData;
		unpinData = UnpinData;
		destroyResource = DestroyResource;
	}
	
	public CreateTextureDelegate createTexture;
	public delegate IntPtr CreateTextureDelegate(TextureOptions options);
	public unsafe IntPtr CreateTexture(TextureOptions options) {
		var desc = new Texture2DDescription {
			Width = options.width,
			Height = options.height,
			MipLevels = 1,
			ArraySize = 1,
			Format = options.format,
			SampleDescription = new SharpDX.DXGI.SampleDescription(1, 0),
			Usage = options.usage,
			BindFlags = BindFlags.ShaderResource,
			CpuAccessFlags = options.cpuflags,
			OptionFlags = ResourceOptionFlags.None,
		};
		
		unsafe {
			var tex = new Texture2D(Aetherment.Device, desc);
			var resource = new ShaderResourceView(Aetherment.Device, tex, new ShaderResourceViewDescription {
				Format = desc.Format,
				Dimension = ShaderResourceViewDimension.Texture2D,
				Texture2D = {MipLevels = desc.MipLevels},
			});
			tex.Dispose();
			
			return resource.NativePointer;
		}
	}
	
	public DestroyResourceDelegate destroyResource;
	public delegate void DestroyResourceDelegate(IntPtr resource);
	public void DestroyResource(IntPtr resource) {
		new ShaderResourceView(resource).Dispose();
	}
	
	public PinDataDelegate pinData;
	public delegate IntPtr PinDataDelegate(IntPtr resource);
	public IntPtr PinData(IntPtr resource) {
		var box = Aetherment.Device.ImmediateContext.MapSubresource(new ShaderResourceView(resource).Resource, 0, SharpDX.Direct3D11.MapMode.WriteDiscard, SharpDX.Direct3D11.MapFlags.None);
		return box.DataPointer;
	}
	
	public UnpinDataDelegate unpinData;
	public delegate void UnpinDataDelegate(IntPtr resource);
	public void UnpinData(IntPtr resource) {
		Aetherment.Device.ImmediateContext.UnmapSubresource(new ShaderResourceView(resource).Resource, 0);
	}
}