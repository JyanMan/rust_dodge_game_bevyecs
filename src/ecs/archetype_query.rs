use crate::ecs::entity::*;
use crate::ecs::ecs::*;
use crate::ecs::archetype::*;
use crate::ecs::sparse_set::*;
use std::any::*;
use paste::*;

macro_rules! query_impl {
    ($($($mut:ident)? ($ty: ident) ), + $(,)?) => {
        paste! {
            #[allow(non_snake_case)]
            impl <'a, $($ty), +> Query <'a> for ($(&$($mut)? $ty), +) 
            where
                $($ty: 'static + Clone),+
            {
                type Output = Vec<(Entity, $(&'a $($mut)? $ty), +)>;

                fn fetch(ecs: &'a ArchetypeManager) -> Self::Output {
                    // init all type id
                    $(let [<id_$ty>] = TypeId::of::<$ty>();) +
                    // init all corresponding signs of id
                    $(let [<sign_$ty>] = ecs.signatures[&[<id_$ty>]];) +
                    // combine all signs to one sign
                    let sign_to_query = $([<sign_$ty>])|+;
                    // init empty components vec
                    let mut components: Vec<(Entity, $(&$($mut)? $ty), +)> = vec![];

                    for (arch_id, arch) in ecs.archetype_map.iter() {
                        // check only for id that has atleast sign to query signature
                        if arch_id & sign_to_query == sign_to_query {
                            $(let [<col_$ty>] = ecs.component_index[&[<id_$ty>]][&arch.id].column;) +

                            unsafe {
                                // get the sparseset 
                                $(let [<set_$ty>] = arch.components[[<col_$ty>]]
                                    .get()
                                    .as_mut()
                                    .unwrap()
                                    .as_any_mut()
                                    .downcast_mut::<SparseSet<$ty>>()
                                    .unwrap();)+

                                let dense_list_a = set_A.get_dense_list();
                                let mut i: usize = 0;
                                for e in dense_list_a.iter() {
                                    $(let [<comp_$ty>] = [<set_$ty>].[<get_by_index$(_$mut)?>](i).unwrap();) +
                                    components.push((*e, $([<comp_$ty>]), +));
                                    i += 1;
                                }
                            }
                        }
                    }
                    components    
                }
            }
        }
    }
}

macro_rules! query_impls {
    // Entry point
    ($($ty:ident),*) => {
        query_impls_inner!((); $($ty),*);
    };
}

macro_rules! query_impls_inner {
    // Base case: no more types to process
    (($($args:tt)*);) => {
        query_impl!($($args)*);
    };

    // Recursive case: add non-mut and mut variants for the head
    (($($args:tt)*); $head:ident $(, $tail:ident)*) => {
        query_impls_inner!(($($args)* ($head)); $($tail),*);
        query_impls_inner!(($($args)* mut ($head)); $($tail),*);
    };
}


pub trait Query <'a> {
    type Output;
    fn fetch(ecs: &'a ArchetypeManager) -> Self::Output;
}

query_impl!((A));
query_impl!(mut (A));

query_impl!((A), (B));
query_impl!(mut (A), (B));
query_impl!((A), mut (B));
query_impl!(mut (A), mut (B));

query_impl!((A), (B), (C));
query_impl!(mut (A), (B), (C));
query_impl!((A), mut (B), (C));
query_impl!((A), (B), mut (C));
query_impl!((A), mut (B), mut (C));
query_impl!(mut (A), mut (B), (C));
query_impl!(mut (A), mut (B), mut (C));

query_impl!((A), (B), (C), (D));
query_impl!(mut (A), (B), (C), (D));
query_impl!((A), mut (B), (C), (D));
query_impl!((A), (B), mut (C), (D));
query_impl!((A), (B), (C), mut (D));
query_impl!(mut (A), mut (B), (C), (D));
query_impl!(mut (A), (B), mut (C), (D));
query_impl!(mut (A), (B), (C), mut (D));
query_impl!((A), mut (B), mut (C), (D));
query_impl!((A), mut (B), (C), mut (D));
query_impl!((A), (B), mut (C), mut (D));
query_impl!(mut (A), mut (B), mut (C), (D));
query_impl!(mut (A), mut (B), mut (C), mut (D));


query_impl!((A), (B), (C), (D), (E));
query_impl!(mut (A), (B), (C), (D), (E));
query_impl!((A), mut (B), (C), (D), (E));
query_impl!((A), (B), mut (C), (D), (E));
query_impl!((A), (B), (C), mut (D), (E));
query_impl!((A), (B), (C), (D), mut (E));
query_impl!(mut (A), mut (B), (C), (D), (E));
query_impl!(mut (A), (B), mut (C), (D), (E));
query_impl!(mut (A), (B), (C), mut (D), (E));
query_impl!(mut (A), (B), (C), (D), mut (E));
query_impl!((A), mut (B), mut (C), (D), (E));
query_impl!((A), mut (B), (C), mut (D), (E));
query_impl!((A), mut (B), (C), (D), mut (E));
query_impl!((A), (B), mut (C), mut (D), (E));
query_impl!((A), (B), mut (C), (D), mut (E));
query_impl!((A), (B), (C), mut (D), mut (E));
query_impl!(mut (A), mut (B), mut (C), (D), (E));
query_impl!(mut (A), mut (B), (C), mut (D), (E));
query_impl!(mut (A), mut (B), (C), (D), mut (E));
query_impl!(mut (A), (B), mut (C), mut (D), (E));
query_impl!(mut (A), (B), mut (C), (D), mut (E));
query_impl!(mut (A), (B), (C), mut (D), mut (E));
query_impl!((A), mut (B), mut (C), mut (D), (E));
query_impl!((A), mut (B), mut (C), (D), mut (E));
query_impl!((A), mut (B), (C), mut (D), mut (E));
query_impl!((A), (B), mut (C), mut (D), mut (E));
query_impl!(mut (A), mut (B), mut (C), mut (D), (E));
query_impl!(mut (A), mut (B), mut (C), (D), mut (E));
query_impl!(mut (A), mut (B), (C), mut (D), mut (E));
query_impl!(mut (A), (B), mut (C), mut (D), mut (E));
query_impl!((A), mut (B), mut (C), mut (D), mut (E));
query_impl!(mut (A), mut (B), mut (C), mut (D), mut (E));

//impl <'a, A, B> Query <'a> for (&A, &B) 
//where
//    A: 'static + Clone,
//    B: 'static + Clone
//{
//    type Output = Vec<(&'a A, &'a B)>;
//
//    fn fetch(ecs: &'a ArchetypeManager) -> Self::Output {
//        let a_id = TypeId::of::<A>();
//        let b_id = TypeId::of::<B>();
//
//        let a_sign = ecs.signatures[&a_id];
//        let b_sign = ecs.signatures[&b_id];
//        let sign_to_query = a_sign | b_sign;
//        // let arch_map = self.component_index.get(&type_id).expect("component not registered");
//        let mut components: Vec<(&A, &B)> = vec![];
//
//        for (arch_id, arch) in ecs.archetype_map.iter() {
//            if arch_id & sign_to_query == sign_to_query {
//                //archetype included
//                // get columns
//                let a_col = ecs.component_index[&a_id][&arch.id].column;
//                // let b_col = self.component_index[&b_id][&arch.id].column;
//                let b_col = ecs.component_index[&b_id][&arch.id].column;
//
//                unsafe {
//                    // get the sparseset 
//                    let a_set = arch.components[a_col]
//                        .get()
//                        .as_mut()
//                        .unwrap()
//                        .as_any_mut()
//                        .downcast_mut::<SparseSet<A>>()
//                        .unwrap();
//                    let b_set = arch.components[b_col]
//                        .get()
//                        .as_mut()
//                        .unwrap()
//                        .as_any_mut()
//                        .downcast_mut::<SparseSet<B>>()
//                        .unwrap();
//
//                    for i in 0..a_set.len() {
//                        let comp_a = a_set.get_by_index(i).unwrap();
//                        let comp_b = b_set.get_by_index(i).unwrap();
//                        components.push((comp_a, comp_b));
//                    }
//                }
//            }
//        }
//        components    
//    }
//}

impl ArchetypeManager {
    pub fn query_comp<'a, Q: Query <'a>>(&'a self) -> Q::Output {
        Q::fetch(self)
    }
}

impl ECS {
    pub fn query_comp<'a, Q: Query <'a>>(&'a self) -> Q::Output {
        self.archetype_m.query_comp::<Q>()
    }
}

