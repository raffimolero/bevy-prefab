use bevy::{
    ecs::system::EntityCommands,
    prelude::{BuildChildren, Bundle, ChildBuilder, Commands},
};

pub mod prelude {
    pub use super::*;
}

// Prefab section
pub trait Prefab {
    fn insert_into<'b, 'w, 's, 'a>(
        self,
        commands: &'b mut EntityCommands<'w, 's, 'a>,
    ) -> &'b mut EntityCommands<'w, 's, 'a>;
}

impl<T: Bundle> Prefab for T {
    fn insert_into<'b, 'w, 's, 'a>(
        self,
        commands: &'b mut EntityCommands<'w, 's, 'a>,
    ) -> &'b mut EntityCommands<'w, 's, 'a> {
        commands.insert(self)
    }
}

// Parent section
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ParentNode<B: Bundle, C: Children> {
    parent: B,
    children: C,
}

impl<B: Bundle, C: Children> Prefab for ParentNode<B, C> {
    fn insert_into<'b, 'w, 's, 'a>(
        self,
        commands: &'b mut EntityCommands<'w, 's, 'a>,
    ) -> &'b mut EntityCommands<'w, 's, 'a> {
        commands.insert(self.parent).with_children(|parent| {
            self.children.attach_to(parent);
        })
    }
}

// Children section
pub trait Children: Sized {
    fn attach_to(self, parent: &mut ChildBuilder);
}

/// C is the previous children, P is the next child.
/// It is in this order because adding a sibling has to wrap the previous children.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SiblingsNode<C: Children, P: Prefab> {
    seniors: C,
    youngest: P,
}

impl<T: Prefab> Children for T {
    fn attach_to(self, parent: &mut ChildBuilder) {
        parent.spawn_prefab(self);
    }
}

impl<C: Children, P: Prefab> Children for SiblingsNode<C, P> {
    fn attach_to(self, parent: &mut ChildBuilder) {
        self.seniors.attach_to(parent);
        parent.spawn_prefab(self.youngest);
    }
}

// Extension traits
pub trait PushChild: Sized {
    type Output<P: Prefab>: Prefab;

    fn child<P: Prefab>(self, child: P) -> Self::Output<P>;
}

impl<T: Bundle> PushChild for T {
    type Output<P: Prefab> = ParentNode<Self, P>;

    fn child<P: Prefab>(self, child: P) -> Self::Output<P> {
        ParentNode {
            parent: self,
            children: child,
        }
    }
}

impl<PPf: Bundle, C: Children> PushChild for ParentNode<PPf, C> {
    type Output<CPf: Prefab> = ParentNode<PPf, SiblingsNode<C, CPf>>;

    fn child<Cp: Prefab>(self, child: Cp) -> Self::Output<Cp> {
        ParentNode {
            parent: self.parent,
            children: SiblingsNode {
                seniors: self.children,
                youngest: child,
            },
        }
    }
}

pub trait EntityCommandsExt {
    fn insert_prefab<P: Prefab>(&mut self, prefab: P) -> &mut Self;
}

impl<'w, 's, 'a> EntityCommandsExt for EntityCommands<'w, 's, 'a> {
    fn insert_prefab<P: Prefab>(&mut self, prefab: P) -> &mut Self {
        prefab.insert_into(self)
    }
}

pub trait CommandsExt<'w, 's> {
    fn spawn_prefab<'a, P: Prefab>(&'a mut self, prefab: P) -> EntityCommands<'w, 's, 'a>;
}

impl<'w, 's> CommandsExt<'w, 's> for Commands<'w, 's> {
    fn spawn_prefab<'a, P: Prefab>(&'a mut self, prefab: P) -> EntityCommands<'w, 's, 'a> {
        let mut out = self.spawn_empty();
        out.insert_prefab(prefab);
        out
    }
}

impl<'w, 's, 'b> CommandsExt<'w, 's> for ChildBuilder<'w, 's, 'b> {
    fn spawn_prefab<'a, P: Prefab>(&'a mut self, prefab: P) -> EntityCommands<'w, 's, 'a> {
        let mut out = self.spawn_empty();
        out.insert_prefab(prefab);
        out
    }
}
