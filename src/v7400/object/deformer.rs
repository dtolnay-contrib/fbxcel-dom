//! Objects with `Deformer` class.

pub mod skin;

use crate::v7400::object::{ObjectHandle, ObjectId, ObjectNodeId, ObjectSubtypeHandle};
use crate::v7400::Result;

pub use self::skin::{DeformerSkinHandle, DeformerSkinNodeId};

/// Node ID for a deformer object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeformerNodeId(ObjectNodeId);

/// Object handle for a deformer object.
#[derive(Debug, Clone, Copy)]
pub struct DeformerHandle<'a> {
    /// Object handle.
    object: ObjectHandle<'a>,
}

impl<'a> DeformerHandle<'a> {
    /// Returns the object ID.
    #[inline]
    #[must_use]
    pub fn object_id(&self) -> ObjectId {
        self.object.id()
    }
}

impl<'a> ObjectSubtypeHandle<'a> for DeformerHandle<'a> {
    type NodeId = DeformerNodeId;

    fn from_object(object: &ObjectHandle<'a>) -> Result<Self> {
        let class = object.class();
        if class != "Deformer" {
            return Err(error!(
                "not a model object: expected \"Deformer\" class but got {:?} class",
                class
            ));
        }

        Ok(Self { object: *object })
    }

    #[inline]
    fn as_object(&self) -> &ObjectHandle<'a> {
        &self.object
    }

    #[inline]
    fn node_id(&self) -> Self::NodeId {
        DeformerNodeId(self.object.node_id())
    }
}

impl<'a> AsRef<ObjectHandle<'a>> for DeformerHandle<'a> {
    #[inline]
    fn as_ref(&self) -> &ObjectHandle<'a> {
        self.as_object()
    }
}

/// Subclass of a deformer known to the fbxcel-dom crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DeformerSubclass {
    /// `Skin` subclass.
    Skin,
}
