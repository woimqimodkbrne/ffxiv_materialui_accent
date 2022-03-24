using System;
using System.Numerics;
using System.Threading.Tasks;
using System.Collections.Generic;
using System.Runtime.InteropServices;

using ImGuiNET;
using Dalamud.Interface;

using Aetherment.Util;

using FFXIVClientStructs.FFXIV.Component.GUI;

namespace Aetherment.GUI {
	internal partial class TextureFinder : IDisposable {
		private bool shouldDraw = false;
		private Vector2 cursorPos;
		private List<IntPtr> nodes;
		private bool locked = false;
		private bool lastheld = false;
		private int selected = 0;
		
		public TextureFinder() {
			nodes = new();
			
			Aetherment.Interface.UiBuilder.Draw += Draw;
		}
		
		public void Dispose() {
			Aetherment.Interface.UiBuilder.Draw -= Draw;
		}
		
		public void Show() {
			shouldDraw = !shouldDraw;
		}
		
		private void Draw() {
			if(!shouldDraw)
				return;
			
			if(ImGui.GetIO().KeyCtrl && ImGui.GetIO().KeyShift) {
				if(!lastheld) {
					locked = !locked;
					lastheld = true;
				}
			} else
				lastheld = false;
			
			if(!locked) {
				selected = 0;
				CheckElements();
			}
			
			ImGui.SetNextWindowSize(new Vector2(500, 400), ImGuiCond.FirstUseEver);
			ImGui.Begin("Texture Finder", ref shouldDraw);
			
			ImGui.Text($"{(locked ? "Locked" : "Unlocked")} (Shift + Ctrl to toggle)");
			
			if(nodes.Count > 0) {
				unsafe {
					for(int i = 0; i < nodes.Count; i++)
						if(i != selected)
							DrawNode((AtkResNode*)nodes[i], 0xFFFF0000);
					
					var node = (AtkImageNode*)nodes[selected];
					var n = ((AtkResNode*)nodes[selected]);
					DrawNode(n, 0xFF00FF00);
					
					ImGui.TextWrapped("UV might not be displayed correctly because the way the game stores it is really fucking dumb.");
					
					if(ImGuiAeth.ButtonIcon(FontAwesomeIcon.ArrowLeft))
						selected = Math.Max(selected - 1, 0);
					ImGui.SameLine();
					ImGui.SetNextItemWidth(ImGui.CalcTextSize($"{nodes.Count - 1}").X + ImGuiAeth.PaddingX * 2);
					if(ImGui.InputInt("##selected", ref selected, 0, 0))
						selected = Math.Clamp(selected, 0, nodes.Count - 1);
					ImGui.SameLine();
					ImGui.Text($"/");
					ImGui.SameLine();
					ImGui.Text($"{nodes.Count - 1}");
					ImGui.SameLine();
					if(ImGuiAeth.ButtonIcon(FontAwesomeIcon.ArrowRight))
						selected = Math.Min(selected + 1, nodes.Count - 1);
					
					try {
						if(n->Type == NodeType.Image) {
							var part = node->PartsList->Parts[node->PartId];
							var type = part.UldAsset->AtkTexture.TextureType;
							var tex = part.UldAsset->AtkTexture;
							var texture = type == TextureType.Resource ?
								tex.Resource->KernelTextureObject :
								tex.KernelTexture;
							
							// AtkTextureResource.Unk_1 seems to be int iconid with 1000000 added if hq
							// TODO: make pr to ffxivclientstructs to rename that
							if(type == TextureType.Resource) {
								var path = PenumbraApi.GetGamePath(tex.Resource->TexFileResourceHandle->ResourceHandle.FileName.ToString());
								ImGui.SameLine();
								if(ImGui.Button(path, new Vector2(ImGuiAeth.WidthLeft(), ImGuiAeth.Height())))
									ImGui.SetClipboardText(path);
								ImGuiAeth.HoverTooltip("Copy to clipboard");
							}
							
							// The preview
							var ratio = Math.Min(ImGuiAeth.WidthLeft() / texture->Width, ImGuiAeth.HeightLeft() / texture->Height);
							var pos = ImGui.GetCursorScreenPos();
							ImGui.Image(new IntPtr(texture->D3D11ShaderResourceView), new Vector2(texture->Width, texture->Height) * ratio);
							
							// this is needed since uv is based on scaled texture (wtf?)
							// TODO: figure out if there a scale variable somewhere idk
							var (u, v) = (0f, 0f);
							for(int partI = 0; partI < node->PartsList->PartCount; partI++) {
								var p = node->PartsList->Parts[partI];
								u = Math.Max(u, p.U + p.Width);
								v = Math.Max(v, p.V + p.Height);
							}
							var ratio2 = Math.Min(texture->Width / u, texture->Height / v);
							
							pos += new Vector2(part.U, part.V) * ratio2 * ratio;
							ImGui.GetWindowDrawList().AddRect(pos, pos + new Vector2(part.Width, part.Height) * ratio2 * ratio, 0xFF00FF00);
						} else {
							var size = new Vector2(ImGuiAeth.WidthLeft(0, 3), ImGuiAeth.HeightLeft(0, 3));
							
							for(uint i = 0; i < Math.Min(node->PartsList->PartCount, 9); i++) {
								if(i % 3 != 0)
									ImGui.SameLine();
								
								ImGui.BeginChild($"{i}9grid", size);
								var part = node->PartsList->Parts[i];
								var type = part.UldAsset->AtkTexture.TextureType;
								var tex = part.UldAsset->AtkTexture;
								var texture = type == TextureType.Resource ?
									tex.Resource->KernelTextureObject :
									tex.KernelTexture;
								
								if(type == TextureType.Resource) {
									var path = PenumbraApi.GetGamePath(tex.Resource->TexFileResourceHandle->ResourceHandle.FileName.ToString());
									if(ImGui.Button(path, new Vector2(ImGuiAeth.WidthLeft(), ImGuiAeth.Height())))
										ImGui.SetClipboardText(path);
									ImGuiAeth.HoverTooltip("Copy to clipboard");
								}
								
								var ratio = Math.Min(ImGuiAeth.WidthLeft() / texture->Width, ImGuiAeth.HeightLeft() / texture->Height);
								var pos = ImGui.GetCursorScreenPos();
								ImGui.Image(new IntPtr(texture->D3D11ShaderResourceView), new Vector2(texture->Width, texture->Height) * ratio);
								ImGui.EndChild();
							}
						}
					} catch {
						ImGui.Text("Invalid element");
					}
				}
			}
			
			ImGui.End();
		}
		
