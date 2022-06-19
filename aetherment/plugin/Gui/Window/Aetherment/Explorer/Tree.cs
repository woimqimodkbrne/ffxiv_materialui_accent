using System;
using System.Collections.Generic;
using System.Linq;
using ImGuiNET;

namespace Aetherment.Gui.Window.Aetherment.Explorer;

public class Tree {
	private class Node {
		public bool isFolder;
		public Node? parent;
		public string? path;
		public SortedDictionary<string, Node> children;
		
		public Node(Node? parent = null, string? path = null, bool isFolder = true) {
			this.isFolder = isFolder;
			this.parent = parent;
			this.path = path;
			children = new();
		}
	}
	
	public string SelectedPath;
	private string name;
	private Node nodes;
	private Action<string> callback;
	
	public Tree(string name, Action<string> callback) {
		SelectedPath = "";
		this.name = name;
		nodes = new();
		this.callback = callback;
	}
	
	public void AddNode(string path) {
		lock(nodes) {
			var segs = path.Split('/');
			var curnode = nodes;
			for(int i = 0; i < segs.Length - 1; i++) {
				var seg = segs[i];
				if(!curnode.children!.TryGetValue(seg, out var node)) {
					node = new(curnode);
					curnode.children.Add(seg, node);
				}
				
				curnode = node;
			}
			curnode.children!.Add(segs.Last(), new(curnode, path, false));
		}
	}
	
	public void Draw() {
		lock(nodes)
			DrawNode(name, nodes);
	}
	
	private void DrawNode(string name, Node node) {
		if(!node.isFolder) {
			if(ImGui.Selectable(name, SelectedPath == node.path)) {
				SelectedPath = node.path!;
				callback(SelectedPath);
			}
			return;
		}
		
		if(ImGui.TreeNode(name)) {
			foreach(var c in node.children)
				DrawNode(c.Key, c.Value);
			ImGui.TreePop();
		}
	}
}