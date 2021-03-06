
extern crate slab;
extern crate atom;
extern crate fnv;

pub mod world;
pub mod component;

// use std::rc::{Rc, Weak};
// use std::cell::RefCell;
// use component::{EventType, Point, ComponentGroupTree, ComponentGroup};
// use world::{ID, ComponentMgr};

// #[macro_export]
// macro_rules! point{
//     {$name: ident{
//         $(#[component] $field_name_c:ident : $field_type_c:ty,)*
//         $($field_name:ident : $field_type:ty,)*
//     }} => {
//         $crate::paste::item! {
//             #[derive(Clone, Default)]
//             pub struct [<$name Point>](usize);

//             pub struct [<$name Ref>]<M: ComponentMgr>{
//                 point: [<$name Point>],
//                 groups: Rc<RefCell<[<$name Groups>]<M>>>,
//             }
//         }

//         impl_point!{$name{
//             $(#[component] $field_name_c : $field_type_c,)*
//             $($field_name: $field_type,)*
//         }}

//         impl_point_tarit!($name);

//         // def_component_meta!{
//         //     $name{
//         //         $($field_name_c : $field_type_c,)*
//         //         $($field_name : $field_type,)*
//         //     }
//         // }
//     };
// }

// #[macro_export]
// macro_rules! impl_point{
//     ($name: ident{
//         $(#[component] $field_name_c:ident : $field_type_c:ty,)*
//         $($field_name:ident : $field_type:ty,)*
//     }) => {
//         $crate::paste::item! {
//             impl [<$name Point>]{
//                 $(
//                     pub fn [<set_$field_name>]<M: ComponentMgr>(&mut self, value: $field_type, groups: &mut [<$name Groups>]<M>){
//                         groups._group.get_mut(self).$field_name = value;
//                         groups._group.notify(EventType::ModifyField(self.clone(), "$field_name"));
//                     }

//                     pub fn [<get_$field_name>]<'a, M: ComponentMgr>(&mut self, groups: &'a [<$name Groups>]<M>) -> &'a $field_type{
//                         &(groups._group.get(self).$field_name)
//                     }
//                 )*
//                 $(
//                     pub fn [<set_$field_name_c>]<M: ComponentMgr>(&mut self, value: $field_type_c, groups: &mut [<$name Groups>]<M>){
//                         let index = groups.$field_name_c.borrow_mut()._group.insert(value, self.0.clone());
//                         groups._group.get_mut(self).$field_name_c = index;
//                         groups._group.notify(EventType::ModifyField(self.clone(), "$field_name"));
//                     }

//                     pub fn [<get_$field_name_c>]<M: ComponentMgr>(&mut self, groups: &[<$name Groups>]<M>) -> [<$field_type_c Point>]{
//                         groups._group.get(self).$field_name_c.clone()
//                     }
//                 )*
//             }

//             impl<M: ComponentMgr> [<$name Ref>]<M>{
//                 $(
//                     pub fn [<set_$field_name>](&mut self, value: $field_type){
//                         self.point.[<set_$field_name>](value, &mut self.groups.borrow_mut());
//                     }

//                     pub fn [<get_$field_name>](&mut self) -> &$field_type{
//                         unsafe{&*(self.point.[<get_$field_name>](&self.groups.borrow()) as *const$field_type)}
//                     }
//                 )*
//                 $(
//                     pub fn [<set_$field_name_c>](& mut self, value: $field_type_c){
//                         self.point.[<set_$field_name_c>](value, &mut self.groups.borrow_mut());
//                     }

//                     pub fn [<get_$field_name_c>](&mut self) -> [<$field_type_c Ref>]<M>{
//                         let p = self.point.[<get_$field_name_c>](&self.groups.borrow()).clone();
//                         [<$field_type_c Ref>]::new(p, self.groups.borrow_mut().$field_name_c.clone())
//                     }
//                 )*

//                 pub fn new(p: [<$name Point>], g: Rc<RefCell<[<$name Groups>]<M>>>) -> [<$name Ref>]<M>{
//                     [<$name Ref>]{
//                         point: p,
//                         groups: g,
//                     }
//                 }
//             }
//         }
//     };
//     // ($name: ident($($field_type: ty), *)) => {
//     //     $crate::paste::item! {
//     //         impl [<$name Point>]{
//     //             $(pub fn [<set_$field_name>](&mut self, value: $field_type){
//     //                 let group = self.group.upgrade().unwrap();
//     //                 group.get_mut(self.id()).$field_name = value;
//     //                 group.notify_moitor(EventType::ModifyField(self.clone(), "$field_name"));
//     //             }

//     //             pub fn [<get_$field_name>](&mut self) -> &$field_type{
//     //                 unsafe {&*(&self.group.upgrade().unwrap().get(self.id()).$field_name as *const $field_type)}
//     //             })*
//     //         }
//     //     }
//     // };
// }