		private unsafe void DrawNode(AtkResNode* node, uint clr) {
			if(node == null)
				return;
			
			var (pos, scale) = GlobalNode(node);
			var width = node->Width * scale.X;
			var height = node->Height * scale.Y;
			
			// TODO: rotation
			ImGui.GetForegroundDrawList().AddRect(pos, pos + new Vector2(width, height), clr);
		}
		
		private void CheckElements() {
			cursorPos = ImGui.GetMousePos();
			
			unsafe {
				nodes.Clear();
				
				var layersAddress = (IntPtr)AtkStage.GetSingleton()->RaptureAtkUnitManager;
				for(var layerI = 12; layerI >= 0; layerI--) {
					var layer = Marshal.PtrToStructure<AtkUnitList>(layersAddress + 0x30 + 0x810 * layerI);
					
					for(var atkI = 0; atkI < layer.Count; atkI++) {
						var atk = (&layer.AtkUnitEntries)[atkI];
						if(atk->IsVisible)
							CheckNodes(atk->UldManager, ref nodes);
					}
				}
				
				// this shouldnt be needed but stuff like icons on maps dont work and idfk why
				nodes.Sort((x, y) => {
					var a = (AtkResNode*)x;
					var b = (AtkResNode*)y;
					
					var scaleA = GlobalNode(a).Item2;
					var scaleB = GlobalNode(b).Item2;
					
					return (a->Width * a->Height * scaleA.X * scaleA.Y).CompareTo(b->Width * b->Height * scaleB.X * scaleB.Y);
				});
			}
		}
		
		private unsafe void CheckNodes(AtkUldManager uld, ref List<IntPtr> elements) {
			for(var nodeI = (int)uld.NodeListCount - 1; nodeI >= 0; nodeI--) {
				var node = uld.NodeList[nodeI];
				if(!GlobalNodeVisible(node))
					continue;
				
				if(node->Type == NodeType.Image || node->Type == NodeType.NineGrid) {
					var imgnode = (AtkImageNode*)node;
					var type = imgnode->PartsList->Parts[imgnode->PartId].UldAsset->AtkTexture.TextureType;
					if(type == TextureType.Resource || type == TextureType.KernelTexture) {
						var (pos, scale) = GlobalNode(node);
						var width = node->Width * scale.X;
						var height = node->Height * scale.Y;
						
						if(cursorPos.X > pos.X && cursorPos.X < pos.X + width && cursorPos.Y > pos.Y && cursorPos.Y < pos.Y + height)
							elements.Add((IntPtr)node);
					}
				} else if((ushort)node->Type >= 1000) {
					CheckNodes(((AtkComponentNode*)node)->Component->UldManager, ref elements);
				}
			}
		}
		
		private unsafe bool GlobalNodeVisible(AtkResNode* node) {
			while(node != null) {
				if(!node->IsVisible)
					return false;
				
				node = node->ParentNode;
			}
			
			return true;
		}
		
		private unsafe (Vector2, Vector2) GlobalNode(AtkResNode* node) {
			var pos = Vector2.Zero;
			var scale = Vector2.One;
			
			while(node != null) {
				var s = new Vector2(node->ScaleX, node->ScaleY);
				scale *= s;
				pos *= s;
				pos += new Vector2(node->X, node->Y);
				
				node = node->ParentNode;
			}
			
			return (pos, scale);
		}
	}
}