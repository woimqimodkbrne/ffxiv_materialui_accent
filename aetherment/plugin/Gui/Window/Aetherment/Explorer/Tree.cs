using System;
using System.Collections.Generic;
using System.Linq;
using ImGuiNET;

namespace Aetherment.Gui.Window.Aetherment.Explorer;

public class Tree {
	private class Node {
		public bool isFolder;
		public Node? parent;
		public SortedDictionary<string, Node> children;
		
		public Node(Node? parent = null, bool isFolder = true) {
			this.isFolder = isFolder;
			this.parent = parent;
			children = new();
		}
	}
	
	private string name;
	private Node nodes;
	private Node? selectedNode;
	private Action<string> callback;
	
	public Tree(string name, Action<string> callback) {
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
			curnode.children!.Add(segs.Last(), new(curnode, false));
		}
	}
	
	public void Draw() {
		lock(nodes)
			DrawNode(name, nodes);
	}
	
	private void DrawNode(string name, Node node) {
		if(!node.isFolder) {
			if(ImGui.Selectable(name, selectedNode == node)) {
				var path = name;
				var curnode = node.parent;
				while(curnode!.parent != null) {
					path = $"{curnode.parent.children.First(x => x.Value == curnode).Key}/{path}";
					curnode = curnode.parent;
				}
				selectedNode = node;
				callback(path);
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