using System;
using SharpDX.Direct3D;
using SharpDX.Direct3D11;

namespace Aetherment.Gui;

public static partial class Aeth {
	public struct TextureOptions {
		
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
		
		public Texture(byte[] data, uint width, uint height, TextureOptions options = new TextureOptions()) {
			unsafe {
				fixed(void* dataPtr = data) {
					CreateTexture(new IntPtr(dataPtr), width, height, options);
				}
			}
		}
		
		public Texture(IntPtr data, uint width, uint height, TextureOptions options = new TextureOptions()) =>
			CreateTexture(data, width, height, options);
		
		private unsafe void CreateTexture(IntPtr data, uint width, uint height, TextureOptions options = new TextureOptions()) {
			Width = (int)width;
			Height = (int)height;
			
			var desc = new Texture2DDescription {
				Width = Width,
				Height = Height,
				MipLevels = 1,
				ArraySize = 1,
				Format = SharpDX.DXGI.Format.R8G8B8A8_UNorm,
				SampleDescription = new SharpDX.DXGI.SampleDescription(1, 0),
				Usage = ResourceUsage.Immutable,
				BindFlags = BindFlags.ShaderResource,
				CpuAccessFlags = CpuAccessFlags.None,
				OptionFlags = ResourceOptionFlags.None,
			};
			
			// TODO: mby support formats besides the one above, calculate pitch accordingly
			// https://docs.microsoft.com/en-us/windows/win32/direct3ddds/dx-graphics-dds-pguide
			// var pitch = (Width * 4 + 7) / 8;
			var pitch = Width * 4;
			
			unsafe {
				var tex = new Texture2D(Aetherment.Device, desc, new SharpDX.DataRectangle(data, pitch));
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
		
		public static implicit operator IntPtr(Texture tex) => tex.resource == null ? IntPtr.Zero : tex.resource.NativePointer;
	}
}