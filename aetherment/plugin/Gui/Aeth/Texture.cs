using System;
using SharpDX.Direct3D;
using SharpDX.Direct3D11;
using Main = Aetherment.Aetherment;

namespace Aetherment.Gui;

public static partial class Aeth {
	public struct TextureOptions {
		public SharpDX.DXGI.Format Format = SharpDX.DXGI.Format.R8G8B8A8_UNorm;
		public ResourceUsage Usage = ResourceUsage.Immutable;
		public CpuAccessFlags CpuAccessFlags = CpuAccessFlags.None;
		
		public TextureOptions() {}
	}
	
	public class Texture {
		private ShaderResourceView? resource;
		
		public int Width;
		public int Height;
		
		public Texture() {
			resource = null;
			Width = 1;
			Height = 1;
		}
		
		public Texture(byte[] data, uint width, uint height, TextureOptions? options = null) {
			unsafe {
				fixed(void* dataPtr = data) {
					CreateTexture(new IntPtr(dataPtr), width, height, options);
				}
			}
		}
		
		public Texture(IntPtr data, uint width, uint height, TextureOptions? options = null) =>
			CreateTexture(data, width, height, options);
		
		public Texture(uint width, uint height, TextureOptions? options = null) =>
			CreateTexture(null, width, height, options);
		
		private unsafe void CreateTexture(IntPtr? data, uint width, uint height, TextureOptions? options = null) {
			options ??= new TextureOptions();
			
			Width = (int)width;
			Height = (int)height;
			
			var desc = new Texture2DDescription {
				Width = Width,
				Height = Height,
				MipLevels = 1,
				ArraySize = 1,
				Format = options.Value.Format,
				SampleDescription = new SharpDX.DXGI.SampleDescription(1, 0),
				Usage = options.Value.Usage,
				BindFlags = BindFlags.ShaderResource,
				CpuAccessFlags = options.Value.CpuAccessFlags,
				OptionFlags = ResourceOptionFlags.None,
			};
			
			// TODO: mby support formats besides the one above, calculate pitch accordingly
			// https://docs.microsoft.com/en-us/windows/win32/direct3ddds/dx-graphics-dds-pguide
			// var pitch = (Width * 4 + 7) / 8;
			var pitch = Width * 4;
			
			unsafe {
				var tex = data == null ?
					new Texture2D(Aetherment.Device, desc) :
					new Texture2D(Aetherment.Device, desc, new SharpDX.DataRectangle(data.Value, pitch));
				resource = new ShaderResourceView(Aetherment.Device, tex, new ShaderResourceViewDescription {
					Format = desc.Format,
					Dimension = ShaderResourceViewDimension.Texture2D,
					Texture2D = {MipLevels = desc.MipLevels},
				});
				tex.Dispose();
			}
		}
		
		~Texture() {
			resource?.Dispose();
		}
		
		public unsafe void WriteData(IntPtr data) {
			var box = Main.Device.ImmediateContext.MapSubresource(resource!.Resource, 0, SharpDX.Direct3D11.MapMode.WriteDiscard, SharpDX.Direct3D11.MapFlags.None);
			var origin = (byte*)data;
			var target = (byte*)box.DataPointer;
			for(var i = 0; i < Width * Height * 4; i++) // This always assumes its a uncompressed texture!! TODO: fix that probably
				target[i] = origin[i];
			Main.Device.ImmediateContext.UnmapSubresource(resource!.Resource, 0);
		}
		
		public static implicit operator IntPtr(Texture tex) => tex.resource == null ? IntPtr.Zero : tex.resource.NativePointer;
	}
}