// #[macro_export]
// macro_rules! impl_point_tarit{
//     ($name: ident) => (
//         $crate::paste::item! {
//             impl ID for [<$name Point>]{
//                 fn id(& self) -> usize{
//                     self.0
//                 }
//                 fn set_id(&mut self, id: usize){
//                     self.0 = id;
//                 }
//             }

//             impl Point for [<$name Point>]{
//             }
            
//         }
//     );
// }

// #[macro_export]
// macro_rules! def_component{
//     {
//         $(#[derive($($derive:ident),*)])*
//         $name: ident{
//             $(#[component] $field_name_c:ident : $field_type_c:ty,)*
//             $($field_name:ident : $field_type:ty,)*
//         }
//     }=> {
//         $crate::paste::item! {
//             $(#[derive($($derive),*)])*
//             pub struct $name{
//                 $(pub $field_name_c: [<$field_type_c Point>],)*
//                 $(pub $field_name: $field_type,)*
//             }
//         }
//     };
// }

// // #[macro_export]
// // macro_rules! def_component_meta{
// //     {
// //         $name: ident{
// //             $($field_name:ident : $field_type:ty,)*
// //         }
// //     }=> {
// //         $crate::paste::item! {
// //             pub struct [<$name Meta>]{
// //             }

// //             lazy_static! {
// //                 pub static ref [<$name META>]: [<$name Meta>] = [<$name Meta>]{};
// //             }
// //         }
// //     };
// // }

// #[macro_export]
// macro_rules! impl_component{
//     ($name:ident)=> {
//     $crate::paste::item! {
//         // impl Component for $name{
//         //     // type Meta = [<$name Meta>];
//         //     // fn meta() -> &'static Self::Meta{
//         //     //     &[<$name META>]
//         //     // }
//         // }
//     }
//     }
// }

// #[macro_export]
// macro_rules! component1{
//     (
//         $(#[derive($($derive: ident),*)])*
//         $name: ident{
//             $(#[component] $field_name_c:ident : $field_type_c:ty,)*
//             $($field_name:ident : $field_type:ty,)*
//         }
//     ) => {
//         def_component!(
//             $(#[derive($($derive)*)])*
//             $name{
//                 $(#[component] $field_name_c: $field_type_c,)*
//                 $($field_name: $field_type,)*
//             }
//         );
//         impl_component!($name);
//     }
// }

// #[macro_export]
// macro_rules! component_group_tree{
//     (
//         $component: ident{
//             $($feild_name:ident : $feild_type:ty,)*
//         }
//     ) => {
//         $crate::paste::item! {
//             pub struct [<$component Groups>]<M: ComponentMgr>{
//                 pub _group: ComponentGroup<$component, [<$component Point>], M>,
//                 $(pub $feild_name: Rc<RefCell<[<$feild_type Groups>]<M>>>),*
//             }

//             impl<M: ComponentMgr> ComponentGroupTree for [<$component Groups>]<M>{
//                 type C = M;
//                 fn new () -> [<$component Groups>]<M>{
//                     [<$component Groups>]{
//                         $($feild_name: Rc::new(RefCell::new([<$feild_type Groups>]::new())),)*
//                         _group: ComponentGroup::new(),
//                     }
//                 }

//                 fn set_mgr(&mut self, mgr: Weak<RefCell<Self::C>>){
//                     $(self.$feild_name.borrow_mut().set_mgr(mgr.clone());)*
//                     self._group.set_mgr(mgr);
//                 }
//             }
//         }
//     };

//     {
//         #[root]
//         $component: ident{
//             $($feild_name:ident : $feild_type:ty,)*
//         }
//     } => {
//         $crate::paste::item! {
//             component_group_tree!{
//                 $component: ident{
//                     $($feild_name:ident : $feild_type:ty,)*
//                 }
//             }

//             pub struct [<$component Mgr>]{
//                 pub group: ComponentGroup<$feild_type, [<$component $feild_type Point>]>,
//                 pub childs: [<$component $feild_type Group Tree>]
//             }
//         }
//     };
// }

// #[macro_export]
// macro_rules! world{
//     {$Mgr: ident{
//         $($root_name: ident: $root_type: ident),*}
//     } => {
//         $crate::paste::item! {
//             pub struct $Mgr{
//                 $(pub $root_name: Rc<RefCell<[<$root_type Groups>]<$Mgr>>>,)*
//             }

//             impl ComponentMgr for $Mgr{
//                 fn new() -> Rc<RefCell<Self>>{
//                     let m = Rc::new(RefCell::new($Mgr{
//                         $($root_name: Rc::new(RefCell::new([<$root_type Groups>]::new())),)*
//                     }));
//                     let m_weak = Rc::downgrade(&m);
//                     {
//                         let m_borrow = m.borrow();
//                         $(m_borrow.$root_name.borrow_mut().set_mgr(m_weak.clone());)*
//                     }
//                     m
//                 }
//             }

