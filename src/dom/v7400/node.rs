//! DOM node.

use indextree;

use crate::dom::v7400::{Core, Document, StrSym};
use crate::pull_parser::v7400::attribute::DirectAttributeValue;

/// A trait for types convertible into `indextree::NodeId`.
///
/// This should be crate-local (should not exposed to crate users), so this is
/// not implemented using `Into` trait.
pub(crate) trait IntoRawNodeId: Copy + std::fmt::Debug {
    /// Returns raw node ID.
    fn raw_node_id(self) -> indextree::NodeId;
}

impl IntoRawNodeId for indextree::NodeId {
    fn raw_node_id(self) -> indextree::NodeId {
        self
    }
}

impl<T: Into<NodeId> + Copy + std::fmt::Debug> IntoRawNodeId for T {
    fn raw_node_id(self) -> indextree::NodeId {
        self.into().0
    }
}

/// A trait for types which might be convertible from other ID types.
pub trait DowncastId<T>: Copy {
    /// Returns an ID corresponding to the `self`.
    fn downcast(self, doc: &Document) -> Option<T>;
}

/// A trait for ID types.
pub(crate) trait ValidateId: Copy {
    /// Validates the ID value is valid in the given document.
    ///
    /// Note that the given `self` may be invalid.
    fn validate_id(self, doc: &Document) -> bool;
}

/// FBX tree node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(indextree::NodeId);

impl NodeId {
    /// Creates a new `NodeId`.
    pub(crate) fn new(id: indextree::NodeId) -> Self {
        NodeId(id)
    }

    /// Returns the node from the node ID.
    ///
    /// # Panics
    ///
    /// Panics if the node with the id does not exist in the given document.
    pub fn node(self, core: &impl AsRef<Core>) -> Node<'_> {
        core.as_ref().node(self)
    }

    /// Returns an iterator of childrens with the given name.
    pub fn children_by_name<'a>(
        self,
        core: &'a impl AsRef<Core>,
        name: &str,
    ) -> impl Iterator<Item = NodeId> + 'a {
        core.as_ref().children_by_name(self, name)
    }

    /// Returns the node ID of first found node with the given path.
    pub fn first_node_by_path(self, core: impl AsRef<Core>, path: &[&str]) -> Option<Self> {
        self.first_node_by_path_impl(core.as_ref(), path)
    }

    /// Returns the node ID of first found node with the given path.
    fn first_node_by_path_impl(self, core: &Core, path: &[&str]) -> Option<Self> {
        let mut current = self;
        for component in path {
            current = core.children_by_name(current, component).next()?;
        }
        Some(current)
    }
}

/// Node data (including related node ID info).
#[derive(Debug, Clone, PartialEq)]
pub struct Node<'a> {
    /// Node.
    node: &'a indextree::Node<NodeData>,
}

impl<'a> Node<'a> {
    /// Creates a new `Node`.
    pub(crate) fn new(node: &'a indextree::Node<NodeData>) -> Self {
        Self { node }
    }

    /// Returns the node data.
    pub(crate) fn data(&self) -> &'a NodeData {
        &self.node.data
    }

    /// Returns the node name.
    ///
    /// # Panics
    ///
    /// Panics if the node is not in the given document.
    pub fn name(&self, core: &'a impl AsRef<Core>) -> &'a str {
        core.as_ref()
            .string(self.data().name)
            .expect("The node is not registered in the document")
    }

    /// Returns the node attributes.
    pub fn attributes(&self) -> &'a [DirectAttributeValue] {
        &self.data().attributes
    }

    /// Returns the node ID of the parent node.
    pub fn parent(&self) -> Option<NodeId> {
        self.node.parent().map(NodeId::new)
    }

    /// Returns the node ID of the first child node.
    pub fn first_child(&self) -> Option<NodeId> {
        self.node.first_child().map(NodeId::new)
    }

    /// Returns the node ID of the last child node.
    pub fn last_child(&self) -> Option<NodeId> {
        self.node.last_child().map(NodeId::new)
    }

    /// Returns the node ID of the previous sibling node.
    pub fn previous_sibling(&self) -> Option<NodeId> {
        self.node.previous_sibling().map(NodeId::new)
    }

    /// Returns the node ID of the next sibling node.
    pub fn next_sibling(&self) -> Option<NodeId> {
        self.node.next_sibling().map(NodeId::new)
    }
}

/// Pure node data (without connections between related nodes).
#[derive(Debug, Clone, PartialEq)]
pub struct NodeData {
    /// Node name.
    name: StrSym,
    /// Node attributes.
    attributes: Vec<DirectAttributeValue>,
}

impl NodeData {
    /// Creates a new `NodeData`.
    pub(crate) fn new(name: StrSym, attributes: Vec<DirectAttributeValue>) -> Self {
        Self { name, attributes }
    }

    /// Returns node name symbol.
    pub(crate) fn name_sym(&self) -> StrSym {
        self.name
    }

    /// Returns node attributes.
    pub(crate) fn attributes(&self) -> &[DirectAttributeValue] {
        &self.attributes
    }
}
