using System;
using System.Collections.Generic;
using System.Linq;
using ImGuiNET;

namespace Aetherment.Gui.Window.Aetherment.Explorer;

public class Tree {
	private class Node {
		public bool isFolder;
		public bool enabled;
		public Node? parent;
		public string? path;
		public SortedDictionary<string, Node> children;
		
		public Node(Node? parent = null, string? path = null, bool isFolder = true) {
			this.isFolder = isFolder;
			enabled = true;
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
	
	public void SetNodeState(bool state) {
		void setstate(Node node) {
			node.enabled = state;
			foreach(var n in node.children.Values)
				setstate(n);
		}
		
		foreach(var n in nodes.children.Values)
			setstate(n);
	}
	
	public void SetNodeState(string path, bool state) {
		var node = nodes;
		foreach(var seg in path.Split('/'))
			node = node.children[seg];
		SetNodeState(node, state);
	}
	
	private void SetNodeState(Node node, bool state) {
		if(node == nodes || node.enabled == state)
			return;
		
		node.enabled = state;
		if(node.parent!.children.Any(n => n.Value.enabled) != node.parent!.enabled)
			SetNodeState(node.parent, state);
	}
	
	public void Draw() {
		lock(nodes)
			DrawNode(name, nodes);
	}
	
	private void DrawNode(string name, Node node) {
		if(!node.isFolder) {
			if(!node.enabled)
				ImGui.PushStyleColor(ImGuiCol.Text, ImGui.GetColorU32(ImGuiCol.TextDisabled));
			if(ImGui.Selectable(name, SelectedPath == node.path)) {
				SelectedPath = node.path!;
				callback(SelectedPath);
			}
			if(!node.enabled)
				ImGui.PopStyleColor();
			return;
		}
		
		if(!node.enabled)
			ImGui.PushStyleColor(ImGuiCol.Text, ImGui.GetColorU32(ImGuiCol.TextDisabled));
		var treeOpen = ImGui.TreeNode(name);
		if(!node.enabled)
				ImGui.PopStyleColor();
		
		if(treeOpen) {
			foreach(var c in node.children)
				if(!c.Value.isFolder)
					DrawNode(c.Key, c.Value);
			foreach(var c in node.children)
				if(c.Value.isFolder)
					DrawNode(c.Key, c.Value);
			ImGui.TreePop();
		}
	}
}