//             impl $Mgr {
//                 $(
//                     pub fn [<add_$root_name>](&mut self, $root_name: $root_type) -> [<$root_type Ref>]<$Mgr>{
//                         let point = self.$root_name.borrow_mut()._group.insert($root_name, 0);
//                         [<$root_type Ref>]::new(point, self.$root_name.clone())
//                     }
//                 )*
//             }
//         }
//     };
// }

// #[macro_export]
// macro_rules! out_component{
//     {$component_o: ident{
//         $($c_f_n_o:ident : $c_f_t_o:ty,)*
//     }} => (
//         point!{$component_o{
//             $($c_f_n_o : $c_f_t_o,)*
//         }}
//         impl_component!($component_o);
//         component_group_tree!(
//             $component_o{}
//         );
//     )
// }

// #[macro_export]
// macro_rules! component{
//     {
//         $(#[derive($($derive_r: ident),*)])*
//         $component_root: ident{
//             $(#[component]$c_r_f_n:ident : $c_r_f_t:ty,)*
//             $($c_r_f_n1:ident : $c_r_f_t1:ty,)*
//         }
//     } => {
//         point!{$component_root{
//             $(#[component] $c_r_f_n : $c_r_f_t,)*
//             $($c_r_f_n1 : $c_r_f_t1,)*
//         }}
//         component_group_tree!(
//             $component_root{
//                 $($c_r_f_n : $c_r_f_t,)*
//             }
//         );
//         $crate::paste::item! {
//             component1!{
//                 $(#[derive($($derive_r),*)])*
//                 $component_root{
//                     $(#[component] $c_r_f_n : $c_r_f_t,)*
//                     $($c_r_f_n1 : $c_r_f_t1,)*
//                 }
//             }
//         }
//     };

//     {
//         $(#[derive($($derive: ident),*)])*
//         $component: ident{
//             $(#[component] $c_f_n:ident : $c_f_t:ty,)*
//             $($c_f_n1:ident : $c_f_t1:ty,)*
//         }
//     } => {
//         point!{$component{
//             $(#[component] $c_f_n : $c_f_t,)*
//             $($c_f_n1 : $c_f_t1,)*
//         }}
//         component_group_tree!(
//             $component{
//                 $($c_f_n : $c_f_t,)*
//             }
//         );
//         $crate::paste::item! {
//             component1!{
//                 $(#[derive($($derive),*)])*
//                 $component{
//                     $(#[component] $c_f_n : $c_f_t,)*
//                     $($c_f_n1 : $c_f_t1,)*
//                 }
//             }
//         }
//     };
// }



// // component!(Node{p: PositionPoint});
// // component!(Position{x: u32, y: u32}, Node);
// // //component!(Z(Position, Position));
// // // component!(Z(Position, Position), Node);
// // component_mgr!(ComponentMgr1{position_group: Position, node_group: Node});

// // component!(Node{
// //     //object: Object,
// //     z_index: usize,
// //     z: usize,
// //     layer: usize,
// //     visibility: bool
// //     //renderer: Renderer,
// //     //layout: Layout
// // }, Node, Rc);

// // pub struct AA {
// //     pub left: usize,
// //     pub right: usize,
// //     pub top: usize,
// //     pub bottom: usize,
// // }

// // world!(
// //     GuiComponentMgr{
// //         #[outtype]
// //         AA{
// //             left: usize,
// //             right: usize,
// //             top: usize,
// //             bottom: usize,
// //         }

// //         #[derive(Default)]
// //         Layout{
// //             left: usize,
// //             right: usize,
// //             top: usize,
// //             bottom: usize,
// //         }

// //         #[root=node]
// //         #[derive(Default)]
// //         Node{
// //             #[component]
// //             layout: Layout,
// //             //node_type: NodeType,
// //             // z_index: usize,
// //             // z: usize,
// //             layer: usize,
// //             // visibility: bool,
// //             layout_dirt: bool,
// //             childs: Vec<NodePoint>,
// //             lay_out:NodePoint,
// //             //renderer: Renderer,
// //         }
// //     }, ()
// // );

// pub struct Vector2 {
//     pub x: f32,
//     pub y: f32,
// }

// world!{
//     GuiComponentMgr1{
//         node: Node
//     }
// }

// out_component!{Vector2{
//     x: f32,
//     y:f32,
// }}

// component!{
//     Node{
//         #[component]
//         bound_box: Vector2,
//         // node_type: NodeType,
//         // z_index: usize,
//         // z: usize,
//         // layer: usize,
//         // visibility: bool,
//         // layout_dirt: bool,
//         // childs: Vec<NodePoint>,
//         // position: Vector2<f32>,
//         // layout_node: YgNode,
//         //renderer: Renderer,
//     }
// }
     
        

