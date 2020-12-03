#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use core::convert::{TryInto, TryFrom};
use sp_core::U256;
use sp_std::{prelude::*, if_std, fmt::Debug, result, ops::Not};
use sp_runtime::{
    traits::{Bounded, Member, Zero, CheckedSub, Hash, AtLeast32Bit},
};
use frame_support::{
    decl_module, decl_storage, decl_event, decl_error, StorageValue, StorageMap, ensure, Parameter,
    dispatch,
    traits::{Get, Randomness},
    weights::{Weight},
};
use frame_system::ensure_signed;
use codec::{Encode, Decode};
use byteorder::{ByteOrder, LittleEndian};
mod types {
    use codec::{Decode, Encode, EncodeLike};
    use sp_std::prelude::*;
    use sp_runtime::{
        DispatchResult as Result,
        traits::{Bounded, Member, AtLeast32Bit},
    };
    use frame_support::{ensure, Parameter, StorageMap};
    pub use crate as trade;
    type OrderType = trade::OrderType;
    pub struct LinkedItem<K1, K2, K3> {
        pub prev: Option<K2>,
        pub next: Option<K2>,
        pub price: Option<K2>,
        pub buy_amount: K3,
        pub sell_amount: K3,
        pub orders: Vec<K1>,
    }
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<K1, K2, K3> _parity_scale_codec::Encode for LinkedItem<K1, K2, K3>
        where
            Option<K2>: _parity_scale_codec::Encode,
            Option<K2>: _parity_scale_codec::Encode,
            Option<K2>: _parity_scale_codec::Encode,
            Option<K2>: _parity_scale_codec::Encode,
            Option<K2>: _parity_scale_codec::Encode,
            Option<K2>: _parity_scale_codec::Encode,
            K3: _parity_scale_codec::Encode,
            K3: _parity_scale_codec::Encode,
            K3: _parity_scale_codec::Encode,
            K3: _parity_scale_codec::Encode,
            Vec<K1>: _parity_scale_codec::Encode,
            Vec<K1>: _parity_scale_codec::Encode,
        {
            fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
                dest.push(&self.prev);
                dest.push(&self.next);
                dest.push(&self.price);
                dest.push(&self.buy_amount);
                dest.push(&self.sell_amount);
                dest.push(&self.orders);
            }
        }
        impl<K1, K2, K3> _parity_scale_codec::EncodeLike for LinkedItem<K1, K2, K3>
        where
            Option<K2>: _parity_scale_codec::Encode,
            Option<K2>: _parity_scale_codec::Encode,
            Option<K2>: _parity_scale_codec::Encode,
            Option<K2>: _parity_scale_codec::Encode,
            Option<K2>: _parity_scale_codec::Encode,
            Option<K2>: _parity_scale_codec::Encode,
            K3: _parity_scale_codec::Encode,
            K3: _parity_scale_codec::Encode,
            K3: _parity_scale_codec::Encode,
            K3: _parity_scale_codec::Encode,
            Vec<K1>: _parity_scale_codec::Encode,
            Vec<K1>: _parity_scale_codec::Encode,
        {
        }
    };
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<K1, K2, K3> _parity_scale_codec::Decode for LinkedItem<K1, K2, K3>
        where
            Option<K2>: _parity_scale_codec::Decode,
            Option<K2>: _parity_scale_codec::Decode,
            Option<K2>: _parity_scale_codec::Decode,
            Option<K2>: _parity_scale_codec::Decode,
            Option<K2>: _parity_scale_codec::Decode,
            Option<K2>: _parity_scale_codec::Decode,
            K3: _parity_scale_codec::Decode,
            K3: _parity_scale_codec::Decode,
            K3: _parity_scale_codec::Decode,
            K3: _parity_scale_codec::Decode,
            Vec<K1>: _parity_scale_codec::Decode,
            Vec<K1>: _parity_scale_codec::Decode,
        {
            fn decode<DecIn: _parity_scale_codec::Input>(
                input: &mut DecIn,
            ) -> core::result::Result<Self, _parity_scale_codec::Error> {
                Ok(LinkedItem {
                    prev: {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => return Err("Error decoding field LinkedItem.prev".into()),
                            Ok(a) => a,
                        }
                    },
                    next: {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => return Err("Error decoding field LinkedItem.next".into()),
                            Ok(a) => a,
                        }
                    },
                    price: {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => return Err("Error decoding field LinkedItem.price".into()),
                            Ok(a) => a,
                        }
                    },
                    buy_amount: {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field LinkedItem.buy_amount".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    sell_amount: {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field LinkedItem.sell_amount".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    orders: {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => return Err("Error decoding field LinkedItem.orders".into()),
                            Ok(a) => a,
                        }
                    },
                })
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<K1: ::core::clone::Clone, K2: ::core::clone::Clone, K3: ::core::clone::Clone>
        ::core::clone::Clone for LinkedItem<K1, K2, K3>
    {
        #[inline]
        fn clone(&self) -> LinkedItem<K1, K2, K3> {
            match *self {
                LinkedItem {
                    prev: ref __self_0_0,
                    next: ref __self_0_1,
                    price: ref __self_0_2,
                    buy_amount: ref __self_0_3,
                    sell_amount: ref __self_0_4,
                    orders: ref __self_0_5,
                } => LinkedItem {
                    prev: ::core::clone::Clone::clone(&(*__self_0_0)),
                    next: ::core::clone::Clone::clone(&(*__self_0_1)),
                    price: ::core::clone::Clone::clone(&(*__self_0_2)),
                    buy_amount: ::core::clone::Clone::clone(&(*__self_0_3)),
                    sell_amount: ::core::clone::Clone::clone(&(*__self_0_4)),
                    orders: ::core::clone::Clone::clone(&(*__self_0_5)),
                },
            }
        }
    }
    impl<K1, K2, K3> ::core::marker::StructuralPartialEq for LinkedItem<K1, K2, K3> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<K1: ::core::cmp::PartialEq, K2: ::core::cmp::PartialEq, K3: ::core::cmp::PartialEq>
        ::core::cmp::PartialEq for LinkedItem<K1, K2, K3>
    {
        #[inline]
        fn eq(&self, other: &LinkedItem<K1, K2, K3>) -> bool {
            match *other {
                LinkedItem {
                    prev: ref __self_1_0,
                    next: ref __self_1_1,
                    price: ref __self_1_2,
                    buy_amount: ref __self_1_3,
                    sell_amount: ref __self_1_4,
                    orders: ref __self_1_5,
                } => match *self {
                    LinkedItem {
                        prev: ref __self_0_0,
                        next: ref __self_0_1,
                        price: ref __self_0_2,
                        buy_amount: ref __self_0_3,
                        sell_amount: ref __self_0_4,
                        orders: ref __self_0_5,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                            && (*__self_0_4) == (*__self_1_4)
                            && (*__self_0_5) == (*__self_1_5)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &LinkedItem<K1, K2, K3>) -> bool {
            match *other {
                LinkedItem {
                    prev: ref __self_1_0,
                    next: ref __self_1_1,
                    price: ref __self_1_2,
                    buy_amount: ref __self_1_3,
                    sell_amount: ref __self_1_4,
                    orders: ref __self_1_5,
                } => match *self {
                    LinkedItem {
                        prev: ref __self_0_0,
                        next: ref __self_0_1,
                        price: ref __self_0_2,
                        buy_amount: ref __self_0_3,
                        sell_amount: ref __self_0_4,
                        orders: ref __self_0_5,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                            || (*__self_0_4) != (*__self_1_4)
                            || (*__self_0_5) != (*__self_1_5)
                    }
                },
            }
        }
    }
    impl<K1, K2, K3> ::core::marker::StructuralEq for LinkedItem<K1, K2, K3> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<K1: ::core::cmp::Eq, K2: ::core::cmp::Eq, K3: ::core::cmp::Eq> ::core::cmp::Eq
        for LinkedItem<K1, K2, K3>
    {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<Option<K2>>;
                let _: ::core::cmp::AssertParamIsEq<Option<K2>>;
                let _: ::core::cmp::AssertParamIsEq<Option<K2>>;
                let _: ::core::cmp::AssertParamIsEq<K3>;
                let _: ::core::cmp::AssertParamIsEq<K3>;
                let _: ::core::cmp::AssertParamIsEq<Vec<K1>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<K1: ::core::fmt::Debug, K2: ::core::fmt::Debug, K3: ::core::fmt::Debug> ::core::fmt::Debug
        for LinkedItem<K1, K2, K3>
    {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                LinkedItem {
                    prev: ref __self_0_0,
                    next: ref __self_0_1,
                    price: ref __self_0_2,
                    buy_amount: ref __self_0_3,
                    sell_amount: ref __self_0_4,
                    orders: ref __self_0_5,
                } => {
                    let mut debug_trait_builder = f.debug_struct("LinkedItem");
                    let _ = debug_trait_builder.field("prev", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("next", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("price", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("buy_amount", &&(*__self_0_3));
                    let _ = debug_trait_builder.field("sell_amount", &&(*__self_0_4));
                    let _ = debug_trait_builder.field("orders", &&(*__self_0_5));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    pub struct LinkedList<T, S, K1, K2, K3>(sp_std::marker::PhantomData<(T, S, K1, K2, K3)>);
    ///             LinkedItem          LinkedItem			LinkedItem          LinkedItem          LinkedItem
    ///             Bottom              Buy Order			Head                Sell Order          Top
    ///   			Next	    ---->   Price: 8	<----	Prev                Next       ---->    Price: max
    ///   max <---- Prev				Next		---->	Price:None  <----   Prev                Next        ---->   Price: 0
    ///         	Price:0		<----   Prev     			Next        ---->   Price 10   <----    Prev
    ///                                 Orders									Orders
    ///                                 o1: Hash -> buy 1@8						o101: Hash -> sell 100@10
    ///                                 o2: Hash -> buy 5@8						o102: Hash -> sell 100@10
    ///                                 o3: Hash -> buy 100@8
    ///                                 o4: Hash -> buy 40@8
    ///                                 o5: Hash -> buy 1000@8
    ///
    /// when do order matching, o1 will match before o2 and so on
    impl<T, S, K1, K2, K3> LinkedList<T, S, K1, K2, K3>
    where
        T: trade::Trait,
        K1: EncodeLike
            + Encode
            + Decode
            + Clone
            + sp_std::borrow::Borrow<<T as frame_system::Trait>::Hash>
            + Copy
            + PartialEq
            + AsRef<[u8]>,
        K2: Parameter + Default + Member + AtLeast32Bit + Bounded + Copy,
        K3: Parameter + Default + Member + AtLeast32Bit + Bounded + Copy,
        S: StorageMap<
            (K1, Option<K2>),
            LinkedItem<K1, K2, K3>,
            Query = Option<LinkedItem<K1, K2, K3>>,
        >,
    {
        pub fn read_head(key: K1) -> LinkedItem<K1, K2, K3> {
            Self::read(key, None)
        }
        #[allow(dead_code)]
        pub fn read_bottom(key: K1) -> LinkedItem<K1, K2, K3> {
            Self::read(key, Some(K2::min_value()))
        }
        #[allow(dead_code)]
        pub fn read_top(key: K1) -> LinkedItem<K1, K2, K3> {
            Self::read(key, Some(K2::max_value()))
        }
        pub fn read(key1: K1, key2: Option<K2>) -> LinkedItem<K1, K2, K3> {
            S::get((key1, key2)).unwrap_or_else(|| {
                let bottom = LinkedItem {
                    prev: Some(K2::max_value()),
                    next: None,
                    price: Some(K2::min_value()),
                    orders: Vec::<K1>::new(),
                    buy_amount: Default::default(),
                    sell_amount: Default::default(),
                };
                let top = LinkedItem {
                    prev: None,
                    next: Some(K2::min_value()),
                    price: Some(K2::max_value()),
                    orders: Vec::<K1>::new(),
                    buy_amount: Default::default(),
                    sell_amount: Default::default(),
                };
                let head = LinkedItem {
                    prev: Some(K2::min_value()),
                    next: Some(K2::max_value()),
                    price: None,
                    orders: Vec::<K1>::new(),
                    buy_amount: Default::default(),
                    sell_amount: Default::default(),
                };
                Self::write(key1, bottom.price, bottom);
                Self::write(key1, top.price, top);
                Self::write(key1, head.price, head.clone());
                head
            })
        }
        pub fn write(key1: K1, key2: Option<K2>, item: LinkedItem<K1, K2, K3>) {
            S::insert((key1, key2), item);
        }
        pub fn append(
            key1: K1,
            key2: K2,
            value: K1,
            sell_amount: K3,
            buy_amount: K3,
            otype: OrderType,
        ) {
            let item = S::get((key1, Some(key2)));
            match item {
                Some(mut item) => {
                    item.orders.push(value);
                    item.buy_amount = item.buy_amount + buy_amount;
                    item.sell_amount = item.sell_amount + sell_amount;
                    Self::write(key1, Some(key2), item);
                    return;
                }
                None => {
                    let start_item;
                    let end_item;
                    match otype {
                        OrderType::Buy => {
                            start_item = Some(K2::min_value());
                            end_item = None;
                        }
                        OrderType::Sell => {
                            start_item = None;
                            end_item = Some(K2::max_value());
                        }
                    }
                    let mut item = Self::read(key1, start_item);
                    while item.next != end_item {
                        match item.next {
                            None => {}
                            Some(price) => {
                                if key2 < price {
                                    break;
                                }
                            }
                        }
                        item = Self::read(key1, item.next);
                    }
                    let new_prev = LinkedItem {
                        next: Some(key2),
                        ..item
                    };
                    Self::write(key1, new_prev.price, new_prev.clone());
                    let next = Self::read(key1, item.next);
                    let new_next = LinkedItem {
                        prev: Some(key2),
                        ..next
                    };
                    Self::write(key1, new_next.price, new_next.clone());
                    let mut v = Vec::new();
                    v.push(value);
                    let item = LinkedItem {
                        prev: new_prev.price,
                        next: new_next.price,
                        buy_amount,
                        sell_amount,
                        orders: v,
                        price: Some(key2),
                    };
                    Self::write(key1, Some(key2), item);
                }
            };
        }
        pub fn next_match_price(item: &LinkedItem<K1, K2, K3>, otype: OrderType) -> Option<K2> {
            if otype == OrderType::Buy {
                item.prev
            } else {
                item.next
            }
        }
        pub fn update_amount(key1: K1, key2: K2, sell_amount: K3, buy_amount: K3) {
            let mut item = Self::read(key1, Some(key2));
            item.buy_amount = item.buy_amount - buy_amount;
            item.sell_amount = item.sell_amount - sell_amount;
            Self::write(key1, Some(key2), item);
        }
        pub fn remove_all(key1: K1, otype: OrderType) {
            let end_item;
            if otype == OrderType::Buy {
                end_item = Some(K2::min_value());
            } else {
                end_item = Some(K2::max_value());
            }
            let mut head = Self::read_head(key1);
            loop {
                let key2 = Self::next_match_price(&head, otype);
                if key2 == end_item {
                    break;
                }
                match Self::remove_orders_in_one_item(key1, key2.unwrap()) {
                    Err(_) => break,
                    _ => {}
                };
                head = Self::read_head(key1);
            }
        }
        pub fn remove_order(
            key1: K1,
            key2: K2,
            order_hash: K1,
            sell_amount: K3,
            buy_amount: K3,
        ) -> Result {
            match S::get((key1, Some(key2))) {
                Some(mut item) => {
                    {
                        if !item.orders.contains(&order_hash) {
                            {
                                return Err("cancel the order but not in market order list".into());
                            };
                        }
                    };
                    item.orders.retain(|&x| x != order_hash);
                    item.buy_amount = item.buy_amount - buy_amount;
                    item.sell_amount = item.sell_amount - sell_amount;
                    Self::write(key1, Some(key2), item.clone());
                    if item.orders.len() == 0 {
                        Self::remove_item(key1, key2);
                    }
                }
                None => {}
            }
            Ok(())
        }
        pub fn remove_item(key1: K1, key2: K2) {
            if let Some(item) = S::take((key1, Some(key2))) {
                S::mutate((key1.clone(), item.prev), |x| {
                    if let Some(x) = x {
                        x.next = item.next;
                    }
                });
                S::mutate((key1.clone(), item.next), |x| {
                    if let Some(x) = x {
                        x.prev = item.prev;
                    }
                });
            }
        }
        pub fn remove_orders_in_one_item(key1: K1, key2: K2) -> Result {
            match S::get((key1, Some(key2))) {
                Some(mut item) => {
                    while item.orders.len() > 0 {
                        let order_hash = item.orders.get(0).ok_or("can not get order hash")?;
                        let order = <trade::Module<T>>::order(order_hash.borrow())
                            .ok_or("can not get order")?;
                        {
                            if !order.is_finished() {
                                {
                                    return Err("try to remove not finished order".into());
                                };
                            }
                        };
                        item.orders.remove(0);
                        Self::write(key1, Some(key2), item.clone());
                    }
                    if item.orders.len() == 0 {
                        Self::remove_item(key1, key2);
                    }
                }
                None => {}
            }
            Ok(())
        }
    }
}
pub trait Trait: token::Trait + frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Price: Parameter
        + Default
        + Member
        + Bounded
        + AtLeast32Bit
        + Copy
        + From<u128>
        + Into<u128>;
    type PriceFactor: Get<u128>;
    type BlocksPerDay: Get<u32>;
    type OpenedOrdersArrayCap: Get<u8>;
    type ClosedOrdersArrayCap: Get<u8>;
}
///交易对
pub struct TradePair<T>
where
    T: Trait,
{
    hash: T::Hash,
    base: T::Hash,
    quote: T::Hash,
    latest_matched_price: Option<T::Price>,
    one_day_trade_volume: T::Balance,
    one_day_highest_price: Option<T::Price>,
    one_day_lowest_price: Option<T::Price>,
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T> _parity_scale_codec::Encode for TradePair<T>
    where
        T: Trait,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        Option<T::Price>: _parity_scale_codec::Encode,
        Option<T::Price>: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        Option<T::Price>: _parity_scale_codec::Encode,
        Option<T::Price>: _parity_scale_codec::Encode,
        Option<T::Price>: _parity_scale_codec::Encode,
        Option<T::Price>: _parity_scale_codec::Encode,
    {
        fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
            dest.push(&self.hash);
            dest.push(&self.base);
            dest.push(&self.quote);
            dest.push(&self.latest_matched_price);
            dest.push(&self.one_day_trade_volume);
            dest.push(&self.one_day_highest_price);
            dest.push(&self.one_day_lowest_price);
        }
    }
    impl<T> _parity_scale_codec::EncodeLike for TradePair<T>
    where
        T: Trait,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        Option<T::Price>: _parity_scale_codec::Encode,
        Option<T::Price>: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        Option<T::Price>: _parity_scale_codec::Encode,
        Option<T::Price>: _parity_scale_codec::Encode,
        Option<T::Price>: _parity_scale_codec::Encode,
        Option<T::Price>: _parity_scale_codec::Encode,
    {
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T> _parity_scale_codec::Decode for TradePair<T>
    where
        T: Trait,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        Option<T::Price>: _parity_scale_codec::Decode,
        Option<T::Price>: _parity_scale_codec::Decode,
        T::Balance: _parity_scale_codec::Decode,
        T::Balance: _parity_scale_codec::Decode,
        Option<T::Price>: _parity_scale_codec::Decode,
        Option<T::Price>: _parity_scale_codec::Decode,
        Option<T::Price>: _parity_scale_codec::Decode,
        Option<T::Price>: _parity_scale_codec::Decode,
    {
        fn decode<DecIn: _parity_scale_codec::Input>(
            input: &mut DecIn,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            Ok(TradePair {
                hash: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field TradePair.hash".into()),
                        Ok(a) => a,
                    }
                },
                base: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field TradePair.base".into()),
                        Ok(a) => a,
                    }
                },
                quote: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field TradePair.quote".into()),
                        Ok(a) => a,
                    }
                },
                latest_matched_price: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => {
                            return Err("Error decoding field TradePair.latest_matched_price".into())
                        }
                        Ok(a) => a,
                    }
                },
                one_day_trade_volume: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => {
                            return Err("Error decoding field TradePair.one_day_trade_volume".into())
                        }
                        Ok(a) => a,
                    }
                },
                one_day_highest_price: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => {
                            return Err(
                                "Error decoding field TradePair.one_day_highest_price".into()
                            )
                        }
                        Ok(a) => a,
                    }
                },
                one_day_lowest_price: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => {
                            return Err("Error decoding field TradePair.one_day_lowest_price".into())
                        }
                        Ok(a) => a,
                    }
                },
            })
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::clone::Clone> ::core::clone::Clone for TradePair<T>
where
    T: Trait,
    T::Hash: ::core::clone::Clone,
    T::Hash: ::core::clone::Clone,
    T::Hash: ::core::clone::Clone,
    T::Price: ::core::clone::Clone,
    T::Balance: ::core::clone::Clone,
    T::Price: ::core::clone::Clone,
    T::Price: ::core::clone::Clone,
{
    #[inline]
    fn clone(&self) -> TradePair<T> {
        match *self {
            TradePair {
                hash: ref __self_0_0,
                base: ref __self_0_1,
                quote: ref __self_0_2,
                latest_matched_price: ref __self_0_3,
                one_day_trade_volume: ref __self_0_4,
                one_day_highest_price: ref __self_0_5,
                one_day_lowest_price: ref __self_0_6,
            } => TradePair {
                hash: ::core::clone::Clone::clone(&(*__self_0_0)),
                base: ::core::clone::Clone::clone(&(*__self_0_1)),
                quote: ::core::clone::Clone::clone(&(*__self_0_2)),
                latest_matched_price: ::core::clone::Clone::clone(&(*__self_0_3)),
                one_day_trade_volume: ::core::clone::Clone::clone(&(*__self_0_4)),
                one_day_highest_price: ::core::clone::Clone::clone(&(*__self_0_5)),
                one_day_lowest_price: ::core::clone::Clone::clone(&(*__self_0_6)),
            },
        }
    }
}
impl<T> ::core::marker::StructuralPartialEq for TradePair<T> where T: Trait {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for TradePair<T>
where
    T: Trait,
    T::Hash: ::core::cmp::PartialEq,
    T::Hash: ::core::cmp::PartialEq,
    T::Hash: ::core::cmp::PartialEq,
    T::Price: ::core::cmp::PartialEq,
    T::Balance: ::core::cmp::PartialEq,
    T::Price: ::core::cmp::PartialEq,
    T::Price: ::core::cmp::PartialEq,
{
    #[inline]
    fn eq(&self, other: &TradePair<T>) -> bool {
        match *other {
            TradePair {
                hash: ref __self_1_0,
                base: ref __self_1_1,
                quote: ref __self_1_2,
                latest_matched_price: ref __self_1_3,
                one_day_trade_volume: ref __self_1_4,
                one_day_highest_price: ref __self_1_5,
                one_day_lowest_price: ref __self_1_6,
            } => match *self {
                TradePair {
                    hash: ref __self_0_0,
                    base: ref __self_0_1,
                    quote: ref __self_0_2,
                    latest_matched_price: ref __self_0_3,
                    one_day_trade_volume: ref __self_0_4,
                    one_day_highest_price: ref __self_0_5,
                    one_day_lowest_price: ref __self_0_6,
                } => {
                    (*__self_0_0) == (*__self_1_0)
                        && (*__self_0_1) == (*__self_1_1)
                        && (*__self_0_2) == (*__self_1_2)
                        && (*__self_0_3) == (*__self_1_3)
                        && (*__self_0_4) == (*__self_1_4)
                        && (*__self_0_5) == (*__self_1_5)
                        && (*__self_0_6) == (*__self_1_6)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, other: &TradePair<T>) -> bool {
        match *other {
            TradePair {
                hash: ref __self_1_0,
                base: ref __self_1_1,
                quote: ref __self_1_2,
                latest_matched_price: ref __self_1_3,
                one_day_trade_volume: ref __self_1_4,
                one_day_highest_price: ref __self_1_5,
                one_day_lowest_price: ref __self_1_6,
            } => match *self {
                TradePair {
                    hash: ref __self_0_0,
                    base: ref __self_0_1,
                    quote: ref __self_0_2,
                    latest_matched_price: ref __self_0_3,
                    one_day_trade_volume: ref __self_0_4,
                    one_day_highest_price: ref __self_0_5,
                    one_day_lowest_price: ref __self_0_6,
                } => {
                    (*__self_0_0) != (*__self_1_0)
                        || (*__self_0_1) != (*__self_1_1)
                        || (*__self_0_2) != (*__self_1_2)
                        || (*__self_0_3) != (*__self_1_3)
                        || (*__self_0_4) != (*__self_1_4)
                        || (*__self_0_5) != (*__self_1_5)
                        || (*__self_0_6) != (*__self_1_6)
                }
            },
        }
    }
}
impl<T> ::core::marker::StructuralEq for TradePair<T> where T: Trait {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::Eq> ::core::cmp::Eq for TradePair<T>
where
    T: Trait,
    T::Hash: ::core::cmp::Eq,
    T::Hash: ::core::cmp::Eq,
    T::Hash: ::core::cmp::Eq,
    T::Price: ::core::cmp::Eq,
    T::Balance: ::core::cmp::Eq,
    T::Price: ::core::cmp::Eq,
    T::Price: ::core::cmp::Eq,
{
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<T::Hash>;
            let _: ::core::cmp::AssertParamIsEq<T::Hash>;
            let _: ::core::cmp::AssertParamIsEq<T::Hash>;
            let _: ::core::cmp::AssertParamIsEq<Option<T::Price>>;
            let _: ::core::cmp::AssertParamIsEq<T::Balance>;
            let _: ::core::cmp::AssertParamIsEq<Option<T::Price>>;
            let _: ::core::cmp::AssertParamIsEq<Option<T::Price>>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::fmt::Debug> ::core::fmt::Debug for TradePair<T>
where
    T: Trait,
    T::Hash: ::core::fmt::Debug,
    T::Hash: ::core::fmt::Debug,
    T::Hash: ::core::fmt::Debug,
    T::Price: ::core::fmt::Debug,
    T::Balance: ::core::fmt::Debug,
    T::Price: ::core::fmt::Debug,
    T::Price: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            TradePair {
                hash: ref __self_0_0,
                base: ref __self_0_1,
                quote: ref __self_0_2,
                latest_matched_price: ref __self_0_3,
                one_day_trade_volume: ref __self_0_4,
                one_day_highest_price: ref __self_0_5,
                one_day_lowest_price: ref __self_0_6,
            } => {
                let mut debug_trait_builder = f.debug_struct("TradePair");
                let _ = debug_trait_builder.field("hash", &&(*__self_0_0));
                let _ = debug_trait_builder.field("base", &&(*__self_0_1));
                let _ = debug_trait_builder.field("quote", &&(*__self_0_2));
                let _ = debug_trait_builder.field("latest_matched_price", &&(*__self_0_3));
                let _ = debug_trait_builder.field("one_day_trade_volume", &&(*__self_0_4));
                let _ = debug_trait_builder.field("one_day_highest_price", &&(*__self_0_5));
                let _ = debug_trait_builder.field("one_day_lowest_price", &&(*__self_0_6));
                debug_trait_builder.finish()
            }
        }
    }
}
pub enum OrderType {
    Buy,
    Sell,
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Encode for OrderType {
        fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
            match *self {
                OrderType::Buy => {
                    dest.push_byte(0usize as u8);
                }
                OrderType::Sell => {
                    dest.push_byte(1usize as u8);
                }
                _ => (),
            }
        }
    }
    impl _parity_scale_codec::EncodeLike for OrderType {}
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Decode for OrderType {
        fn decode<DecIn: _parity_scale_codec::Input>(
            input: &mut DecIn,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            match input.read_byte()? {
                x if x == 0usize as u8 => Ok(OrderType::Buy),
                x if x == 1usize as u8 => Ok(OrderType::Sell),
                x => Err("No such variant in enum OrderType".into()),
            }
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for OrderType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&OrderType::Buy,) => {
                let mut debug_trait_builder = f.debug_tuple("Buy");
                debug_trait_builder.finish()
            }
            (&OrderType::Sell,) => {
                let mut debug_trait_builder = f.debug_tuple("Sell");
                debug_trait_builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for OrderType {
    #[inline]
    fn clone(&self) -> OrderType {
        {
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for OrderType {}
impl ::core::marker::StructuralPartialEq for OrderType {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for OrderType {
    #[inline]
    fn eq(&self, other: &OrderType) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) };
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) };
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ => true,
                }
            } else {
                false
            }
        }
    }
}
impl ::core::marker::StructuralEq for OrderType {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for OrderType {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
impl Not for OrderType {
    type Output = OrderType;
    ///取反方向的交易单
    fn not(self) -> Self::Output {
        match self {
            OrderType::Sell => OrderType::Buy,
            OrderType::Buy => OrderType::Sell,
        }
    }
}
pub enum OrderStatus {
    Created,
    PartialFilled,
    Filled,
    Canceled,
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Encode for OrderStatus {
        fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
            match *self {
                OrderStatus::Created => {
                    dest.push_byte(0usize as u8);
                }
                OrderStatus::PartialFilled => {
                    dest.push_byte(1usize as u8);
                }
                OrderStatus::Filled => {
                    dest.push_byte(2usize as u8);
                }
                OrderStatus::Canceled => {
                    dest.push_byte(3usize as u8);
                }
                _ => (),
            }
        }
    }
    impl _parity_scale_codec::EncodeLike for OrderStatus {}
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Decode for OrderStatus {
        fn decode<DecIn: _parity_scale_codec::Input>(
            input: &mut DecIn,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            match input.read_byte()? {
                x if x == 0usize as u8 => Ok(OrderStatus::Created),
                x if x == 1usize as u8 => Ok(OrderStatus::PartialFilled),
                x if x == 2usize as u8 => Ok(OrderStatus::Filled),
                x if x == 3usize as u8 => Ok(OrderStatus::Canceled),
                x => Err("No such variant in enum OrderStatus".into()),
            }
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for OrderStatus {
    #[inline]
    fn clone(&self) -> OrderStatus {
        match (&*self,) {
            (&OrderStatus::Created,) => OrderStatus::Created,
            (&OrderStatus::PartialFilled,) => OrderStatus::PartialFilled,
            (&OrderStatus::Filled,) => OrderStatus::Filled,
            (&OrderStatus::Canceled,) => OrderStatus::Canceled,
        }
    }
}
impl ::core::marker::StructuralPartialEq for OrderStatus {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for OrderStatus {
    #[inline]
    fn eq(&self, other: &OrderStatus) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) };
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) };
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    _ => true,
                }
            } else {
                false
            }
        }
    }
}
impl ::core::marker::StructuralEq for OrderStatus {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for OrderStatus {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for OrderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&OrderStatus::Created,) => {
                let mut debug_trait_builder = f.debug_tuple("Created");
                debug_trait_builder.finish()
            }
            (&OrderStatus::PartialFilled,) => {
                let mut debug_trait_builder = f.debug_tuple("PartialFilled");
                debug_trait_builder.finish()
            }
            (&OrderStatus::Filled,) => {
                let mut debug_trait_builder = f.debug_tuple("Filled");
                debug_trait_builder.finish()
            }
            (&OrderStatus::Canceled,) => {
                let mut debug_trait_builder = f.debug_tuple("Canceled");
                debug_trait_builder.finish()
            }
        }
    }
}
///限价单,订单的定义
pub struct LimitOrder<T>
where
    T: Trait,
{
    pub hash: T::Hash,
    pub base: T::Hash,
    pub quote: T::Hash,
    pub owner: T::AccountId,
    pub price: T::Price,
    pub sell_amount: T::Balance,
    pub buy_amount: T::Balance,
    pub remained_sell_amount: T::Balance,
    pub remained_buy_amount: T::Balance,
    pub otype: OrderType,
    pub status: OrderStatus,
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T> _parity_scale_codec::Encode for LimitOrder<T>
    where
        T: Trait,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::Price: _parity_scale_codec::Encode,
        T::Price: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
    {
        fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
            dest.push(&self.hash);
            dest.push(&self.base);
            dest.push(&self.quote);
            dest.push(&self.owner);
            dest.push(&self.price);
            dest.push(&self.sell_amount);
            dest.push(&self.buy_amount);
            dest.push(&self.remained_sell_amount);
            dest.push(&self.remained_buy_amount);
            dest.push(&self.otype);
            dest.push(&self.status);
        }
    }
    impl<T> _parity_scale_codec::EncodeLike for LimitOrder<T>
    where
        T: Trait,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::Price: _parity_scale_codec::Encode,
        T::Price: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
    {
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T> _parity_scale_codec::Decode for LimitOrder<T>
    where
        T: Trait,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::AccountId: _parity_scale_codec::Decode,
        T::AccountId: _parity_scale_codec::Decode,
        T::Price: _parity_scale_codec::Decode,
        T::Price: _parity_scale_codec::Decode,
        T::Balance: _parity_scale_codec::Decode,
        T::Balance: _parity_scale_codec::Decode,
        T::Balance: _parity_scale_codec::Decode,
        T::Balance: _parity_scale_codec::Decode,
        T::Balance: _parity_scale_codec::Decode,
        T::Balance: _parity_scale_codec::Decode,
        T::Balance: _parity_scale_codec::Decode,
        T::Balance: _parity_scale_codec::Decode,
    {
        fn decode<DecIn: _parity_scale_codec::Input>(
            input: &mut DecIn,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            Ok(LimitOrder {
                hash: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field LimitOrder.hash".into()),
                        Ok(a) => a,
                    }
                },
                base: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field LimitOrder.base".into()),
                        Ok(a) => a,
                    }
                },
                quote: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field LimitOrder.quote".into()),
                        Ok(a) => a,
                    }
                },
                owner: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field LimitOrder.owner".into()),
                        Ok(a) => a,
                    }
                },
                price: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field LimitOrder.price".into()),
                        Ok(a) => a,
                    }
                },
                sell_amount: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field LimitOrder.sell_amount".into()),
                        Ok(a) => a,
                    }
                },
                buy_amount: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field LimitOrder.buy_amount".into()),
                        Ok(a) => a,
                    }
                },
                remained_sell_amount: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => {
                            return Err(
                                "Error decoding field LimitOrder.remained_sell_amount".into()
                            )
                        }
                        Ok(a) => a,
                    }
                },
                remained_buy_amount: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => {
                            return Err("Error decoding field LimitOrder.remained_buy_amount".into())
                        }
                        Ok(a) => a,
                    }
                },
                otype: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field LimitOrder.otype".into()),
                        Ok(a) => a,
                    }
                },
                status: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field LimitOrder.status".into()),
                        Ok(a) => a,
                    }
                },
            })
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::clone::Clone> ::core::clone::Clone for LimitOrder<T>
where
    T: Trait,
    T::Hash: ::core::clone::Clone,
    T::Hash: ::core::clone::Clone,
    T::Hash: ::core::clone::Clone,
    T::AccountId: ::core::clone::Clone,
    T::Price: ::core::clone::Clone,
    T::Balance: ::core::clone::Clone,
    T::Balance: ::core::clone::Clone,
    T::Balance: ::core::clone::Clone,
    T::Balance: ::core::clone::Clone,
{
    #[inline]
    fn clone(&self) -> LimitOrder<T> {
        match *self {
            LimitOrder {
                hash: ref __self_0_0,
                base: ref __self_0_1,
                quote: ref __self_0_2,
                owner: ref __self_0_3,
                price: ref __self_0_4,
                sell_amount: ref __self_0_5,
                buy_amount: ref __self_0_6,
                remained_sell_amount: ref __self_0_7,
                remained_buy_amount: ref __self_0_8,
                otype: ref __self_0_9,
                status: ref __self_0_10,
            } => LimitOrder {
                hash: ::core::clone::Clone::clone(&(*__self_0_0)),
                base: ::core::clone::Clone::clone(&(*__self_0_1)),
                quote: ::core::clone::Clone::clone(&(*__self_0_2)),
                owner: ::core::clone::Clone::clone(&(*__self_0_3)),
                price: ::core::clone::Clone::clone(&(*__self_0_4)),
                sell_amount: ::core::clone::Clone::clone(&(*__self_0_5)),
                buy_amount: ::core::clone::Clone::clone(&(*__self_0_6)),
                remained_sell_amount: ::core::clone::Clone::clone(&(*__self_0_7)),
                remained_buy_amount: ::core::clone::Clone::clone(&(*__self_0_8)),
                otype: ::core::clone::Clone::clone(&(*__self_0_9)),
                status: ::core::clone::Clone::clone(&(*__self_0_10)),
            },
        }
    }
}
impl<T> ::core::marker::StructuralPartialEq for LimitOrder<T> where T: Trait {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for LimitOrder<T>
where
    T: Trait,
    T::Hash: ::core::cmp::PartialEq,
    T::Hash: ::core::cmp::PartialEq,
    T::Hash: ::core::cmp::PartialEq,
    T::AccountId: ::core::cmp::PartialEq,
    T::Price: ::core::cmp::PartialEq,
    T::Balance: ::core::cmp::PartialEq,
    T::Balance: ::core::cmp::PartialEq,
    T::Balance: ::core::cmp::PartialEq,
    T::Balance: ::core::cmp::PartialEq,
{
    #[inline]
    fn eq(&self, other: &LimitOrder<T>) -> bool {
        match *other {
            LimitOrder {
                hash: ref __self_1_0,
                base: ref __self_1_1,
                quote: ref __self_1_2,
                owner: ref __self_1_3,
                price: ref __self_1_4,
                sell_amount: ref __self_1_5,
                buy_amount: ref __self_1_6,
                remained_sell_amount: ref __self_1_7,
                remained_buy_amount: ref __self_1_8,
                otype: ref __self_1_9,
                status: ref __self_1_10,
            } => match *self {
                LimitOrder {
                    hash: ref __self_0_0,
                    base: ref __self_0_1,
                    quote: ref __self_0_2,
                    owner: ref __self_0_3,
                    price: ref __self_0_4,
                    sell_amount: ref __self_0_5,
                    buy_amount: ref __self_0_6,
                    remained_sell_amount: ref __self_0_7,
                    remained_buy_amount: ref __self_0_8,
                    otype: ref __self_0_9,
                    status: ref __self_0_10,
                } => {
                    (*__self_0_0) == (*__self_1_0)
                        && (*__self_0_1) == (*__self_1_1)
                        && (*__self_0_2) == (*__self_1_2)
                        && (*__self_0_3) == (*__self_1_3)
                        && (*__self_0_4) == (*__self_1_4)
                        && (*__self_0_5) == (*__self_1_5)
                        && (*__self_0_6) == (*__self_1_6)
                        && (*__self_0_7) == (*__self_1_7)
                        && (*__self_0_8) == (*__self_1_8)
                        && (*__self_0_9) == (*__self_1_9)
                        && (*__self_0_10) == (*__self_1_10)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, other: &LimitOrder<T>) -> bool {
        match *other {
            LimitOrder {
                hash: ref __self_1_0,
                base: ref __self_1_1,
                quote: ref __self_1_2,
                owner: ref __self_1_3,
                price: ref __self_1_4,
                sell_amount: ref __self_1_5,
                buy_amount: ref __self_1_6,
                remained_sell_amount: ref __self_1_7,
                remained_buy_amount: ref __self_1_8,
                otype: ref __self_1_9,
                status: ref __self_1_10,
            } => match *self {
                LimitOrder {
                    hash: ref __self_0_0,
                    base: ref __self_0_1,
                    quote: ref __self_0_2,
                    owner: ref __self_0_3,
                    price: ref __self_0_4,
                    sell_amount: ref __self_0_5,
                    buy_amount: ref __self_0_6,
                    remained_sell_amount: ref __self_0_7,
                    remained_buy_amount: ref __self_0_8,
                    otype: ref __self_0_9,
                    status: ref __self_0_10,
                } => {
                    (*__self_0_0) != (*__self_1_0)
                        || (*__self_0_1) != (*__self_1_1)
                        || (*__self_0_2) != (*__self_1_2)
                        || (*__self_0_3) != (*__self_1_3)
                        || (*__self_0_4) != (*__self_1_4)
                        || (*__self_0_5) != (*__self_1_5)
                        || (*__self_0_6) != (*__self_1_6)
                        || (*__self_0_7) != (*__self_1_7)
                        || (*__self_0_8) != (*__self_1_8)
                        || (*__self_0_9) != (*__self_1_9)
                        || (*__self_0_10) != (*__self_1_10)
                }
            },
        }
    }
}
impl<T> ::core::marker::StructuralEq for LimitOrder<T> where T: Trait {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::Eq> ::core::cmp::Eq for LimitOrder<T>
where
    T: Trait,
    T::Hash: ::core::cmp::Eq,
    T::Hash: ::core::cmp::Eq,
    T::Hash: ::core::cmp::Eq,
    T::AccountId: ::core::cmp::Eq,
    T::Price: ::core::cmp::Eq,
    T::Balance: ::core::cmp::Eq,
    T::Balance: ::core::cmp::Eq,
    T::Balance: ::core::cmp::Eq,
    T::Balance: ::core::cmp::Eq,
{
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<T::Hash>;
            let _: ::core::cmp::AssertParamIsEq<T::Hash>;
            let _: ::core::cmp::AssertParamIsEq<T::Hash>;
            let _: ::core::cmp::AssertParamIsEq<T::AccountId>;
            let _: ::core::cmp::AssertParamIsEq<T::Price>;
            let _: ::core::cmp::AssertParamIsEq<T::Balance>;
            let _: ::core::cmp::AssertParamIsEq<T::Balance>;
            let _: ::core::cmp::AssertParamIsEq<T::Balance>;
            let _: ::core::cmp::AssertParamIsEq<T::Balance>;
            let _: ::core::cmp::AssertParamIsEq<OrderType>;
            let _: ::core::cmp::AssertParamIsEq<OrderStatus>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::fmt::Debug> ::core::fmt::Debug for LimitOrder<T>
where
    T: Trait,
    T::Hash: ::core::fmt::Debug,
    T::Hash: ::core::fmt::Debug,
    T::Hash: ::core::fmt::Debug,
    T::AccountId: ::core::fmt::Debug,
    T::Price: ::core::fmt::Debug,
    T::Balance: ::core::fmt::Debug,
    T::Balance: ::core::fmt::Debug,
    T::Balance: ::core::fmt::Debug,
    T::Balance: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            LimitOrder {
                hash: ref __self_0_0,
                base: ref __self_0_1,
                quote: ref __self_0_2,
                owner: ref __self_0_3,
                price: ref __self_0_4,
                sell_amount: ref __self_0_5,
                buy_amount: ref __self_0_6,
                remained_sell_amount: ref __self_0_7,
                remained_buy_amount: ref __self_0_8,
                otype: ref __self_0_9,
                status: ref __self_0_10,
            } => {
                let mut debug_trait_builder = f.debug_struct("LimitOrder");
                let _ = debug_trait_builder.field("hash", &&(*__self_0_0));
                let _ = debug_trait_builder.field("base", &&(*__self_0_1));
                let _ = debug_trait_builder.field("quote", &&(*__self_0_2));
                let _ = debug_trait_builder.field("owner", &&(*__self_0_3));
                let _ = debug_trait_builder.field("price", &&(*__self_0_4));
                let _ = debug_trait_builder.field("sell_amount", &&(*__self_0_5));
                let _ = debug_trait_builder.field("buy_amount", &&(*__self_0_6));
                let _ = debug_trait_builder.field("remained_sell_amount", &&(*__self_0_7));
                let _ = debug_trait_builder.field("remained_buy_amount", &&(*__self_0_8));
                let _ = debug_trait_builder.field("otype", &&(*__self_0_9));
                let _ = debug_trait_builder.field("status", &&(*__self_0_10));
                debug_trait_builder.finish()
            }
        }
    }
}
///交易结果数据
pub struct Trade<T>
where
    T: Trait,
{
    hash: T::Hash,
    base: T::Hash,
    quote: T::Hash,
    buyer: T::AccountId,
    seller: T::AccountId,
    maker: T::AccountId,
    taker: T::AccountId,
    otype: OrderType,
    price: T::Price,
    base_amount: T::Balance,
    quote_amount: T::Balance,
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T> _parity_scale_codec::Encode for Trade<T>
    where
        T: Trait,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::Price: _parity_scale_codec::Encode,
        T::Price: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
    {
        fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
            dest.push(&self.hash);
            dest.push(&self.base);
            dest.push(&self.quote);
            dest.push(&self.buyer);
            dest.push(&self.seller);
            dest.push(&self.maker);
            dest.push(&self.taker);
            dest.push(&self.otype);
            dest.push(&self.price);
            dest.push(&self.base_amount);
            dest.push(&self.quote_amount);
        }
    }
    impl<T> _parity_scale_codec::EncodeLike for Trade<T>
    where
        T: Trait,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::AccountId: _parity_scale_codec::Encode,
        T::Price: _parity_scale_codec::Encode,
        T::Price: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
    {
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T> _parity_scale_codec::Decode for Trade<T>
    where
        T: Trait,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::AccountId: _parity_scale_codec::Decode,
        T::AccountId: _parity_scale_codec::Decode,
        T::AccountId: _parity_scale_codec::Decode,
        T::AccountId: _parity_scale_codec::Decode,
        T::AccountId: _parity_scale_codec::Decode,
        T::AccountId: _parity_scale_codec::Decode,
        T::AccountId: _parity_scale_codec::Decode,
        T::AccountId: _parity_scale_codec::Decode,
        T::Price: _parity_scale_codec::Decode,
        T::Price: _parity_scale_codec::Decode,
        T::Balance: _parity_scale_codec::Decode,
        T::Balance: _parity_scale_codec::Decode,
        T::Balance: _parity_scale_codec::Decode,
        T::Balance: _parity_scale_codec::Decode,
    {
        fn decode<DecIn: _parity_scale_codec::Input>(
            input: &mut DecIn,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            Ok(Trade {
                hash: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Trade.hash".into()),
                        Ok(a) => a,
                    }
                },
                base: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Trade.base".into()),
                        Ok(a) => a,
                    }
                },
                quote: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Trade.quote".into()),
                        Ok(a) => a,
                    }
                },
                buyer: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Trade.buyer".into()),
                        Ok(a) => a,
                    }
                },
                seller: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Trade.seller".into()),
                        Ok(a) => a,
                    }
                },
                maker: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Trade.maker".into()),
                        Ok(a) => a,
                    }
                },
                taker: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Trade.taker".into()),
                        Ok(a) => a,
                    }
                },
                otype: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Trade.otype".into()),
                        Ok(a) => a,
                    }
                },
                price: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Trade.price".into()),
                        Ok(a) => a,
                    }
                },
                base_amount: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Trade.base_amount".into()),
                        Ok(a) => a,
                    }
                },
                quote_amount: {
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => return Err("Error decoding field Trade.quote_amount".into()),
                        Ok(a) => a,
                    }
                },
            })
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::clone::Clone> ::core::clone::Clone for Trade<T>
where
    T: Trait,
    T::Hash: ::core::clone::Clone,
    T::Hash: ::core::clone::Clone,
    T::Hash: ::core::clone::Clone,
    T::AccountId: ::core::clone::Clone,
    T::AccountId: ::core::clone::Clone,
    T::AccountId: ::core::clone::Clone,
    T::AccountId: ::core::clone::Clone,
    T::Price: ::core::clone::Clone,
    T::Balance: ::core::clone::Clone,
    T::Balance: ::core::clone::Clone,
{
    #[inline]
    fn clone(&self) -> Trade<T> {
        match *self {
            Trade {
                hash: ref __self_0_0,
                base: ref __self_0_1,
                quote: ref __self_0_2,
                buyer: ref __self_0_3,
                seller: ref __self_0_4,
                maker: ref __self_0_5,
                taker: ref __self_0_6,
                otype: ref __self_0_7,
                price: ref __self_0_8,
                base_amount: ref __self_0_9,
                quote_amount: ref __self_0_10,
            } => Trade {
                hash: ::core::clone::Clone::clone(&(*__self_0_0)),
                base: ::core::clone::Clone::clone(&(*__self_0_1)),
                quote: ::core::clone::Clone::clone(&(*__self_0_2)),
                buyer: ::core::clone::Clone::clone(&(*__self_0_3)),
                seller: ::core::clone::Clone::clone(&(*__self_0_4)),
                maker: ::core::clone::Clone::clone(&(*__self_0_5)),
                taker: ::core::clone::Clone::clone(&(*__self_0_6)),
                otype: ::core::clone::Clone::clone(&(*__self_0_7)),
                price: ::core::clone::Clone::clone(&(*__self_0_8)),
                base_amount: ::core::clone::Clone::clone(&(*__self_0_9)),
                quote_amount: ::core::clone::Clone::clone(&(*__self_0_10)),
            },
        }
    }
}
impl<T> ::core::marker::StructuralPartialEq for Trade<T> where T: Trait {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for Trade<T>
where
    T: Trait,
    T::Hash: ::core::cmp::PartialEq,
    T::Hash: ::core::cmp::PartialEq,
    T::Hash: ::core::cmp::PartialEq,
    T::AccountId: ::core::cmp::PartialEq,
    T::AccountId: ::core::cmp::PartialEq,
    T::AccountId: ::core::cmp::PartialEq,
    T::AccountId: ::core::cmp::PartialEq,
    T::Price: ::core::cmp::PartialEq,
    T::Balance: ::core::cmp::PartialEq,
    T::Balance: ::core::cmp::PartialEq,
{
    #[inline]
    fn eq(&self, other: &Trade<T>) -> bool {
        match *other {
            Trade {
                hash: ref __self_1_0,
                base: ref __self_1_1,
                quote: ref __self_1_2,
                buyer: ref __self_1_3,
                seller: ref __self_1_4,
                maker: ref __self_1_5,
                taker: ref __self_1_6,
                otype: ref __self_1_7,
                price: ref __self_1_8,
                base_amount: ref __self_1_9,
                quote_amount: ref __self_1_10,
            } => match *self {
                Trade {
                    hash: ref __self_0_0,
                    base: ref __self_0_1,
                    quote: ref __self_0_2,
                    buyer: ref __self_0_3,
                    seller: ref __self_0_4,
                    maker: ref __self_0_5,
                    taker: ref __self_0_6,
                    otype: ref __self_0_7,
                    price: ref __self_0_8,
                    base_amount: ref __self_0_9,
                    quote_amount: ref __self_0_10,
                } => {
                    (*__self_0_0) == (*__self_1_0)
                        && (*__self_0_1) == (*__self_1_1)
                        && (*__self_0_2) == (*__self_1_2)
                        && (*__self_0_3) == (*__self_1_3)
                        && (*__self_0_4) == (*__self_1_4)
                        && (*__self_0_5) == (*__self_1_5)
                        && (*__self_0_6) == (*__self_1_6)
                        && (*__self_0_7) == (*__self_1_7)
                        && (*__self_0_8) == (*__self_1_8)
                        && (*__self_0_9) == (*__self_1_9)
                        && (*__self_0_10) == (*__self_1_10)
                }
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Trade<T>) -> bool {
        match *other {
            Trade {
                hash: ref __self_1_0,
                base: ref __self_1_1,
                quote: ref __self_1_2,
                buyer: ref __self_1_3,
                seller: ref __self_1_4,
                maker: ref __self_1_5,
                taker: ref __self_1_6,
                otype: ref __self_1_7,
                price: ref __self_1_8,
                base_amount: ref __self_1_9,
                quote_amount: ref __self_1_10,
            } => match *self {
                Trade {
                    hash: ref __self_0_0,
                    base: ref __self_0_1,
                    quote: ref __self_0_2,
                    buyer: ref __self_0_3,
                    seller: ref __self_0_4,
                    maker: ref __self_0_5,
                    taker: ref __self_0_6,
                    otype: ref __self_0_7,
                    price: ref __self_0_8,
                    base_amount: ref __self_0_9,
                    quote_amount: ref __self_0_10,
                } => {
                    (*__self_0_0) != (*__self_1_0)
                        || (*__self_0_1) != (*__self_1_1)
                        || (*__self_0_2) != (*__self_1_2)
                        || (*__self_0_3) != (*__self_1_3)
                        || (*__self_0_4) != (*__self_1_4)
                        || (*__self_0_5) != (*__self_1_5)
                        || (*__self_0_6) != (*__self_1_6)
                        || (*__self_0_7) != (*__self_1_7)
                        || (*__self_0_8) != (*__self_1_8)
                        || (*__self_0_9) != (*__self_1_9)
                        || (*__self_0_10) != (*__self_1_10)
                }
            },
        }
    }
}
impl<T> ::core::marker::StructuralEq for Trade<T> where T: Trait {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::Eq> ::core::cmp::Eq for Trade<T>
where
    T: Trait,
    T::Hash: ::core::cmp::Eq,
    T::Hash: ::core::cmp::Eq,
    T::Hash: ::core::cmp::Eq,
    T::AccountId: ::core::cmp::Eq,
    T::AccountId: ::core::cmp::Eq,
    T::AccountId: ::core::cmp::Eq,
    T::AccountId: ::core::cmp::Eq,
    T::Price: ::core::cmp::Eq,
    T::Balance: ::core::cmp::Eq,
    T::Balance: ::core::cmp::Eq,
{
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<T::Hash>;
            let _: ::core::cmp::AssertParamIsEq<T::Hash>;
            let _: ::core::cmp::AssertParamIsEq<T::Hash>;
            let _: ::core::cmp::AssertParamIsEq<T::AccountId>;
            let _: ::core::cmp::AssertParamIsEq<T::AccountId>;
            let _: ::core::cmp::AssertParamIsEq<T::AccountId>;
            let _: ::core::cmp::AssertParamIsEq<T::AccountId>;
            let _: ::core::cmp::AssertParamIsEq<OrderType>;
            let _: ::core::cmp::AssertParamIsEq<T::Price>;
            let _: ::core::cmp::AssertParamIsEq<T::Balance>;
            let _: ::core::cmp::AssertParamIsEq<T::Balance>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::fmt::Debug> ::core::fmt::Debug for Trade<T>
where
    T: Trait,
    T::Hash: ::core::fmt::Debug,
    T::Hash: ::core::fmt::Debug,
    T::Hash: ::core::fmt::Debug,
    T::AccountId: ::core::fmt::Debug,
    T::AccountId: ::core::fmt::Debug,
    T::AccountId: ::core::fmt::Debug,
    T::AccountId: ::core::fmt::Debug,
    T::Price: ::core::fmt::Debug,
    T::Balance: ::core::fmt::Debug,
    T::Balance: ::core::fmt::Debug,
{
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Trade {
                hash: ref __self_0_0,
                base: ref __self_0_1,
                quote: ref __self_0_2,
                buyer: ref __self_0_3,
                seller: ref __self_0_4,
                maker: ref __self_0_5,
                taker: ref __self_0_6,
                otype: ref __self_0_7,
                price: ref __self_0_8,
                base_amount: ref __self_0_9,
                quote_amount: ref __self_0_10,
            } => {
                let mut debug_trait_builder = f.debug_struct("Trade");
                let _ = debug_trait_builder.field("hash", &&(*__self_0_0));
                let _ = debug_trait_builder.field("base", &&(*__self_0_1));
                let _ = debug_trait_builder.field("quote", &&(*__self_0_2));
                let _ = debug_trait_builder.field("buyer", &&(*__self_0_3));
                let _ = debug_trait_builder.field("seller", &&(*__self_0_4));
                let _ = debug_trait_builder.field("maker", &&(*__self_0_5));
                let _ = debug_trait_builder.field("taker", &&(*__self_0_6));
                let _ = debug_trait_builder.field("otype", &&(*__self_0_7));
                let _ = debug_trait_builder.field("price", &&(*__self_0_8));
                let _ = debug_trait_builder.field("base_amount", &&(*__self_0_9));
                let _ = debug_trait_builder.field("quote_amount", &&(*__self_0_10));
                debug_trait_builder.finish()
            }
        }
    }
}
///限价单的方法实现
impl<T> LimitOrder<T>
where
    T: Trait,
{
    ///新建一个订单
    fn new(
        base: T::Hash,
        quote: T::Hash,
        owner: T::AccountId,
        price: T::Price,
        sell_amount: T::Balance,
        buy_amount: T::Balance,
        otype: OrderType,
    ) -> Self {
        let nonce = Nonce::get();
        let random_seed = <randomness_collective_flip::Module<T>>::random_seed();
        let hash = (
            random_seed,
            <frame_system::Module<T>>::block_number(),
            base,
            quote,
            owner.clone(),
            price,
            sell_amount,
            buy_amount,
            otype,
            nonce,
        )
            .using_encoded(<T as frame_system::Trait>::hash);
        LimitOrder {
            hash,
            base,
            quote,
            owner,
            price,
            otype,
            sell_amount,
            buy_amount,
            remained_buy_amount: buy_amount,
            remained_sell_amount: sell_amount,
            status: OrderStatus::Created,
        }
    }
    ///订单是否完成
    pub fn is_finished(&self) -> bool {
        (self.remained_buy_amount == Zero::zero() && self.status == OrderStatus::Filled)
            || self.status == OrderStatus::Canceled
    }
}
impl<T> Trade<T>
where
    T: Trait,
{
    fn new(
        base: T::Hash,
        quote: T::Hash,
        maker_order: &LimitOrder<T>,
        taker_order: &LimitOrder<T>,
        base_amount: T::Balance,
        quote_amount: T::Balance,
    ) -> Self {
        let nonce = Nonce::get();
        let random_seed = <randomness_collective_flip::Module<T>>::random_seed();
        let hash = (
            random_seed,
            <frame_system::Module<T>>::block_number(),
            nonce,
            maker_order.hash,
            maker_order.remained_sell_amount,
            maker_order.owner.clone(),
            taker_order.hash,
            taker_order.remained_sell_amount,
            taker_order.owner.clone(),
        )
            .using_encoded(<T as frame_system::Trait>::hash);
        Nonce::mutate(|x| *x += 1);
        let buyer;
        let seller;
        if taker_order.otype == OrderType::Buy {
            buyer = taker_order.owner.clone();
            seller = maker_order.owner.clone();
        } else {
            buyer = maker_order.owner.clone();
            seller = taker_order.owner.clone();
        }
        Trade {
            hash,
            base,
            quote,
            buyer,
            seller,
            base_amount,
            quote_amount,
            maker: maker_order.owner.clone(),
            taker: taker_order.owner.clone(),
            otype: taker_order.otype,
            price: maker_order.price,
        }
    }
}
type OrderLinkedItem<T> = types::LinkedItem<
    <T as frame_system::Trait>::Hash,
    <T as Trait>::Price,
    <T as balances::Trait>::Balance,
>;
type OrderLinkedItemList<T> = types::LinkedList<
    T,
    LinkedItemList<T>,
    <T as frame_system::Trait>::Hash,
    <T as Trait>::Price,
    <T as balances::Trait>::Balance,
>;
/// Error for the trade module.
pub enum Error<T: Trait> {
    #[doc(hidden)]
    __Ignore(
        ::frame_support::sp_std::marker::PhantomData<(T,)>,
        ::frame_support::Never,
    ),
    /// Price bounds check failed
    BoundsCheckFailed,
    /// Price length check failed
    PriceLengthCheckFailed,
    /// Number cast error
    NumberCastError,
    /// Overflow error
    OverflowError,
    /// No matching trade pair
    NoMatchingTradePair,
    /// Base equals to quote
    BaseEqualQuote,
    /// Token owner not found
    TokenOwnerNotFound,
    /// Sender not equal to base or quote owner
    SenderNotEqualToBaseOrQuoteOwner,
    /// Same trade pair with the given base and quote was already exist
    TradePairExisted,
    /// Get price error
    OrderMatchGetPriceError,
    /// Get linked list item error
    OrderMatchGetLinkedListItemError,
    /// Get order error
    OrderMatchGetOrderError,
    /// Order match substract error
    OrderMatchSubstractError,
    /// Order match order is not finish
    OrderMatchOrderIsNotFinished,
    /// No matching order
    NoMatchingOrder,
    /// Can only cancel own order
    CanOnlyCancelOwnOrder,
    /// can only cancel not finished order
    CanOnlyCancelNotFinishedOrder,
}
impl<T: Trait> ::frame_support::sp_std::fmt::Debug for Error<T> {
    fn fmt(
        &self,
        f: &mut ::frame_support::sp_std::fmt::Formatter<'_>,
    ) -> ::frame_support::sp_std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl<T: Trait> Error<T> {
    fn as_u8(&self) -> u8 {
        match self {
            Error::__Ignore(_, _) => ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                &["internal error: entered unreachable code: "],
                &match (&"`__Ignore` can never be constructed",) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            )),
            Error::BoundsCheckFailed => 0,
            Error::PriceLengthCheckFailed => 0 + 1,
            Error::NumberCastError => 0 + 1 + 1,
            Error::OverflowError => 0 + 1 + 1 + 1,
            Error::NoMatchingTradePair => 0 + 1 + 1 + 1 + 1,
            Error::BaseEqualQuote => 0 + 1 + 1 + 1 + 1 + 1,
            Error::TokenOwnerNotFound => 0 + 1 + 1 + 1 + 1 + 1 + 1,
            Error::SenderNotEqualToBaseOrQuoteOwner => 0 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
            Error::TradePairExisted => 0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
            Error::OrderMatchGetPriceError => 0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
            Error::OrderMatchGetLinkedListItemError => 0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
            Error::OrderMatchGetOrderError => 0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
            Error::OrderMatchSubstractError => 0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
            Error::OrderMatchOrderIsNotFinished => {
                0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
            }
            Error::NoMatchingOrder => 0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
            Error::CanOnlyCancelOwnOrder => {
                0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
            }
            Error::CanOnlyCancelNotFinishedOrder => {
                0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
            }
        }
    }
    fn as_str(&self) -> &'static str {
        match self {
            Self::__Ignore(_, _) => ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                &["internal error: entered unreachable code: "],
                &match (&"`__Ignore` can never be constructed",) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            )),
            Error::BoundsCheckFailed => "BoundsCheckFailed",
            Error::PriceLengthCheckFailed => "PriceLengthCheckFailed",
            Error::NumberCastError => "NumberCastError",
            Error::OverflowError => "OverflowError",
            Error::NoMatchingTradePair => "NoMatchingTradePair",
            Error::BaseEqualQuote => "BaseEqualQuote",
            Error::TokenOwnerNotFound => "TokenOwnerNotFound",
            Error::SenderNotEqualToBaseOrQuoteOwner => "SenderNotEqualToBaseOrQuoteOwner",
            Error::TradePairExisted => "TradePairExisted",
            Error::OrderMatchGetPriceError => "OrderMatchGetPriceError",
            Error::OrderMatchGetLinkedListItemError => "OrderMatchGetLinkedListItemError",
            Error::OrderMatchGetOrderError => "OrderMatchGetOrderError",
            Error::OrderMatchSubstractError => "OrderMatchSubstractError",
            Error::OrderMatchOrderIsNotFinished => "OrderMatchOrderIsNotFinished",
            Error::NoMatchingOrder => "NoMatchingOrder",
            Error::CanOnlyCancelOwnOrder => "CanOnlyCancelOwnOrder",
            Error::CanOnlyCancelNotFinishedOrder => "CanOnlyCancelNotFinishedOrder",
        }
    }
}
impl<T: Trait> From<Error<T>> for &'static str {
    fn from(err: Error<T>) -> &'static str {
        err.as_str()
    }
}
impl<T: Trait> From<Error<T>> for ::frame_support::sp_runtime::DispatchError {
    fn from(err: Error<T>) -> Self {
        let index = <T::PalletInfo as ::frame_support::traits::PalletInfo>::index::<Module<T>>()
            .expect("Every active module has an index in the runtime; qed")
            as u8;
        ::frame_support::sp_runtime::DispatchError::Module {
            index,
            error: err.as_u8(),
            message: Some(err.as_str()),
        }
    }
}
impl<T: Trait> ::frame_support::error::ModuleErrorMetadata for Error<T> {
    fn metadata() -> &'static [::frame_support::error::ErrorMetadata] {
        &[
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("BoundsCheckFailed"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" Price bounds check failed",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("PriceLengthCheckFailed"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" Price length check failed",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("NumberCastError"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" Number cast error",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("OverflowError"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" Overflow error",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("NoMatchingTradePair"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" No matching trade pair",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("BaseEqualQuote"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" Base equals to quote",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("TokenOwnerNotFound"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" Token owner not found",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode(
                    "SenderNotEqualToBaseOrQuoteOwner",
                ),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" Sender not equal to base or quote owner",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("TradePairExisted"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" Same trade pair with the given base and quote was already exist",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("OrderMatchGetPriceError"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" Get price error",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode(
                    "OrderMatchGetLinkedListItemError",
                ),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" Get linked list item error",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("OrderMatchGetOrderError"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" Get order error",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("OrderMatchSubstractError"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" Order match substract error",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode(
                    "OrderMatchOrderIsNotFinished",
                ),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" Order match order is not finish",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("NoMatchingOrder"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" No matching order",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode("CanOnlyCancelOwnOrder"),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" Can only cancel own order",
                ]),
            },
            ::frame_support::error::ErrorMetadata {
                name: ::frame_support::error::DecodeDifferent::Encode(
                    "CanOnlyCancelNotFinishedOrder",
                ),
                documentation: ::frame_support::error::DecodeDifferent::Encode(&[
                    r" can only cancel not finished order",
                ]),
            },
        ]
    }
}
use self::sp_api_hidden_includes_decl_storage::hidden_include::{
    StorageValue as _, StorageMap as _, StorageDoubleMap as _, StoragePrefixedMap as _,
    IterableStorageMap as _, IterableStorageDoubleMap as _,
};
#[doc(hidden)]
mod sp_api_hidden_includes_decl_storage {
    pub extern crate frame_support as hidden_include;
}
trait Store {
    type TradePairs;
    type TradePairsHashByBaseQuote;
    type TradePairsHashByIndex;
    type TradePairsIndex;
    type Orders;
    type OwnedOrders;
    type OwnedOrdersIndex;
    type OrderOwnedTrades;
    type OrderOwnedTradesIndex;
    type TradePairOwnedOrders;
    type TradePairOwnedOrdersIndex;
    type LinkedItemList;
    type Trades;
    type OwnedTrades;
    type OwnedTradesIndex;
    type OwnedTPTrades;
    type OwnedTPTradesIndex;
    type OwnedTPOpenedOrders;
    type OwnedTPClosedOrders;
    type TradePairOwnedTrades;
    type TradePairOwnedTradesIndex;
    type TPTradeDataBucket;
    type TPTradePriceBucket;
    type Nonce;
}
impl<T: Trait + 'static> Store for Module<T> {
    type TradePairs = TradePairs<T>;
    type TradePairsHashByBaseQuote = TradePairsHashByBaseQuote<T>;
    type TradePairsHashByIndex = TradePairsHashByIndex<T>;
    type TradePairsIndex = TradePairsIndex;
    type Orders = Orders<T>;
    type OwnedOrders = OwnedOrders<T>;
    type OwnedOrdersIndex = OwnedOrdersIndex<T>;
    type OrderOwnedTrades = OrderOwnedTrades<T>;
    type OrderOwnedTradesIndex = OrderOwnedTradesIndex<T>;
    type TradePairOwnedOrders = TradePairOwnedOrders<T>;
    type TradePairOwnedOrdersIndex = TradePairOwnedOrdersIndex<T>;
    type LinkedItemList = LinkedItemList<T>;
    type Trades = Trades<T>;
    type OwnedTrades = OwnedTrades<T>;
    type OwnedTradesIndex = OwnedTradesIndex<T>;
    type OwnedTPTrades = OwnedTPTrades<T>;
    type OwnedTPTradesIndex = OwnedTPTradesIndex<T>;
    type OwnedTPOpenedOrders = OwnedTPOpenedOrders<T>;
    type OwnedTPClosedOrders = OwnedTPClosedOrders<T>;
    type TradePairOwnedTrades = TradePairOwnedTrades<T>;
    type TradePairOwnedTradesIndex = TradePairOwnedTradesIndex<T>;
    type TPTradeDataBucket = TPTradeDataBucket<T>;
    type TPTradePriceBucket = TPTradePriceBucket<T>;
    type Nonce = Nonce;
}
impl<T: Trait + 'static> Module<T> {
    ///	TradePairHash => TradePair
    pub fn trade_pair<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<T::Hash>,
    >(
        key: K,
    ) -> Option<TradePair<T>> {
        < TradePairs < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < T :: Hash , TradePair < T > > > :: get (key)
    }
    /// (BaseTokenHash, quoteTokenHash) => TradePairHash
    pub fn trade_pair_hash_by_base_quote<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<(
            T::Hash,
            T::Hash,
        )>,
    >(
        key: K,
    ) -> Option<T::Hash> {
        < TradePairsHashByBaseQuote < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < (T :: Hash , T :: Hash) , T :: Hash > > :: get (key)
    }
    /// Index => TradePairHash
    pub fn trade_pair_hash_by_index<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<u64>,
    >(
        key: K,
    ) -> Option<T::Hash> {
        < TradePairsHashByIndex < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < u64 , T :: Hash > > :: get (key)
    }
    /// Index
    pub fn trade_pair_index() -> u64 {
        < TradePairsIndex < > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageValue < u64 > > :: get ()
    }
    /// OrderHash => Order
    pub fn order<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<T::Hash>,
    >(
        key: K,
    ) -> Option<LimitOrder<T>> {
        < Orders < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < T :: Hash , LimitOrder < T > > > :: get (key)
    }
    /// (AccoundId, Index) => OrderHash
    pub fn owned_order<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<(
            T::AccountId,
            u64,
        )>,
    >(
        key: K,
    ) -> Option<T::Hash> {
        < OwnedOrders < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < (T :: AccountId , u64) , T :: Hash > > :: get (key)
    }
    ///	AccountId => Index
    pub fn owned_orders_index<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<T::AccountId>,
    >(
        key: K,
    ) -> u64 {
        < OwnedOrdersIndex < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < T :: AccountId , u64 > > :: get (key)
    }
    /// (OrderHash, u64) => TradeHash
    pub fn order_owned_trades<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<(
            T::Hash,
            u64,
        )>,
    >(
        key: K,
    ) -> Option<T::Hash> {
        < OrderOwnedTrades < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < (T :: Hash , u64) , T :: Hash > > :: get (key)
    }
    /// (OrderHash, u64) => TradeHash
    pub fn order_owned_trades_index<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<T::Hash>,
    >(
        key: K,
    ) -> u64 {
        < OrderOwnedTradesIndex < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < T :: Hash , u64 > > :: get (key)
    }
    /// (TradePairHash, Index) => OrderHash
    pub fn trade_pair_owned_order<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<(
            T::Hash,
            u64,
        )>,
    >(
        key: K,
    ) -> Option<T::Hash> {
        < TradePairOwnedOrders < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < (T :: Hash , u64) , T :: Hash > > :: get (key)
    }
    /// TradePairHash => Index
    pub fn trade_pair_owned_order_index<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<T::Hash>,
    >(
        key: K,
    ) -> u64 {
        < TradePairOwnedOrdersIndex < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < T :: Hash , u64 > > :: get (key)
    }
    /// (TradePairHash, Price) => LinkedItem
    pub fn linked_item<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<(
            T::Hash,
            Option<T::Price>,
        )>,
    >(
        key: K,
    ) -> Option<OrderLinkedItem<T>> {
        < LinkedItemList < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < (T :: Hash , Option < T :: Price >) , OrderLinkedItem < T > > > :: get (key)
    }
    /// TradeHash => Trade
    pub fn trade<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<T::Hash>,
    >(
        key: K,
    ) -> Option<Trade<T>> {
        < Trades < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < T :: Hash , Trade < T > > > :: get (key)
    }
    /// (AccountId, u64) => TradeHash
    pub fn owned_trades<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<(
            T::AccountId,
            u64,
        )>,
    >(
        key: K,
    ) -> Option<T::Hash> {
        < OwnedTrades < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < (T :: AccountId , u64) , T :: Hash > > :: get (key)
    }
    /// AccountId => u64
    pub fn owned_trades_index<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<T::AccountId>,
    >(
        key: K,
    ) -> u64 {
        < OwnedTradesIndex < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < T :: AccountId , u64 > > :: get (key)
    }
    /// (AccountId, TradePairHash, u64) => TradeHash
    pub fn owned_tp_trades<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<(
            T::AccountId,
            T::Hash,
            u64,
        )>,
    >(
        key: K,
    ) -> Option<T::Hash> {
        < OwnedTPTrades < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < (T :: AccountId , T :: Hash , u64) , T :: Hash > > :: get (key)
    }
    /// (AccountId, TradePairHash) => u64
    pub fn owned_tp_trades_index<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<(
            T::AccountId,
            T::Hash,
        )>,
    >(
        key: K,
    ) -> u64 {
        < OwnedTPTradesIndex < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < (T :: AccountId , T :: Hash) , u64 > > :: get (key)
    }
    /// (AccountId, TradePairHash) => Vec<OrderHash>
    pub fn owned_tp_opened_orders<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<(
            T::AccountId,
            T::Hash,
        )>,
    >(
        key: K,
    ) -> Option<Vec<T::Hash>> {
        < OwnedTPOpenedOrders < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < (T :: AccountId , T :: Hash) , Vec < T :: Hash > > > :: get (key)
    }
    /// (AccountId, TradePairHash) => Vec<OrderHash>
    pub fn owned_tp_closed_orders<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<(
            T::AccountId,
            T::Hash,
        )>,
    >(
        key: K,
    ) -> Option<Vec<T::Hash>> {
        < OwnedTPClosedOrders < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < (T :: AccountId , T :: Hash) , Vec < T :: Hash > > > :: get (key)
    }
    /// (TradePairHash, u64) => TradeHash
    pub fn trade_pair_owned_trades<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<(
            T::Hash,
            u64,
        )>,
    >(
        key: K,
    ) -> Option<T::Hash> {
        < TradePairOwnedTrades < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < (T :: Hash , u64) , T :: Hash > > :: get (key)
    }
    /// TradePairHash => u64
    pub fn trade_pair_owned_trades_index<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<T::Hash>,
    >(
        key: K,
    ) -> u64 {
        < TradePairOwnedTradesIndex < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < T :: Hash , u64 > > :: get (key)
    }
    /// (TradePairHash, BlockNumber) => (Sum_of_Trade_Volume, Highest_Price, Lowest_Price)
    pub fn trade_pair_trade_data_bucket<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<(
            T::Hash,
            T::BlockNumber,
        )>,
    >(
        key: K,
    ) -> (T::Balance, Option<T::Price>, Option<T::Price>) {
        < TPTradeDataBucket < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < (T :: Hash , T :: BlockNumber) , (T :: Balance , Option < T :: Price > , Option < T :: Price >) > > :: get (key)
    }
    /// store the trade pair's H/L price within last day
    /// TradePairHash => (Vec<Highest_Price>, Vec<Lowest_Price>)
    pub fn trade_pair_trade_price_bucket<
        K: self::sp_api_hidden_includes_decl_storage::hidden_include::codec::EncodeLike<T::Hash>,
    >(
        key: K,
    ) -> (Vec<Option<T::Price>>, Vec<Option<T::Price>>) {
        < TPTradePriceBucket < T > as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: storage :: StorageMap < T :: Hash , (Vec < Option < T :: Price > > , Vec < Option < T :: Price > >) > > :: get (key)
    }
}
#[doc(hidden)]
pub struct __GetByteStructTradePairs<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_TradePairs:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructTradePairs<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_TradePairs
            .get_or_init(|| {
                let def_val: Option<TradePair<T>> = Default::default();
                <Option<TradePair<T>> as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructTradePairs<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructTradePairs<T> {}
#[doc(hidden)]
pub struct __GetByteStructTradePairsHashByBaseQuote<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_TradePairsHashByBaseQuote:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructTradePairsHashByBaseQuote<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_TradePairsHashByBaseQuote
            .get_or_init(|| {
                let def_val: Option<T::Hash> = Default::default();
                <Option<T::Hash> as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructTradePairsHashByBaseQuote<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructTradePairsHashByBaseQuote<T> {}
#[doc(hidden)]
pub struct __GetByteStructTradePairsHashByIndex<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_TradePairsHashByIndex:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructTradePairsHashByIndex<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_TradePairsHashByIndex
            .get_or_init(|| {
                let def_val: Option<T::Hash> = Default::default();
                <Option<T::Hash> as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructTradePairsHashByIndex<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructTradePairsHashByIndex<T> {}
#[doc(hidden)]
pub struct __GetByteStructTradePairsIndex<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_TradePairsIndex:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructTradePairsIndex<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_TradePairsIndex
            .get_or_init(|| {
                let def_val: u64 = Default::default();
                <u64 as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructTradePairsIndex<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructTradePairsIndex<T> {}
#[doc(hidden)]
pub struct __GetByteStructOrders<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Orders:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructOrders<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Orders
            .get_or_init(|| {
                let def_val: Option<LimitOrder<T>> = Default::default();
                <Option<LimitOrder<T>> as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructOrders<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructOrders<T> {}
#[doc(hidden)]
pub struct __GetByteStructOwnedOrders<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_OwnedOrders:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructOwnedOrders<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_OwnedOrders
            .get_or_init(|| {
                let def_val: Option<T::Hash> = Default::default();
                <Option<T::Hash> as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructOwnedOrders<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructOwnedOrders<T> {}
#[doc(hidden)]
pub struct __GetByteStructOwnedOrdersIndex<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_OwnedOrdersIndex:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructOwnedOrdersIndex<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_OwnedOrdersIndex
            .get_or_init(|| {
                let def_val: u64 = Default::default();
                <u64 as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructOwnedOrdersIndex<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructOwnedOrdersIndex<T> {}
#[doc(hidden)]
pub struct __GetByteStructOrderOwnedTrades<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_OrderOwnedTrades:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructOrderOwnedTrades<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_OrderOwnedTrades
            .get_or_init(|| {
                let def_val: Option<T::Hash> = Default::default();
                <Option<T::Hash> as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructOrderOwnedTrades<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructOrderOwnedTrades<T> {}
#[doc(hidden)]
pub struct __GetByteStructOrderOwnedTradesIndex<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_OrderOwnedTradesIndex:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructOrderOwnedTradesIndex<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_OrderOwnedTradesIndex
            .get_or_init(|| {
                let def_val: u64 = Default::default();
                <u64 as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructOrderOwnedTradesIndex<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructOrderOwnedTradesIndex<T> {}
#[doc(hidden)]
pub struct __GetByteStructTradePairOwnedOrders<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_TradePairOwnedOrders:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructTradePairOwnedOrders<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_TradePairOwnedOrders
            .get_or_init(|| {
                let def_val: Option<T::Hash> = Default::default();
                <Option<T::Hash> as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructTradePairOwnedOrders<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructTradePairOwnedOrders<T> {}
#[doc(hidden)]
pub struct __GetByteStructTradePairOwnedOrdersIndex<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_TradePairOwnedOrdersIndex:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructTradePairOwnedOrdersIndex<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_TradePairOwnedOrdersIndex
            .get_or_init(|| {
                let def_val: u64 = Default::default();
                <u64 as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructTradePairOwnedOrdersIndex<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructTradePairOwnedOrdersIndex<T> {}
#[doc(hidden)]
pub struct __GetByteStructLinkedItemList<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_LinkedItemList:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructLinkedItemList<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_LinkedItemList
            .get_or_init(|| {
                let def_val: Option<OrderLinkedItem<T>> = Default::default();
                <Option<OrderLinkedItem<T>> as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructLinkedItemList<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructLinkedItemList<T> {}
#[doc(hidden)]
pub struct __GetByteStructTrades<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Trades:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructTrades<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Trades
            .get_or_init(|| {
                let def_val: Option<Trade<T>> = Default::default();
                <Option<Trade<T>> as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructTrades<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructTrades<T> {}
#[doc(hidden)]
pub struct __GetByteStructOwnedTrades<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_OwnedTrades:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructOwnedTrades<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_OwnedTrades
            .get_or_init(|| {
                let def_val: Option<T::Hash> = Default::default();
                <Option<T::Hash> as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructOwnedTrades<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructOwnedTrades<T> {}
#[doc(hidden)]
pub struct __GetByteStructOwnedTradesIndex<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_OwnedTradesIndex:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructOwnedTradesIndex<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_OwnedTradesIndex
            .get_or_init(|| {
                let def_val: u64 = Default::default();
                <u64 as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructOwnedTradesIndex<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructOwnedTradesIndex<T> {}
#[doc(hidden)]
pub struct __GetByteStructOwnedTPTrades<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_OwnedTPTrades:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructOwnedTPTrades<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_OwnedTPTrades
            .get_or_init(|| {
                let def_val: Option<T::Hash> = Default::default();
                <Option<T::Hash> as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructOwnedTPTrades<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructOwnedTPTrades<T> {}
#[doc(hidden)]
pub struct __GetByteStructOwnedTPTradesIndex<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_OwnedTPTradesIndex:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructOwnedTPTradesIndex<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_OwnedTPTradesIndex
            .get_or_init(|| {
                let def_val: u64 = Default::default();
                <u64 as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructOwnedTPTradesIndex<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructOwnedTPTradesIndex<T> {}
#[doc(hidden)]
pub struct __GetByteStructOwnedTPOpenedOrders<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_OwnedTPOpenedOrders:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructOwnedTPOpenedOrders<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_OwnedTPOpenedOrders
            .get_or_init(|| {
                let def_val: Option<Vec<T::Hash>> = Default::default();
                <Option<Vec<T::Hash>> as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructOwnedTPOpenedOrders<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructOwnedTPOpenedOrders<T> {}
#[doc(hidden)]
pub struct __GetByteStructOwnedTPClosedOrders<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_OwnedTPClosedOrders:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructOwnedTPClosedOrders<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_OwnedTPClosedOrders
            .get_or_init(|| {
                let def_val: Option<Vec<T::Hash>> = Default::default();
                <Option<Vec<T::Hash>> as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructOwnedTPClosedOrders<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructOwnedTPClosedOrders<T> {}
#[doc(hidden)]
pub struct __GetByteStructTradePairOwnedTrades<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_TradePairOwnedTrades:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructTradePairOwnedTrades<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_TradePairOwnedTrades
            .get_or_init(|| {
                let def_val: Option<T::Hash> = Default::default();
                <Option<T::Hash> as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructTradePairOwnedTrades<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructTradePairOwnedTrades<T> {}
#[doc(hidden)]
pub struct __GetByteStructTradePairOwnedTradesIndex<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_TradePairOwnedTradesIndex:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructTradePairOwnedTradesIndex<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_TradePairOwnedTradesIndex
            .get_or_init(|| {
                let def_val: u64 = Default::default();
                <u64 as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructTradePairOwnedTradesIndex<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructTradePairOwnedTradesIndex<T> {}
#[doc(hidden)]
pub struct __GetByteStructTPTradeDataBucket<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_TPTradeDataBucket:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructTPTradeDataBucket<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_TPTradeDataBucket
            .get_or_init(|| {
                let def_val: (T::Balance, Option<T::Price>, Option<T::Price>) = Default::default();
                <(T::Balance, Option<T::Price>, Option<T::Price>) as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructTPTradeDataBucket<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructTPTradeDataBucket<T> {}
#[doc(hidden)]
pub struct __GetByteStructTPTradePriceBucket<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_TPTradePriceBucket:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructTPTradePriceBucket<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_TPTradePriceBucket
            .get_or_init(|| {
                let def_val: (Vec<Option<T::Price>>, Vec<Option<T::Price>>) = Default::default();
                <(Vec<Option<T::Price>>, Vec<Option<T::Price>>) as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructTPTradePriceBucket<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructTPTradePriceBucket<T> {}
#[doc(hidden)]
pub struct __GetByteStructNonce<T>(
    pub self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T)>,
);
#[cfg(feature = "std")]
#[allow(non_upper_case_globals)]
static __CACHE_GET_BYTE_STRUCT_Nonce:
    self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
        self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8>,
    > = self::sp_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::new();
#[cfg(feature = "std")]
impl<T: Trait> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
    for __GetByteStructNonce<T>
{
    fn default_byte(
        &self,
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::vec::Vec<u8> {
        use self::sp_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
        __CACHE_GET_BYTE_STRUCT_Nonce
            .get_or_init(|| {
                let def_val: u64 = Default::default();
                <u64 as Encode>::encode(&def_val)
            })
            .clone()
    }
}
unsafe impl<T: Trait> Send for __GetByteStructNonce<T> {}
unsafe impl<T: Trait> Sync for __GetByteStructNonce<T> {}
impl<T: Trait + 'static> Module<T> {
    #[doc(hidden)]
    pub fn storage_metadata(
    ) -> self::sp_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata {
        self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageMetadata { prefix : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("TradeModule") , entries : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("TradePairs") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Optional , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("T::Hash") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("TradePair<T>") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructTradePairs :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& ["\tTradePairHash => TradePair"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("TradePairsHashByBaseQuote") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Optional , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("(T::Hash, T::Hash)") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("T::Hash") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructTradePairsHashByBaseQuote :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" (BaseTokenHash, quoteTokenHash) => TradePairHash"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("TradePairsHashByIndex") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Optional , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("u64") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("T::Hash") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructTradePairsHashByIndex :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" Index => TradePairHash"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("TradePairsIndex") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Default , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Plain (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("u64")) , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructTradePairsIndex :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" Index"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Orders") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Optional , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("T::Hash") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("LimitOrder<T>") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructOrders :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" OrderHash => Order"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("OwnedOrders") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Optional , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("(T::AccountId, u64)") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("T::Hash") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructOwnedOrders :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" (AccoundId, Index) => OrderHash"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("OwnedOrdersIndex") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Default , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("T::AccountId") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("u64") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructOwnedOrdersIndex :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& ["\tAccountId => Index"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("OrderOwnedTrades") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Optional , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("(T::Hash, u64)") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("T::Hash") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructOrderOwnedTrades :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" (OrderHash, u64) => TradeHash"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("OrderOwnedTradesIndex") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Default , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("T::Hash") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("u64") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructOrderOwnedTradesIndex :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" (OrderHash, u64) => TradeHash"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("TradePairOwnedOrders") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Optional , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("(T::Hash, u64)") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("T::Hash") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructTradePairOwnedOrders :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" (TradePairHash, Index) => OrderHash"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("TradePairOwnedOrdersIndex") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Default , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("T::Hash") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("u64") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructTradePairOwnedOrdersIndex :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" TradePairHash => Index"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("LinkedItemList") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Optional , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("(T::Hash, Option<T::Price>)") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("OrderLinkedItem<T>") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructLinkedItemList :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" (TradePairHash, Price) => LinkedItem"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Trades") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Optional , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("T::Hash") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Trade<T>") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructTrades :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" TradeHash => Trade"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("OwnedTrades") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Optional , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("(T::AccountId, u64)") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("T::Hash") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructOwnedTrades :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" (AccountId, u64) => TradeHash"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("OwnedTradesIndex") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Default , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("T::AccountId") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("u64") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructOwnedTradesIndex :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" AccountId => u64"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("OwnedTPTrades") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Optional , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("(T::AccountId, T::Hash, u64)") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("T::Hash") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructOwnedTPTrades :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" (AccountId, TradePairHash, u64) => TradeHash"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("OwnedTPTradesIndex") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Default , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("(T::AccountId, T::Hash)") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("u64") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructOwnedTPTradesIndex :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" (AccountId, TradePairHash) => u64"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("OwnedTPOpenedOrders") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Optional , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("(T::AccountId, T::Hash)") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Vec<T::Hash>") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructOwnedTPOpenedOrders :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" (AccountId, TradePairHash) => Vec<OrderHash>"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("OwnedTPClosedOrders") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Optional , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("(T::AccountId, T::Hash)") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Vec<T::Hash>") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructOwnedTPClosedOrders :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" (AccountId, TradePairHash) => Vec<OrderHash>"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("TradePairOwnedTrades") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Optional , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("(T::Hash, u64)") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("T::Hash") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructTradePairOwnedTrades :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" (TradePairHash, u64) => TradeHash"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("TradePairOwnedTradesIndex") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Default , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("T::Hash") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("u64") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructTradePairOwnedTradesIndex :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" TradePairHash => u64"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("TPTradeDataBucket") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Default , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("(T::Hash, T::BlockNumber)") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("(T::Balance, Option<T::Price>, Option<T::Price>)") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructTPTradeDataBucket :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" (TradePairHash, BlockNumber) => (Sum_of_Trade_Volume, Highest_Price, Lowest_Price)"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("TPTradePriceBucket") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Default , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Map { hasher : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_128Concat , key : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("T::Hash") , value : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("(Vec<Option<T::Price>>, Vec<Option<T::Price>>)") , unused : false , } , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructTPTradePriceBucket :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& [" store the trade pair\'s H/L price within last day" , " TradePairHash => (Vec<Highest_Price>, Vec<Lowest_Price>)"]) , } , self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryMetadata { name : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("Nonce") , modifier : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryModifier :: Default , ty : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageEntryType :: Plain (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ("u64")) , default : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter (& __GetByteStructNonce :: < T > (self :: sp_api_hidden_includes_decl_storage :: hidden_include :: sp_std :: marker :: PhantomData))) , documentation : self :: sp_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode (& []) , }] [..]) , }
    }
}
/// Hidden instance generated to be internally used when module is used without
/// instance.
#[doc(hidden)]
pub struct __InherentHiddenInstance;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for __InherentHiddenInstance {
    #[inline]
    fn clone(&self) -> __InherentHiddenInstance {
        match *self {
            __InherentHiddenInstance => __InherentHiddenInstance,
        }
    }
}
impl ::core::marker::StructuralEq for __InherentHiddenInstance {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for __InherentHiddenInstance {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
impl ::core::marker::StructuralPartialEq for __InherentHiddenInstance {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for __InherentHiddenInstance {
    #[inline]
    fn eq(&self, other: &__InherentHiddenInstance) -> bool {
        match *other {
            __InherentHiddenInstance => match *self {
                __InherentHiddenInstance => true,
            },
        }
    }
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Encode for __InherentHiddenInstance {
        fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {}
    }
    impl _parity_scale_codec::EncodeLike for __InherentHiddenInstance {}
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl _parity_scale_codec::Decode for __InherentHiddenInstance {
        fn decode<DecIn: _parity_scale_codec::Input>(
            input: &mut DecIn,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            Ok(__InherentHiddenInstance)
        }
    }
};
impl core::fmt::Debug for __InherentHiddenInstance {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("__InherentHiddenInstance").finish()
    }
}
impl self::sp_api_hidden_includes_decl_storage::hidden_include::traits::Instance
    for __InherentHiddenInstance
{
    const PREFIX: &'static str = "TradeModule";
}
///	TradePairHash => TradePair
struct TradePairs<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<
        TradePair<T>,
    > for TradePairs<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"TradePairs"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        T::Hash,
        TradePair<T>,
    > for TradePairs<T>
{
    type Query = Option<TradePair<T>>;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"TradePairs"
    }
    fn from_optional_value_to_query(v: Option<TradePair<T>>) -> Self::Query {
        v.or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<TradePair<T>> {
        v
    }
}
/// (BaseTokenHash, quoteTokenHash) => TradePairHash
struct TradePairsHashByBaseQuote<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<T::Hash>
    for TradePairsHashByBaseQuote<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"TradePairsHashByBaseQuote"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        (T::Hash, T::Hash),
        T::Hash,
    > for TradePairsHashByBaseQuote<T>
{
    type Query = Option<T::Hash>;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"TradePairsHashByBaseQuote"
    }
    fn from_optional_value_to_query(v: Option<T::Hash>) -> Self::Query {
        v.or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<T::Hash> {
        v
    }
}
/// Index => TradePairHash
struct TradePairsHashByIndex<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<T::Hash>
    for TradePairsHashByIndex<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"TradePairsHashByIndex"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        u64,
        T::Hash,
    > for TradePairsHashByIndex<T>
{
    type Query = Option<T::Hash>;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"TradePairsHashByIndex"
    }
    fn from_optional_value_to_query(v: Option<T::Hash>) -> Self::Query {
        v.or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<T::Hash> {
        v
    }
}
/// Index
struct TradePairsIndex(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<()>,
);
impl
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u64>
    for TradePairsIndex
{
    type Query = u64;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"TradePairsIndex"
    }
    fn from_optional_value_to_query(v: Option<u64>) -> Self::Query {
        v.unwrap_or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<u64> {
        Some(v)
    }
}
/// OrderHash => Order
struct Orders<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<
        LimitOrder<T>,
    > for Orders<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"Orders"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        T::Hash,
        LimitOrder<T>,
    > for Orders<T>
{
    type Query = Option<LimitOrder<T>>;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"Orders"
    }
    fn from_optional_value_to_query(v: Option<LimitOrder<T>>) -> Self::Query {
        v.or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<LimitOrder<T>> {
        v
    }
}
/// (AccoundId, Index) => OrderHash
struct OwnedOrders<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<T::Hash>
    for OwnedOrders<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OwnedOrders"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        (T::AccountId, u64),
        T::Hash,
    > for OwnedOrders<T>
{
    type Query = Option<T::Hash>;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OwnedOrders"
    }
    fn from_optional_value_to_query(v: Option<T::Hash>) -> Self::Query {
        v.or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<T::Hash> {
        v
    }
}
///	AccountId => Index
struct OwnedOrdersIndex<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<u64>
    for OwnedOrdersIndex<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OwnedOrdersIndex"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        T::AccountId,
        u64,
    > for OwnedOrdersIndex<T>
{
    type Query = u64;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OwnedOrdersIndex"
    }
    fn from_optional_value_to_query(v: Option<u64>) -> Self::Query {
        v.unwrap_or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<u64> {
        Some(v)
    }
}
/// (OrderHash, u64) => TradeHash
struct OrderOwnedTrades<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<T::Hash>
    for OrderOwnedTrades<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OrderOwnedTrades"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        (T::Hash, u64),
        T::Hash,
    > for OrderOwnedTrades<T>
{
    type Query = Option<T::Hash>;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OrderOwnedTrades"
    }
    fn from_optional_value_to_query(v: Option<T::Hash>) -> Self::Query {
        v.or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<T::Hash> {
        v
    }
}
/// (OrderHash, u64) => TradeHash
struct OrderOwnedTradesIndex<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<u64>
    for OrderOwnedTradesIndex<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OrderOwnedTradesIndex"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        T::Hash,
        u64,
    > for OrderOwnedTradesIndex<T>
{
    type Query = u64;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OrderOwnedTradesIndex"
    }
    fn from_optional_value_to_query(v: Option<u64>) -> Self::Query {
        v.unwrap_or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<u64> {
        Some(v)
    }
}
/// (TradePairHash, Index) => OrderHash
struct TradePairOwnedOrders<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<T::Hash>
    for TradePairOwnedOrders<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"TradePairOwnedOrders"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        (T::Hash, u64),
        T::Hash,
    > for TradePairOwnedOrders<T>
{
    type Query = Option<T::Hash>;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"TradePairOwnedOrders"
    }
    fn from_optional_value_to_query(v: Option<T::Hash>) -> Self::Query {
        v.or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<T::Hash> {
        v
    }
}
/// TradePairHash => Index
struct TradePairOwnedOrdersIndex<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<u64>
    for TradePairOwnedOrdersIndex<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"TradePairOwnedOrdersIndex"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        T::Hash,
        u64,
    > for TradePairOwnedOrdersIndex<T>
{
    type Query = u64;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"TradePairOwnedOrdersIndex"
    }
    fn from_optional_value_to_query(v: Option<u64>) -> Self::Query {
        v.unwrap_or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<u64> {
        Some(v)
    }
}
/// (TradePairHash, Price) => LinkedItem
struct LinkedItemList<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<
        OrderLinkedItem<T>,
    > for LinkedItemList<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"LinkedItemList"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        (T::Hash, Option<T::Price>),
        OrderLinkedItem<T>,
    > for LinkedItemList<T>
{
    type Query = Option<OrderLinkedItem<T>>;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"LinkedItemList"
    }
    fn from_optional_value_to_query(v: Option<OrderLinkedItem<T>>) -> Self::Query {
        v.or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<OrderLinkedItem<T>> {
        v
    }
}
/// TradeHash => Trade
struct Trades<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<Trade<T>>
    for Trades<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"Trades"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        T::Hash,
        Trade<T>,
    > for Trades<T>
{
    type Query = Option<Trade<T>>;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"Trades"
    }
    fn from_optional_value_to_query(v: Option<Trade<T>>) -> Self::Query {
        v.or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<Trade<T>> {
        v
    }
}
/// (AccountId, u64) => TradeHash
struct OwnedTrades<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<T::Hash>
    for OwnedTrades<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OwnedTrades"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        (T::AccountId, u64),
        T::Hash,
    > for OwnedTrades<T>
{
    type Query = Option<T::Hash>;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OwnedTrades"
    }
    fn from_optional_value_to_query(v: Option<T::Hash>) -> Self::Query {
        v.or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<T::Hash> {
        v
    }
}
/// AccountId => u64
struct OwnedTradesIndex<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<u64>
    for OwnedTradesIndex<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OwnedTradesIndex"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        T::AccountId,
        u64,
    > for OwnedTradesIndex<T>
{
    type Query = u64;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OwnedTradesIndex"
    }
    fn from_optional_value_to_query(v: Option<u64>) -> Self::Query {
        v.unwrap_or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<u64> {
        Some(v)
    }
}
/// (AccountId, TradePairHash, u64) => TradeHash
struct OwnedTPTrades<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<T::Hash>
    for OwnedTPTrades<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OwnedTPTrades"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        (T::AccountId, T::Hash, u64),
        T::Hash,
    > for OwnedTPTrades<T>
{
    type Query = Option<T::Hash>;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OwnedTPTrades"
    }
    fn from_optional_value_to_query(v: Option<T::Hash>) -> Self::Query {
        v.or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<T::Hash> {
        v
    }
}
/// (AccountId, TradePairHash) => u64
struct OwnedTPTradesIndex<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<u64>
    for OwnedTPTradesIndex<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OwnedTPTradesIndex"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        (T::AccountId, T::Hash),
        u64,
    > for OwnedTPTradesIndex<T>
{
    type Query = u64;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OwnedTPTradesIndex"
    }
    fn from_optional_value_to_query(v: Option<u64>) -> Self::Query {
        v.unwrap_or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<u64> {
        Some(v)
    }
}
/// (AccountId, TradePairHash) => Vec<OrderHash>
struct OwnedTPOpenedOrders<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<
        Vec<T::Hash>,
    > for OwnedTPOpenedOrders<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OwnedTPOpenedOrders"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        (T::AccountId, T::Hash),
        Vec<T::Hash>,
    > for OwnedTPOpenedOrders<T>
{
    type Query = Option<Vec<T::Hash>>;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OwnedTPOpenedOrders"
    }
    fn from_optional_value_to_query(v: Option<Vec<T::Hash>>) -> Self::Query {
        v.or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<Vec<T::Hash>> {
        v
    }
}
/// (AccountId, TradePairHash) => Vec<OrderHash>
struct OwnedTPClosedOrders<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<
        Vec<T::Hash>,
    > for OwnedTPClosedOrders<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OwnedTPClosedOrders"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        (T::AccountId, T::Hash),
        Vec<T::Hash>,
    > for OwnedTPClosedOrders<T>
{
    type Query = Option<Vec<T::Hash>>;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"OwnedTPClosedOrders"
    }
    fn from_optional_value_to_query(v: Option<Vec<T::Hash>>) -> Self::Query {
        v.or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<Vec<T::Hash>> {
        v
    }
}
/// (TradePairHash, u64) => TradeHash
struct TradePairOwnedTrades<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<T::Hash>
    for TradePairOwnedTrades<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"TradePairOwnedTrades"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        (T::Hash, u64),
        T::Hash,
    > for TradePairOwnedTrades<T>
{
    type Query = Option<T::Hash>;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"TradePairOwnedTrades"
    }
    fn from_optional_value_to_query(v: Option<T::Hash>) -> Self::Query {
        v.or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<T::Hash> {
        v
    }
}
/// TradePairHash => u64
struct TradePairOwnedTradesIndex<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<u64>
    for TradePairOwnedTradesIndex<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"TradePairOwnedTradesIndex"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        T::Hash,
        u64,
    > for TradePairOwnedTradesIndex<T>
{
    type Query = u64;
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"TradePairOwnedTradesIndex"
    }
    fn from_optional_value_to_query(v: Option<u64>) -> Self::Query {
        v.unwrap_or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<u64> {
        Some(v)
    }
}
/// (TradePairHash, BlockNumber) => (Sum_of_Trade_Volume, Highest_Price, Lowest_Price)
struct TPTradeDataBucket<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<(
        T::Balance,
        Option<T::Price>,
        Option<T::Price>,
    )> for TPTradeDataBucket<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"TPTradeDataBucket"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        (T::Hash, T::BlockNumber),
        (T::Balance, Option<T::Price>, Option<T::Price>),
    > for TPTradeDataBucket<T>
{
    type Query = (T::Balance, Option<T::Price>, Option<T::Price>);
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"TPTradeDataBucket"
    }
    fn from_optional_value_to_query(
        v: Option<(T::Balance, Option<T::Price>, Option<T::Price>)>,
    ) -> Self::Query {
        v.unwrap_or_else(|| Default::default())
    }
    fn from_query_to_optional_value(
        v: Self::Query,
    ) -> Option<(T::Balance, Option<T::Price>, Option<T::Price>)> {
        Some(v)
    }
}
/// store the trade pair's H/L price within last day
/// TradePairHash => (Vec<Highest_Price>, Vec<Lowest_Price>)
struct TPTradePriceBucket<T: Trait>(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<(T,)>,
);
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::StoragePrefixedMap<(
        Vec<Option<T::Price>>,
        Vec<Option<T::Price>>,
    )> for TPTradePriceBucket<T>
{
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"TPTradePriceBucket"
    }
}
impl<T: Trait>
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageMap<
        T::Hash,
        (Vec<Option<T::Price>>, Vec<Option<T::Price>>),
    > for TPTradePriceBucket<T>
{
    type Query = (Vec<Option<T::Price>>, Vec<Option<T::Price>>);
    type Hasher = self::sp_api_hidden_includes_decl_storage::hidden_include::Blake2_128Concat;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"TPTradePriceBucket"
    }
    fn from_optional_value_to_query(
        v: Option<(Vec<Option<T::Price>>, Vec<Option<T::Price>>)>,
    ) -> Self::Query {
        v.unwrap_or_else(|| Default::default())
    }
    fn from_query_to_optional_value(
        v: Self::Query,
    ) -> Option<(Vec<Option<T::Price>>, Vec<Option<T::Price>>)> {
        Some(v)
    }
}
struct Nonce(
    self::sp_api_hidden_includes_decl_storage::hidden_include::sp_std::marker::PhantomData<()>,
);
impl
    self::sp_api_hidden_includes_decl_storage::hidden_include::storage::generator::StorageValue<u64>
    for Nonce
{
    type Query = u64;
    fn module_prefix() -> &'static [u8] {
        < __InherentHiddenInstance as self :: sp_api_hidden_includes_decl_storage :: hidden_include :: traits :: Instance > :: PREFIX . as_bytes ()
    }
    fn storage_prefix() -> &'static [u8] {
        b"Nonce"
    }
    fn from_optional_value_to_query(v: Option<u64>) -> Self::Query {
        v.unwrap_or_else(|| Default::default())
    }
    fn from_query_to_optional_value(v: Self::Query) -> Option<u64> {
        Some(v)
    }
}
/// [`RawEvent`] specialized for the configuration [`Trait`]
///
/// [`RawEvent`]: enum.RawEvent.html
/// [`Trait`]: trait.Trait.html
pub type Event<T> = RawEvent<
    <T as frame_system::Trait>::AccountId,
    <T as frame_system::Trait>::Hash,
    TradePair<T>,
    LimitOrder<T>,
    Trade<T>,
>;
/// Events for this module.
///
pub enum RawEvent<AccountId, Hash, TradePair, LimitOrder, Trade> {
    TradePairCreated(AccountId, Hash, TradePair),
    OrderCreated(AccountId, Hash, Hash, Hash, LimitOrder),
    TradeCreated(AccountId, Hash, Hash, Hash, Trade),
    OrderCanceled(AccountId, Hash),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<
        AccountId: ::core::clone::Clone,
        Hash: ::core::clone::Clone,
        TradePair: ::core::clone::Clone,
        LimitOrder: ::core::clone::Clone,
        Trade: ::core::clone::Clone,
    > ::core::clone::Clone for RawEvent<AccountId, Hash, TradePair, LimitOrder, Trade>
{
    #[inline]
    fn clone(&self) -> RawEvent<AccountId, Hash, TradePair, LimitOrder, Trade> {
        match (&*self,) {
            (&RawEvent::TradePairCreated(ref __self_0, ref __self_1, ref __self_2),) => {
                RawEvent::TradePairCreated(
                    ::core::clone::Clone::clone(&(*__self_0)),
                    ::core::clone::Clone::clone(&(*__self_1)),
                    ::core::clone::Clone::clone(&(*__self_2)),
                )
            }
            (&RawEvent::OrderCreated(
                ref __self_0,
                ref __self_1,
                ref __self_2,
                ref __self_3,
                ref __self_4,
            ),) => RawEvent::OrderCreated(
                ::core::clone::Clone::clone(&(*__self_0)),
                ::core::clone::Clone::clone(&(*__self_1)),
                ::core::clone::Clone::clone(&(*__self_2)),
                ::core::clone::Clone::clone(&(*__self_3)),
                ::core::clone::Clone::clone(&(*__self_4)),
            ),
            (&RawEvent::TradeCreated(
                ref __self_0,
                ref __self_1,
                ref __self_2,
                ref __self_3,
                ref __self_4,
            ),) => RawEvent::TradeCreated(
                ::core::clone::Clone::clone(&(*__self_0)),
                ::core::clone::Clone::clone(&(*__self_1)),
                ::core::clone::Clone::clone(&(*__self_2)),
                ::core::clone::Clone::clone(&(*__self_3)),
                ::core::clone::Clone::clone(&(*__self_4)),
            ),
            (&RawEvent::OrderCanceled(ref __self_0, ref __self_1),) => RawEvent::OrderCanceled(
                ::core::clone::Clone::clone(&(*__self_0)),
                ::core::clone::Clone::clone(&(*__self_1)),
            ),
        }
    }
}
impl<AccountId, Hash, TradePair, LimitOrder, Trade> ::core::marker::StructuralPartialEq
    for RawEvent<AccountId, Hash, TradePair, LimitOrder, Trade>
{
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<
        AccountId: ::core::cmp::PartialEq,
        Hash: ::core::cmp::PartialEq,
        TradePair: ::core::cmp::PartialEq,
        LimitOrder: ::core::cmp::PartialEq,
        Trade: ::core::cmp::PartialEq,
    > ::core::cmp::PartialEq for RawEvent<AccountId, Hash, TradePair, LimitOrder, Trade>
{
    #[inline]
    fn eq(&self, other: &RawEvent<AccountId, Hash, TradePair, LimitOrder, Trade>) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) };
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) };
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &RawEvent::TradePairCreated(ref __self_0, ref __self_1, ref __self_2),
                        &RawEvent::TradePairCreated(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                    ) => {
                        (*__self_0) == (*__arg_1_0)
                            && (*__self_1) == (*__arg_1_1)
                            && (*__self_2) == (*__arg_1_2)
                    }
                    (
                        &RawEvent::OrderCreated(
                            ref __self_0,
                            ref __self_1,
                            ref __self_2,
                            ref __self_3,
                            ref __self_4,
                        ),
                        &RawEvent::OrderCreated(
                            ref __arg_1_0,
                            ref __arg_1_1,
                            ref __arg_1_2,
                            ref __arg_1_3,
                            ref __arg_1_4,
                        ),
                    ) => {
                        (*__self_0) == (*__arg_1_0)
                            && (*__self_1) == (*__arg_1_1)
                            && (*__self_2) == (*__arg_1_2)
                            && (*__self_3) == (*__arg_1_3)
                            && (*__self_4) == (*__arg_1_4)
                    }
                    (
                        &RawEvent::TradeCreated(
                            ref __self_0,
                            ref __self_1,
                            ref __self_2,
                            ref __self_3,
                            ref __self_4,
                        ),
                        &RawEvent::TradeCreated(
                            ref __arg_1_0,
                            ref __arg_1_1,
                            ref __arg_1_2,
                            ref __arg_1_3,
                            ref __arg_1_4,
                        ),
                    ) => {
                        (*__self_0) == (*__arg_1_0)
                            && (*__self_1) == (*__arg_1_1)
                            && (*__self_2) == (*__arg_1_2)
                            && (*__self_3) == (*__arg_1_3)
                            && (*__self_4) == (*__arg_1_4)
                    }
                    (
                        &RawEvent::OrderCanceled(ref __self_0, ref __self_1),
                        &RawEvent::OrderCanceled(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &RawEvent<AccountId, Hash, TradePair, LimitOrder, Trade>) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) };
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) };
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (
                        &RawEvent::TradePairCreated(ref __self_0, ref __self_1, ref __self_2),
                        &RawEvent::TradePairCreated(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                    ) => {
                        (*__self_0) != (*__arg_1_0)
                            || (*__self_1) != (*__arg_1_1)
                            || (*__self_2) != (*__arg_1_2)
                    }
                    (
                        &RawEvent::OrderCreated(
                            ref __self_0,
                            ref __self_1,
                            ref __self_2,
                            ref __self_3,
                            ref __self_4,
                        ),
                        &RawEvent::OrderCreated(
                            ref __arg_1_0,
                            ref __arg_1_1,
                            ref __arg_1_2,
                            ref __arg_1_3,
                            ref __arg_1_4,
                        ),
                    ) => {
                        (*__self_0) != (*__arg_1_0)
                            || (*__self_1) != (*__arg_1_1)
                            || (*__self_2) != (*__arg_1_2)
                            || (*__self_3) != (*__arg_1_3)
                            || (*__self_4) != (*__arg_1_4)
                    }
                    (
                        &RawEvent::TradeCreated(
                            ref __self_0,
                            ref __self_1,
                            ref __self_2,
                            ref __self_3,
                            ref __self_4,
                        ),
                        &RawEvent::TradeCreated(
                            ref __arg_1_0,
                            ref __arg_1_1,
                            ref __arg_1_2,
                            ref __arg_1_3,
                            ref __arg_1_4,
                        ),
                    ) => {
                        (*__self_0) != (*__arg_1_0)
                            || (*__self_1) != (*__arg_1_1)
                            || (*__self_2) != (*__arg_1_2)
                            || (*__self_3) != (*__arg_1_3)
                            || (*__self_4) != (*__arg_1_4)
                    }
                    (
                        &RawEvent::OrderCanceled(ref __self_0, ref __self_1),
                        &RawEvent::OrderCanceled(ref __arg_1_0, ref __arg_1_1),
                    ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
impl<AccountId, Hash, TradePair, LimitOrder, Trade> ::core::marker::StructuralEq
    for RawEvent<AccountId, Hash, TradePair, LimitOrder, Trade>
{
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<
        AccountId: ::core::cmp::Eq,
        Hash: ::core::cmp::Eq,
        TradePair: ::core::cmp::Eq,
        LimitOrder: ::core::cmp::Eq,
        Trade: ::core::cmp::Eq,
    > ::core::cmp::Eq for RawEvent<AccountId, Hash, TradePair, LimitOrder, Trade>
{
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<Hash>;
            let _: ::core::cmp::AssertParamIsEq<TradePair>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<Hash>;
            let _: ::core::cmp::AssertParamIsEq<Hash>;
            let _: ::core::cmp::AssertParamIsEq<Hash>;
            let _: ::core::cmp::AssertParamIsEq<LimitOrder>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<Hash>;
            let _: ::core::cmp::AssertParamIsEq<Hash>;
            let _: ::core::cmp::AssertParamIsEq<Hash>;
            let _: ::core::cmp::AssertParamIsEq<Trade>;
            let _: ::core::cmp::AssertParamIsEq<AccountId>;
            let _: ::core::cmp::AssertParamIsEq<Hash>;
        }
    }
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<AccountId, Hash, TradePair, LimitOrder, Trade> _parity_scale_codec::Encode
        for RawEvent<AccountId, Hash, TradePair, LimitOrder, Trade>
    where
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        TradePair: _parity_scale_codec::Encode,
        TradePair: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        LimitOrder: _parity_scale_codec::Encode,
        LimitOrder: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Trade: _parity_scale_codec::Encode,
        Trade: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
    {
        fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
            match *self {
                RawEvent::TradePairCreated(ref aa, ref ba, ref ca) => {
                    dest.push_byte(0usize as u8);
                    dest.push(aa);
                    dest.push(ba);
                    dest.push(ca);
                }
                RawEvent::OrderCreated(ref aa, ref ba, ref ca, ref da, ref ea) => {
                    dest.push_byte(1usize as u8);
                    dest.push(aa);
                    dest.push(ba);
                    dest.push(ca);
                    dest.push(da);
                    dest.push(ea);
                }
                RawEvent::TradeCreated(ref aa, ref ba, ref ca, ref da, ref ea) => {
                    dest.push_byte(2usize as u8);
                    dest.push(aa);
                    dest.push(ba);
                    dest.push(ca);
                    dest.push(da);
                    dest.push(ea);
                }
                RawEvent::OrderCanceled(ref aa, ref ba) => {
                    dest.push_byte(3usize as u8);
                    dest.push(aa);
                    dest.push(ba);
                }
                _ => (),
            }
        }
    }
    impl<AccountId, Hash, TradePair, LimitOrder, Trade> _parity_scale_codec::EncodeLike
        for RawEvent<AccountId, Hash, TradePair, LimitOrder, Trade>
    where
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        TradePair: _parity_scale_codec::Encode,
        TradePair: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        LimitOrder: _parity_scale_codec::Encode,
        LimitOrder: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Trade: _parity_scale_codec::Encode,
        Trade: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        AccountId: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
        Hash: _parity_scale_codec::Encode,
    {
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<AccountId, Hash, TradePair, LimitOrder, Trade> _parity_scale_codec::Decode
        for RawEvent<AccountId, Hash, TradePair, LimitOrder, Trade>
    where
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        Hash: _parity_scale_codec::Decode,
        Hash: _parity_scale_codec::Decode,
        TradePair: _parity_scale_codec::Decode,
        TradePair: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        Hash: _parity_scale_codec::Decode,
        Hash: _parity_scale_codec::Decode,
        Hash: _parity_scale_codec::Decode,
        Hash: _parity_scale_codec::Decode,
        Hash: _parity_scale_codec::Decode,
        Hash: _parity_scale_codec::Decode,
        LimitOrder: _parity_scale_codec::Decode,
        LimitOrder: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        Hash: _parity_scale_codec::Decode,
        Hash: _parity_scale_codec::Decode,
        Hash: _parity_scale_codec::Decode,
        Hash: _parity_scale_codec::Decode,
        Hash: _parity_scale_codec::Decode,
        Hash: _parity_scale_codec::Decode,
        Trade: _parity_scale_codec::Decode,
        Trade: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        AccountId: _parity_scale_codec::Decode,
        Hash: _parity_scale_codec::Decode,
        Hash: _parity_scale_codec::Decode,
    {
        fn decode<DecIn: _parity_scale_codec::Input>(
            input: &mut DecIn,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            match input.read_byte()? {
                x if x == 0usize as u8 => Ok(RawEvent::TradePairCreated(
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err(
                                    "Error decoding field RawEvent :: TradePairCreated.0".into()
                                )
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err(
                                    "Error decoding field RawEvent :: TradePairCreated.1".into()
                                )
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err(
                                    "Error decoding field RawEvent :: TradePairCreated.2".into()
                                )
                            }
                            Ok(a) => a,
                        }
                    },
                )),
                x if x == 1usize as u8 => Ok(RawEvent::OrderCreated(
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: OrderCreated.0".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: OrderCreated.1".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: OrderCreated.2".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: OrderCreated.3".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: OrderCreated.4".into())
                            }
                            Ok(a) => a,
                        }
                    },
                )),
                x if x == 2usize as u8 => Ok(RawEvent::TradeCreated(
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: TradeCreated.0".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: TradeCreated.1".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: TradeCreated.2".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: TradeCreated.3".into())
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err("Error decoding field RawEvent :: TradeCreated.4".into())
                            }
                            Ok(a) => a,
                        }
                    },
                )),
                x if x == 3usize as u8 => Ok(RawEvent::OrderCanceled(
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err(
                                    "Error decoding field RawEvent :: OrderCanceled.0".into()
                                )
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err(
                                    "Error decoding field RawEvent :: OrderCanceled.1".into()
                                )
                            }
                            Ok(a) => a,
                        }
                    },
                )),
                x => Err("No such variant in enum RawEvent".into()),
            }
        }
    }
};
impl<AccountId, Hash, TradePair, LimitOrder, Trade> core::fmt::Debug
    for RawEvent<AccountId, Hash, TradePair, LimitOrder, Trade>
where
    AccountId: core::fmt::Debug,
    Hash: core::fmt::Debug,
    TradePair: core::fmt::Debug,
    LimitOrder: core::fmt::Debug,
    Trade: core::fmt::Debug,
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::TradePairCreated(ref a0, ref a1, ref a2) => fmt
                .debug_tuple("RawEvent::TradePairCreated")
                .field(a0)
                .field(a1)
                .field(a2)
                .finish(),
            Self::OrderCreated(ref a0, ref a1, ref a2, ref a3, ref a4) => fmt
                .debug_tuple("RawEvent::OrderCreated")
                .field(a0)
                .field(a1)
                .field(a2)
                .field(a3)
                .field(a4)
                .finish(),
            Self::TradeCreated(ref a0, ref a1, ref a2, ref a3, ref a4) => fmt
                .debug_tuple("RawEvent::TradeCreated")
                .field(a0)
                .field(a1)
                .field(a2)
                .field(a3)
                .field(a4)
                .finish(),
            Self::OrderCanceled(ref a0, ref a1) => fmt
                .debug_tuple("RawEvent::OrderCanceled")
                .field(a0)
                .field(a1)
                .finish(),
            _ => Ok(()),
        }
    }
}
impl<AccountId, Hash, TradePair, LimitOrder, Trade>
    From<RawEvent<AccountId, Hash, TradePair, LimitOrder, Trade>> for ()
{
    fn from(_: RawEvent<AccountId, Hash, TradePair, LimitOrder, Trade>) -> () {
        ()
    }
}
impl<AccountId, Hash, TradePair, LimitOrder, Trade>
    RawEvent<AccountId, Hash, TradePair, LimitOrder, Trade>
{
    #[allow(dead_code)]
    #[doc(hidden)]
    pub fn metadata() -> &'static [::frame_support::event::EventMetadata] {
        &[
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("TradePairCreated"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&[
                    "AccountId",
                    "Hash",
                    "TradePair",
                ]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("OrderCreated"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&[
                    "AccountId",
                    "Hash",
                    "Hash",
                    "Hash",
                    "LimitOrder",
                ]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("TradeCreated"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&[
                    "AccountId",
                    "Hash",
                    "Hash",
                    "Hash",
                    "Trade",
                ]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::event::EventMetadata {
                name: ::frame_support::event::DecodeDifferent::Encode("OrderCanceled"),
                arguments: ::frame_support::event::DecodeDifferent::Encode(&["AccountId", "Hash"]),
                documentation: ::frame_support::event::DecodeDifferent::Encode(&[]),
            },
        ]
    }
}
impl<T: Trait> OrderOwnedTrades<T> {
    fn add_trade(order_hash: T::Hash, trade_hash: T::Hash) {
        let index = OrderOwnedTradesIndex::<T>::get(&order_hash);
        Self::insert((order_hash.clone(), index), trade_hash);
        OrderOwnedTradesIndex::<T>::insert(order_hash, index + 1);
    }
}
impl<T: Trait> OwnedTrades<T> {
    fn add_trade(account_id: T::AccountId, trade_hash: T::Hash) {
        let index = OwnedTradesIndex::<T>::get(&account_id);
        Self::insert((account_id.clone(), index), trade_hash);
        OwnedTradesIndex::<T>::insert(account_id, index + 1);
    }
}
impl<T: Trait> TradePairOwnedTrades<T> {
    fn add_trade(tp_hash: T::Hash, trade_hash: T::Hash) {
        let index = TradePairOwnedTradesIndex::<T>::get(&tp_hash);
        Self::insert((tp_hash.clone(), index), trade_hash);
        TradePairOwnedTradesIndex::<T>::insert(tp_hash, index + 1);
    }
}
impl<T: Trait> OwnedTPTrades<T> {
    fn add_trade(account_id: T::AccountId, tp_hash: T::Hash, trade_hash: T::Hash) {
        let index = OwnedTPTradesIndex::<T>::get((account_id.clone(), tp_hash));
        Self::insert((account_id.clone(), tp_hash, index), trade_hash);
        OwnedTPTradesIndex::<T>::insert((account_id.clone(), tp_hash), index + 1);
    }
}
impl<T: Trait> OwnedTPOpenedOrders<T> {
    fn add_order(account_id: T::AccountId, tp_hash: T::Hash, order_hash: T::Hash) {
        let mut orders;
        if let Some(ts) = Self::get((account_id.clone(), tp_hash)) {
            orders = ts;
        } else {
            orders = Vec::<T::Hash>::new();
        }
        match orders.iter().position(|&x| x == order_hash) {
            Some(_) => return,
            None => {
                orders.insert(0, order_hash);
                if orders.len() == T::OpenedOrdersArrayCap::get() as usize {
                    orders.pop();
                }
                <OwnedTPOpenedOrders<T>>::insert((account_id, tp_hash), orders);
            }
        }
    }
    fn remove_order(account_id: T::AccountId, tp_hash: T::Hash, order_hash: T::Hash) {
        let mut orders;
        if let Some(ts) = Self::get((account_id.clone(), tp_hash)) {
            orders = ts;
        } else {
            orders = Vec::<T::Hash>::new();
        }
        orders.retain(|&x| x != order_hash);
        <OwnedTPOpenedOrders<T>>::insert((account_id, tp_hash), orders);
    }
}
impl<T: Trait> OwnedTPClosedOrders<T> {
    fn add_order(account_id: T::AccountId, tp_hash: T::Hash, order_hash: T::Hash) {
        let mut orders;
        if let Some(ts) = Self::get((account_id.clone(), tp_hash)) {
            orders = ts;
        } else {
            orders = Vec::<T::Hash>::new();
        }
        match orders.iter().position(|&x| x == order_hash) {
            Some(_) => return,
            None => {
                orders.insert(0, order_hash);
                if orders.len() == T::ClosedOrdersArrayCap::get() as usize {
                    orders.pop();
                }
                <OwnedTPClosedOrders<T>>::insert((account_id, tp_hash), orders);
            }
        }
    }
}
pub struct Module<T: Trait>(::frame_support::sp_std::marker::PhantomData<(T,)>);
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::clone::Clone + Trait> ::core::clone::Clone for Module<T> {
    #[inline]
    fn clone(&self) -> Module<T> {
        match *self {
            Module(ref __self_0_0) => Module(::core::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::marker::Copy + Trait> ::core::marker::Copy for Module<T> {}
impl<T: Trait> ::core::marker::StructuralPartialEq for Module<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::PartialEq + Trait> ::core::cmp::PartialEq for Module<T> {
    #[inline]
    fn eq(&self, other: &Module<T>) -> bool {
        match *other {
            Module(ref __self_1_0) => match *self {
                Module(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Module<T>) -> bool {
        match *other {
            Module(ref __self_1_0) => match *self {
                Module(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
impl<T: Trait> ::core::marker::StructuralEq for Module<T> {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl<T: ::core::cmp::Eq + Trait> ::core::cmp::Eq for Module<T> {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<
                ::frame_support::sp_std::marker::PhantomData<(T,)>,
            >;
        }
    }
}
impl<T: Trait> core::fmt::Debug for Module<T>
where
    T: core::fmt::Debug,
{
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        fmt.debug_tuple("Module").field(&self.0).finish()
    }
}
impl<T: frame_system::Trait + Trait>
    ::frame_support::traits::OnInitialize<<T as frame_system::Trait>::BlockNumber> for Module<T>
{
    fn on_initialize(block_number: T::BlockNumber) -> Weight {
        let __within_span__ = {
            if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
            {
                use ::tracing::__macro_support::*;
                let callsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_initialize",
                            "pallet_trade",
                            ::tracing::Level::TRACE,
                            Some("pallets/trade/src/lib.rs"),
                            Some(392u32),
                            Some("pallet_trade"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    static CALLSITE: MacroCallsite = MacroCallsite::new(&META);
                    CALLSITE.register();
                    &CALLSITE
                };
                if callsite.is_enabled() {
                    let meta = callsite.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    ::tracing::Span::none()
                }
            } else {
                ::tracing::Span::none()
            }
        };
        let __tracing_guard__ = __within_span__.enter();
        {
            let days: T::BlockNumber =
                <<T as frame_system::Trait>::BlockNumber as From<_>>::from(T::BlocksPerDay::get());
            if block_number <= days {
                return 1000;
            }
            for index in 0..TradePairsIndex::get() {
                let tp_hash = TradePairsHashByIndex::<T>::get(index).unwrap();
                let mut tp = TradePairs::<T>::get(tp_hash).unwrap();
                let (amount, _, _) = TPTradeDataBucket::<T>::get((tp_hash, block_number - days));
                tp.one_day_trade_volume = tp.one_day_trade_volume - amount;
                TradePairs::<T>::insert(tp_hash, tp);
                let mut bucket = TPTradePriceBucket::<T>::get(tp_hash);
                if bucket.0.len() > 0 {
                    bucket.0.remove(0);
                }
                if bucket.1.len() > 0 {
                    bucket.1.remove(0);
                }
                TPTradePriceBucket::<T>::insert(tp_hash, bucket);
            }
            500_000
        }
    }
}
impl<T: Trait> ::frame_support::traits::OnRuntimeUpgrade for Module<T> {}
impl<T: frame_system::Trait + Trait>
    ::frame_support::traits::OnFinalize<<T as frame_system::Trait>::BlockNumber> for Module<T>
{
    fn on_finalize(block_number: T::BlockNumber) {
        let __within_span__ = {
            if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
            {
                use ::tracing::__macro_support::*;
                let callsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_finalize",
                            "pallet_trade",
                            ::tracing::Level::TRACE,
                            Some("pallets/trade/src/lib.rs"),
                            Some(392u32),
                            Some("pallet_trade"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    static CALLSITE: MacroCallsite = MacroCallsite::new(&META);
                    CALLSITE.register();
                    &CALLSITE
                };
                if callsite.is_enabled() {
                    let meta = callsite.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    ::tracing::Span::none()
                }
            } else {
                ::tracing::Span::none()
            }
        };
        let __tracing_guard__ = __within_span__.enter();
        {
            for index in 0..TradePairsIndex::get() {
                let tp_hash = TradePairsHashByIndex::<T>::get(index).unwrap();
                let mut tp = TradePairs::<T>::get(tp_hash).unwrap();
                let data_bucket = TPTradeDataBucket::<T>::get((tp_hash, block_number));
                let mut price_bucket = TPTradePriceBucket::<T>::get(tp_hash);
                price_bucket.0.push(data_bucket.1);
                price_bucket.1.push(data_bucket.2);
                TPTradePriceBucket::<T>::insert(tp_hash, &price_bucket);
                let mut h_price = T::Price::min_value();
                for price in price_bucket.0.iter() {
                    if let &Some(price) = price {
                        if price > h_price {
                            h_price = price;
                        }
                    }
                }
                let mut l_price = T::Price::max_value();
                for price in price_bucket.1.iter() {
                    if let &Some(price) = price {
                        if price < l_price {
                            l_price = price;
                        }
                    }
                }
                tp.one_day_trade_volume = tp.one_day_trade_volume + data_bucket.0;
                if h_price != T::Price::min_value() {
                    tp.one_day_highest_price = Some(h_price);
                } else {
                    tp.one_day_highest_price = None;
                }
                if l_price != T::Price::max_value() {
                    tp.one_day_lowest_price = Some(l_price);
                } else {
                    tp.one_day_lowest_price = None;
                }
                TradePairs::<T>::insert(tp_hash, tp);
            }
        }
    }
}
impl<T: frame_system::Trait + Trait>
    ::frame_support::traits::OffchainWorker<<T as frame_system::Trait>::BlockNumber> for Module<T>
{
}
impl<T: Trait> Module<T> {
    /// Deposits an event using `frame_system::Module::deposit_event`.
    fn deposit_event(event: impl Into<<T as Trait>::Event>) {
        <frame_system::Module<T>>::deposit_event(event.into())
    }
}
#[cfg(feature = "std")]
impl<T: Trait> ::frame_support::traits::IntegrityTest for Module<T> {}
/// Can also be called using [`Call`].
///
/// [`Call`]: enum.Call.html
impl<T: Trait> Module<T> {
    ///
    /// NOTE: Calling this function will bypass origin filters.
    pub fn create_trade_pair(
        origin: T::Origin,
        base: T::Hash,
        quote: T::Hash,
    ) -> Result<(), dispatch::DispatchError> {
        let __within_span__ = {
            if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
            {
                use ::tracing::__macro_support::*;
                let callsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "create_trade_pair",
                            "pallet_trade",
                            ::tracing::Level::TRACE,
                            Some("pallets/trade/src/lib.rs"),
                            Some(392u32),
                            Some("pallet_trade"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    static CALLSITE: MacroCallsite = MacroCallsite::new(&META);
                    CALLSITE.register();
                    &CALLSITE
                };
                if callsite.is_enabled() {
                    let meta = callsite.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    ::tracing::Span::none()
                }
            } else {
                ::tracing::Span::none()
            }
        };
        let __tracing_guard__ = __within_span__.enter();
        let sender = ensure_signed(origin)?;
        Self::do_create_trade_pair(sender, base, quote)
    }
    ///
    /// NOTE: Calling this function will bypass origin filters.
    pub fn create_limit_order(
        origin: T::Origin,
        base: T::Hash,
        quote: T::Hash,
        otype: OrderType,
        price: T::Price,
        sell_amount: T::Balance,
    ) -> Result<(), dispatch::DispatchError> {
        let __within_span__ = {
            if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
            {
                use ::tracing::__macro_support::*;
                let callsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "create_limit_order",
                            "pallet_trade",
                            ::tracing::Level::TRACE,
                            Some("pallets/trade/src/lib.rs"),
                            Some(392u32),
                            Some("pallet_trade"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    static CALLSITE: MacroCallsite = MacroCallsite::new(&META);
                    CALLSITE.register();
                    &CALLSITE
                };
                if callsite.is_enabled() {
                    let meta = callsite.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    ::tracing::Span::none()
                }
            } else {
                ::tracing::Span::none()
            }
        };
        let __tracing_guard__ = __within_span__.enter();
        let sender = ensure_signed(origin)?;
        Self::do_create_limit_order(sender, base, quote, otype, price, sell_amount)
    }
    ///
    /// NOTE: Calling this function will bypass origin filters.
    pub fn create_limit_order_with_le_float(
        origin: T::Origin,
        base: T::Hash,
        quote: T::Hash,
        otype: OrderType,
        price: Vec<u8>,
        sell_amount: T::Balance,
    ) -> Result<(), dispatch::DispatchError> {
        let __within_span__ = {
            if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
            {
                use ::tracing::__macro_support::*;
                let callsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "create_limit_order_with_le_float",
                            "pallet_trade",
                            ::tracing::Level::TRACE,
                            Some("pallets/trade/src/lib.rs"),
                            Some(392u32),
                            Some("pallet_trade"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    static CALLSITE: MacroCallsite = MacroCallsite::new(&META);
                    CALLSITE.register();
                    &CALLSITE
                };
                if callsite.is_enabled() {
                    let meta = callsite.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    ::tracing::Span::none()
                }
            } else {
                ::tracing::Span::none()
            }
        };
        let __tracing_guard__ = __within_span__.enter();
        let sender = ensure_signed(origin)?;
        let price = Self::price_as_vec_u8_to_x_by_100m(price)?;
        Self::do_create_limit_order(sender, base, quote, otype, price, sell_amount)
    }
    ///
    /// NOTE: Calling this function will bypass origin filters.
    pub fn cancel_limit_order(
        origin: T::Origin,
        order_hash: T::Hash,
    ) -> Result<(), dispatch::DispatchError> {
        let __within_span__ = {
            if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && ::tracing::Level::TRACE <= ::tracing::level_filters::LevelFilter::current()
            {
                use ::tracing::__macro_support::*;
                let callsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "cancel_limit_order",
                            "pallet_trade",
                            ::tracing::Level::TRACE,
                            Some("pallets/trade/src/lib.rs"),
                            Some(392u32),
                            Some("pallet_trade"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    static CALLSITE: MacroCallsite = MacroCallsite::new(&META);
                    CALLSITE.register();
                    &CALLSITE
                };
                if callsite.is_enabled() {
                    let meta = callsite.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    ::tracing::Span::none()
                }
            } else {
                ::tracing::Span::none()
            }
        };
        let __tracing_guard__ = __within_span__.enter();
        let sender = ensure_signed(origin)?;
        Self::do_cancel_limit_order(sender, order_hash)
    }
}
/// Dispatchable calls.
///
/// Each variant of this enum maps to a dispatchable function from the associated module.
pub enum Call<T: Trait> {
    #[doc(hidden)]
    #[codec(skip)]
    __PhantomItem(
        ::frame_support::sp_std::marker::PhantomData<(T,)>,
        ::frame_support::Never,
    ),
    #[allow(non_camel_case_types)]
    create_trade_pair(T::Hash, T::Hash),
    #[allow(non_camel_case_types)]
    create_limit_order(T::Hash, T::Hash, OrderType, T::Price, T::Balance),
    #[allow(non_camel_case_types)]
    create_limit_order_with_le_float(T::Hash, T::Hash, OrderType, Vec<u8>, T::Balance),
    #[allow(non_camel_case_types)]
    cancel_limit_order(T::Hash),
}
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T: Trait> _parity_scale_codec::Encode for Call<T>
    where
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Price: _parity_scale_codec::Encode,
        T::Price: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
    {
        fn encode_to<EncOut: _parity_scale_codec::Output>(&self, dest: &mut EncOut) {
            match *self {
                Call::create_trade_pair(ref aa, ref ba) => {
                    dest.push_byte(0usize as u8);
                    dest.push(aa);
                    dest.push(ba);
                }
                Call::create_limit_order(ref aa, ref ba, ref ca, ref da, ref ea) => {
                    dest.push_byte(1usize as u8);
                    dest.push(aa);
                    dest.push(ba);
                    dest.push(ca);
                    dest.push(da);
                    dest.push(ea);
                }
                Call::create_limit_order_with_le_float(ref aa, ref ba, ref ca, ref da, ref ea) => {
                    dest.push_byte(2usize as u8);
                    dest.push(aa);
                    dest.push(ba);
                    dest.push(ca);
                    dest.push(da);
                    dest.push(ea);
                }
                Call::cancel_limit_order(ref aa) => {
                    dest.push_byte(3usize as u8);
                    dest.push(aa);
                }
                _ => (),
            }
        }
    }
    impl<T: Trait> _parity_scale_codec::EncodeLike for Call<T>
    where
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Price: _parity_scale_codec::Encode,
        T::Price: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Balance: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
        T::Hash: _parity_scale_codec::Encode,
    {
    }
};
const _: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate codec as _parity_scale_codec;
    impl<T: Trait> _parity_scale_codec::Decode for Call<T>
    where
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Price: _parity_scale_codec::Decode,
        T::Price: _parity_scale_codec::Decode,
        T::Balance: _parity_scale_codec::Decode,
        T::Balance: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Balance: _parity_scale_codec::Decode,
        T::Balance: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
        T::Hash: _parity_scale_codec::Decode,
    {
        fn decode<DecIn: _parity_scale_codec::Input>(
            input: &mut DecIn,
        ) -> core::result::Result<Self, _parity_scale_codec::Error> {
            match input.read_byte()? {
                x if x == 0usize as u8 => Ok(Call::create_trade_pair(
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err(
                                    "Error decoding field Call :: create_trade_pair.0".into()
                                )
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err(
                                    "Error decoding field Call :: create_trade_pair.1".into()
                                )
                            }
                            Ok(a) => a,
                        }
                    },
                )),
                x if x == 1usize as u8 => Ok(Call::create_limit_order(
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err(
                                    "Error decoding field Call :: create_limit_order.0".into()
                                )
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err(
                                    "Error decoding field Call :: create_limit_order.1".into()
                                )
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err(
                                    "Error decoding field Call :: create_limit_order.2".into()
                                )
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err(
                                    "Error decoding field Call :: create_limit_order.3".into()
                                )
                            }
                            Ok(a) => a,
                        }
                    },
                    {
                        let res = _parity_scale_codec::Decode::decode(input);
                        match res {
                            Err(_) => {
                                return Err(
                                    "Error decoding field Call :: create_limit_order.4".into()
                                )
                            }
                            Ok(a) => a,
                        }
                    },
                )),
                x if x == 2usize as u8 => {
                    Ok(Call::create_limit_order_with_le_float(
                        {
                            let res = _parity_scale_codec::Decode::decode(input);
                            match res { Err (_) => return Err ("Error decoding field Call :: create_limit_order_with_le_float.0" . into ()) , Ok (a) => a , }
                        },
                        {
                            let res = _parity_scale_codec::Decode::decode(input);
                            match res { Err (_) => return Err ("Error decoding field Call :: create_limit_order_with_le_float.1" . into ()) , Ok (a) => a , }
                        },
                        {
                            let res = _parity_scale_codec::Decode::decode(input);
                            match res { Err (_) => return Err ("Error decoding field Call :: create_limit_order_with_le_float.2" . into ()) , Ok (a) => a , }
                        },
                        {
                            let res = _parity_scale_codec::Decode::decode(input);
                            match res { Err (_) => return Err ("Error decoding field Call :: create_limit_order_with_le_float.3" . into ()) , Ok (a) => a , }
                        },
                        {
                            let res = _parity_scale_codec::Decode::decode(input);
                            match res { Err (_) => return Err ("Error decoding field Call :: create_limit_order_with_le_float.4" . into ()) , Ok (a) => a , }
                        },
                    ))
                }
                x if x == 3usize as u8 => Ok(Call::cancel_limit_order({
                    let res = _parity_scale_codec::Decode::decode(input);
                    match res {
                        Err(_) => {
                            return Err("Error decoding field Call :: cancel_limit_order.0".into())
                        }
                        Ok(a) => a,
                    }
                })),
                x => Err("No such variant in enum Call".into()),
            }
        }
    }
};
impl<T: Trait> ::frame_support::dispatch::GetDispatchInfo for Call<T> {
    fn get_dispatch_info(&self) -> ::frame_support::dispatch::DispatchInfo {
        match *self {
            Call::create_trade_pair(ref base, ref quote) => {
                let base_weight = 1_000_000;
                let weight =
                    <dyn ::frame_support::dispatch::WeighData<(&T::Hash, &T::Hash)>>::weigh_data(
                        &base_weight,
                        (base, quote),
                    );
                let class = < dyn :: frame_support :: dispatch :: ClassifyDispatch < (& T :: Hash , & T :: Hash) > > :: classify_dispatch (& base_weight , (base , quote)) ;
                let pays_fee =
                    <dyn ::frame_support::dispatch::PaysFee<(&T::Hash, &T::Hash)>>::pays_fee(
                        &base_weight,
                        (base, quote),
                    );
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::create_limit_order(
                ref base,
                ref quote,
                ref otype,
                ref price,
                ref sell_amount,
            ) => {
                let base_weight = 1_000_000;
                let weight = <dyn ::frame_support::dispatch::WeighData<(
                    &T::Hash,
                    &T::Hash,
                    &OrderType,
                    &T::Price,
                    &T::Balance,
                )>>::weigh_data(
                    &base_weight, (base, quote, otype, price, sell_amount)
                );
                let class = <dyn ::frame_support::dispatch::ClassifyDispatch<(
                    &T::Hash,
                    &T::Hash,
                    &OrderType,
                    &T::Price,
                    &T::Balance,
                )>>::classify_dispatch(
                    &base_weight, (base, quote, otype, price, sell_amount)
                );
                let pays_fee = <dyn ::frame_support::dispatch::PaysFee<(
                    &T::Hash,
                    &T::Hash,
                    &OrderType,
                    &T::Price,
                    &T::Balance,
                )>>::pays_fee(
                    &base_weight, (base, quote, otype, price, sell_amount)
                );
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::create_limit_order_with_le_float(
                ref base,
                ref quote,
                ref otype,
                ref price,
                ref sell_amount,
            ) => {
                let base_weight = 1_000_000;
                let weight = <dyn ::frame_support::dispatch::WeighData<(
                    &T::Hash,
                    &T::Hash,
                    &OrderType,
                    &Vec<u8>,
                    &T::Balance,
                )>>::weigh_data(
                    &base_weight, (base, quote, otype, price, sell_amount)
                );
                let class = <dyn ::frame_support::dispatch::ClassifyDispatch<(
                    &T::Hash,
                    &T::Hash,
                    &OrderType,
                    &Vec<u8>,
                    &T::Balance,
                )>>::classify_dispatch(
                    &base_weight, (base, quote, otype, price, sell_amount)
                );
                let pays_fee = <dyn ::frame_support::dispatch::PaysFee<(
                    &T::Hash,
                    &T::Hash,
                    &OrderType,
                    &Vec<u8>,
                    &T::Balance,
                )>>::pays_fee(
                    &base_weight, (base, quote, otype, price, sell_amount)
                );
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::cancel_limit_order(ref order_hash) => {
                let base_weight = 1_000_000;
                let weight = <dyn ::frame_support::dispatch::WeighData<(&T::Hash,)>>::weigh_data(
                    &base_weight,
                    (order_hash,),
                );
                let class = < dyn :: frame_support :: dispatch :: ClassifyDispatch < (& T :: Hash ,) > > :: classify_dispatch (& base_weight , (order_hash ,)) ;
                let pays_fee = <dyn ::frame_support::dispatch::PaysFee<(&T::Hash,)>>::pays_fee(
                    &base_weight,
                    (order_hash,),
                );
                ::frame_support::dispatch::DispatchInfo {
                    weight,
                    class,
                    pays_fee,
                }
            }
            Call::__PhantomItem(_, _) => {
                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::GetCallName for Call<T> {
    fn get_call_name(&self) -> &'static str {
        match *self {
            Call::create_trade_pair(ref base, ref quote) => {
                let _ = (base, quote);
                "create_trade_pair"
            }
            Call::create_limit_order(
                ref base,
                ref quote,
                ref otype,
                ref price,
                ref sell_amount,
            ) => {
                let _ = (base, quote, otype, price, sell_amount);
                "create_limit_order"
            }
            Call::create_limit_order_with_le_float(
                ref base,
                ref quote,
                ref otype,
                ref price,
                ref sell_amount,
            ) => {
                let _ = (base, quote, otype, price, sell_amount);
                "create_limit_order_with_le_float"
            }
            Call::cancel_limit_order(ref order_hash) => {
                let _ = (order_hash);
                "cancel_limit_order"
            }
            Call::__PhantomItem(_, _) => {
                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
    fn get_call_names() -> &'static [&'static str] {
        &[
            "create_trade_pair",
            "create_limit_order",
            "create_limit_order_with_le_float",
            "cancel_limit_order",
        ]
    }
}
impl<T: Trait> ::frame_support::dispatch::Clone for Call<T> {
    fn clone(&self) -> Self {
        match *self {
            Call::create_trade_pair(ref base, ref quote) => {
                Call::create_trade_pair((*base).clone(), (*quote).clone())
            }
            Call::create_limit_order(
                ref base,
                ref quote,
                ref otype,
                ref price,
                ref sell_amount,
            ) => Call::create_limit_order(
                (*base).clone(),
                (*quote).clone(),
                (*otype).clone(),
                (*price).clone(),
                (*sell_amount).clone(),
            ),
            Call::create_limit_order_with_le_float(
                ref base,
                ref quote,
                ref otype,
                ref price,
                ref sell_amount,
            ) => Call::create_limit_order_with_le_float(
                (*base).clone(),
                (*quote).clone(),
                (*otype).clone(),
                (*price).clone(),
                (*sell_amount).clone(),
            ),
            Call::cancel_limit_order(ref order_hash) => {
                Call::cancel_limit_order((*order_hash).clone())
            }
            _ => ::std::rt::begin_panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::PartialEq for Call<T> {
    fn eq(&self, _other: &Self) -> bool {
        match *self {
            Call::create_trade_pair(ref base, ref quote) => {
                let self_params = (base, quote);
                if let Call::create_trade_pair(ref base, ref quote) = *_other {
                    self_params == (base, quote)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::create_limit_order(
                ref base,
                ref quote,
                ref otype,
                ref price,
                ref sell_amount,
            ) => {
                let self_params = (base, quote, otype, price, sell_amount);
                if let Call::create_limit_order(
                    ref base,
                    ref quote,
                    ref otype,
                    ref price,
                    ref sell_amount,
                ) = *_other
                {
                    self_params == (base, quote, otype, price, sell_amount)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::create_limit_order_with_le_float(
                ref base,
                ref quote,
                ref otype,
                ref price,
                ref sell_amount,
            ) => {
                let self_params = (base, quote, otype, price, sell_amount);
                if let Call::create_limit_order_with_le_float(
                    ref base,
                    ref quote,
                    ref otype,
                    ref price,
                    ref sell_amount,
                ) = *_other
                {
                    self_params == (base, quote, otype, price, sell_amount)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            Call::cancel_limit_order(ref order_hash) => {
                let self_params = (order_hash,);
                if let Call::cancel_limit_order(ref order_hash) = *_other {
                    self_params == (order_hash,)
                } else {
                    match *_other {
                        Call::__PhantomItem(_, _) => {
                            ::std::rt::begin_panic("internal error: entered unreachable code")
                        }
                        _ => false,
                    }
                }
            }
            _ => ::std::rt::begin_panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::Eq for Call<T> {}
impl<T: Trait> ::frame_support::dispatch::fmt::Debug for Call<T> {
    fn fmt(
        &self,
        _f: &mut ::frame_support::dispatch::fmt::Formatter,
    ) -> ::frame_support::dispatch::result::Result<(), ::frame_support::dispatch::fmt::Error> {
        match *self {
            Call::create_trade_pair(ref base, ref quote) => {
                _f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (&"create_trade_pair", &(base.clone(), quote.clone())) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                        ],
                    },
                ))
            }
            Call::create_limit_order(
                ref base,
                ref quote,
                ref otype,
                ref price,
                ref sell_amount,
            ) => _f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (
                    &"create_limit_order",
                    &(
                        base.clone(),
                        quote.clone(),
                        otype.clone(),
                        price.clone(),
                        sell_amount.clone(),
                    ),
                ) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ],
                },
            )),
            Call::create_limit_order_with_le_float(
                ref base,
                ref quote,
                ref otype,
                ref price,
                ref sell_amount,
            ) => _f.write_fmt(::core::fmt::Arguments::new_v1(
                &["", ""],
                &match (
                    &"create_limit_order_with_le_float",
                    &(
                        base.clone(),
                        quote.clone(),
                        otype.clone(),
                        price.clone(),
                        sell_amount.clone(),
                    ),
                ) {
                    (arg0, arg1) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                    ],
                },
            )),
            Call::cancel_limit_order(ref order_hash) => {
                _f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (&"cancel_limit_order", &(order_hash.clone(),)) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                        ],
                    },
                ))
            }
            _ => ::std::rt::begin_panic("internal error: entered unreachable code"),
        }
    }
}
impl<T: Trait> ::frame_support::traits::UnfilteredDispatchable for Call<T> {
    type Origin = T::Origin;
    fn dispatch_bypass_filter(
        self,
        _origin: Self::Origin,
    ) -> ::frame_support::dispatch::DispatchResultWithPostInfo {
        match self {
            Call::create_trade_pair(base, quote) => {
                <Module<T>>::create_trade_pair(_origin, base, quote)
                    .map(Into::into)
                    .map_err(Into::into)
            }
            Call::create_limit_order(base, quote, otype, price, sell_amount) => {
                <Module<T>>::create_limit_order(_origin, base, quote, otype, price, sell_amount)
                    .map(Into::into)
                    .map_err(Into::into)
            }
            Call::create_limit_order_with_le_float(base, quote, otype, price, sell_amount) => {
                <Module<T>>::create_limit_order_with_le_float(
                    _origin,
                    base,
                    quote,
                    otype,
                    price,
                    sell_amount,
                )
                .map(Into::into)
                .map_err(Into::into)
            }
            Call::cancel_limit_order(order_hash) => {
                <Module<T>>::cancel_limit_order(_origin, order_hash)
                    .map(Into::into)
                    .map_err(Into::into)
            }
            Call::__PhantomItem(_, _) => {
                ::std::rt::begin_panic_fmt(&::core::fmt::Arguments::new_v1(
                    &["internal error: entered unreachable code: "],
                    &match (&"__PhantomItem should never be used.",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ))
            }
        }
    }
}
impl<T: Trait> ::frame_support::dispatch::Callable<T> for Module<T> {
    type Call = Call<T>;
}
impl<T: Trait> Module<T> {
    #[doc(hidden)]
    #[allow(dead_code)]
    pub fn call_functions() -> &'static [::frame_support::dispatch::FunctionMetadata] {
        &[
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("create_trade_pair"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("base"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("T::Hash"),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("quote"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("T::Hash"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("create_limit_order"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("base"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("T::Hash"),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("quote"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("T::Hash"),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("otype"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("OrderType"),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("price"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("T::Price"),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("sell_amount"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("T::Balance"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode(
                    "create_limit_order_with_le_float",
                ),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("base"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("T::Hash"),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("quote"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("T::Hash"),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("otype"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("OrderType"),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("price"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),
                    },
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("sell_amount"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("T::Balance"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
            },
            ::frame_support::dispatch::FunctionMetadata {
                name: ::frame_support::dispatch::DecodeDifferent::Encode("cancel_limit_order"),
                arguments: ::frame_support::dispatch::DecodeDifferent::Encode(&[
                    ::frame_support::dispatch::FunctionArgumentMetadata {
                        name: ::frame_support::dispatch::DecodeDifferent::Encode("order_hash"),
                        ty: ::frame_support::dispatch::DecodeDifferent::Encode("T::Hash"),
                    },
                ]),
                documentation: ::frame_support::dispatch::DecodeDifferent::Encode(&[]),
            },
        ]
    }
}
impl<T: 'static + Trait> Module<T> {
    #[doc(hidden)]
    #[allow(dead_code)]
    pub fn module_constants_metadata(
    ) -> &'static [::frame_support::dispatch::ModuleConstantMetadata] {
        &[]
    }
}
impl<T: Trait> ::frame_support::dispatch::ModuleErrorMetadata for Module<T> {
    fn metadata() -> &'static [::frame_support::dispatch::ErrorMetadata] {
        <Error<T> as ::frame_support::dispatch::ModuleErrorMetadata>::metadata()
    }
}
impl<T: Trait> Module<T> {
    fn ensure_bounds(price: T::Price, sell_amount: T::Balance) -> dispatch::DispatchResult {
        {
            if !(price > Zero::zero() && price <= T::Price::max_value()) {
                {
                    return Err(Error::<T>::BoundsCheckFailed.into());
                };
            }
        };
        {
            if !(sell_amount > Zero::zero() && sell_amount <= T::Balance::max_value()) {
                {
                    return Err(Error::<T>::BoundsCheckFailed.into());
                };
            }
        };
        Ok(())
    }
    fn price_as_vec_u8_to_x_by_100m(price: Vec<u8>) -> Result<T::Price, dispatch::DispatchError> {
        {
            if !(price.len() >= 8) {
                {
                    return Err(Error::<T>::PriceLengthCheckFailed.into());
                };
            }
        };
        let price = LittleEndian::read_f64(price.as_slice());
        let price_v2 = (T::PriceFactor::get() as f64 * price) as u128;
        let price_v3 = price_v2 as f64 / T::PriceFactor::get() as f64;
        {
            if !(price == price_v3) {
                {
                    return Err(Error::<T>::PriceLengthCheckFailed.into());
                };
            }
        };
        TryFrom::try_from(price_v2).map_err(|_| Error::<T>::NumberCastError.into())
    }
    fn ensure_counterparty_amount_bounds(
        otype: OrderType,
        price: T::Price,
        amount: T::Balance,
    ) -> result::Result<T::Balance, dispatch::DispatchError> {
        let price_u256 = U256::from(Self::into_128(price)?);
        let amount_u256 = U256::from(Self::into_128(amount)?);
        let max_balance_u256 = U256::from(Self::into_128(T::Balance::max_value())?);
        let price_factor_u256 = U256::from(T::PriceFactor::get());
        let amount_v2: U256;
        let counterparty_amount: U256;
        match otype {
            OrderType::Buy => {
                counterparty_amount = amount_u256 * price_factor_u256 / price_u256;
                amount_v2 = counterparty_amount * price_u256 / price_factor_u256;
            }
            OrderType::Sell => {
                counterparty_amount = amount_u256 * price_u256 / price_factor_u256;
                amount_v2 = counterparty_amount * price_factor_u256 / price_u256;
            }
        }
        {
            if !(amount_u256 == amount_v2) {
                {
                    return Err(Error::<T>::BoundsCheckFailed.into());
                };
            }
        };
        {
            if !(counterparty_amount != 0.into() && counterparty_amount <= max_balance_u256) {
                {
                    return Err(Error::<T>::BoundsCheckFailed.into());
                };
            }
        };
        let result: u128 = counterparty_amount
            .try_into()
            .map_err(|_| Error::<T>::OverflowError)?;
        Self::from_128(result)
    }
    fn ensure_trade_pair(
        base: T::Hash,
        quote: T::Hash,
    ) -> result::Result<T::Hash, dispatch::DispatchError> {
        let bq = Self::trade_pair_hash_by_base_quote((base, quote));
        {
            if !bq.is_some() {
                {
                    return Err(Error::<T>::NoMatchingTradePair.into());
                };
            }
        };
        match bq {
            Some(bq) => Ok(bq),
            None => Err(Error::<T>::NoMatchingTradePair.into()),
        }
    }
    fn do_create_trade_pair(
        sender: T::AccountId,
        base: T::Hash,
        quote: T::Hash,
    ) -> dispatch::DispatchResult {
        {
            if !(base != quote) {
                {
                    return Err(Error::<T>::BaseEqualQuote.into());
                };
            }
        };
        let base_owner = <token::Module<T>>::owner(base);
        let quote_owner = <token::Module<T>>::owner(quote);
        {
            if !(base_owner.is_some() && quote_owner.is_some()) {
                {
                    return Err(Error::<T>::TokenOwnerNotFound.into());
                };
            }
        };
        let base_owner = base_owner.unwrap();
        let quote_owner = quote_owner.unwrap();
        {
            if !(sender == base_owner || sender == quote_owner) {
                {
                    return Err(Error::<T>::SenderNotEqualToBaseOrQuoteOwner.into());
                };
            }
        };
        let bq = Self::trade_pair_hash_by_base_quote((base, quote));
        let qb = Self::trade_pair_hash_by_base_quote((quote, base));
        {
            if !(!bq.is_some() && !qb.is_some()) {
                {
                    return Err(Error::<T>::TradePairExisted.into());
                };
            }
        };
        let nonce = Nonce::get();
        let random_seed = <randomness_collective_flip::Module<T>>::random_seed();
        let hash = (
            random_seed,
            <frame_system::Module<T>>::block_number(),
            sender.clone(),
            base,
            quote,
            nonce,
        )
            .using_encoded(<T as frame_system::Trait>::hash);
        let tp = TradePair {
            hash,
            base,
            quote,
            latest_matched_price: None,
            one_day_trade_volume: Default::default(),
            one_day_highest_price: None,
            one_day_lowest_price: None,
        };
        Nonce::mutate(|n| *n += 1);
        TradePairs::insert(hash, tp.clone());
        TradePairsHashByBaseQuote::<T>::insert((base, quote), hash);
        let index = Self::trade_pair_index();
        TradePairsHashByIndex::<T>::insert(index, hash);
        TradePairsIndex::mutate(|n| *n += 1);
        Self::deposit_event(RawEvent::TradePairCreated(sender, hash, tp));
        Ok(())
    }
    fn do_create_limit_order(
        sender: T::AccountId,
        base: T::Hash,
        quote: T::Hash,
        otype: OrderType,
        price: T::Price,
        sell_amount: T::Balance,
    ) -> dispatch::DispatchResult {
        Self::ensure_bounds(price, sell_amount)?;
        let buy_amount = Self::ensure_counterparty_amount_bounds(otype, price, sell_amount)?;
        let tp_hash = Self::ensure_trade_pair(base, quote)?;
        let op_token_hash;
        match otype {
            OrderType::Buy => op_token_hash = base,
            OrderType::Sell => op_token_hash = quote,
        };
        let mut order = LimitOrder::new(
            base,
            quote,
            sender.clone(),
            price,
            sell_amount,
            buy_amount,
            otype,
        );
        let hash = order.hash;
        <token::Module<T>>::ensure_free_balance(sender.clone(), op_token_hash, sell_amount)?;
        <token::Module<T>>::do_freeze(sender.clone(), op_token_hash, sell_amount)?;
        Orders::insert(hash, order.clone());
        Nonce::mutate(|n| *n += 1);
        Self::deposit_event(RawEvent::OrderCreated(
            sender.clone(),
            base,
            quote,
            hash,
            order.clone(),
        ));
        <OwnedTPOpenedOrders<T>>::add_order(sender.clone(), tp_hash, order.hash);
        let owned_index = Self::owned_orders_index(sender.clone());
        OwnedOrders::<T>::insert((sender.clone(), owned_index), hash);
        OwnedOrdersIndex::<T>::insert(sender.clone(), owned_index + 1);
        let tp_owned_index = Self::trade_pair_owned_order_index(tp_hash);
        TradePairOwnedOrders::<T>::insert((tp_hash, tp_owned_index), hash);
        TradePairOwnedOrdersIndex::<T>::insert(tp_hash, tp_owned_index + 1);
        let filled = Self::order_match(tp_hash, &mut order)?;
        if !filled {
            <OrderLinkedItemList<T>>::append(
                tp_hash,
                price,
                hash,
                order.remained_sell_amount,
                order.remained_buy_amount,
                otype,
            );
        } else {
            <OwnedTPOpenedOrders<T>>::remove_order(sender.clone(), tp_hash, order.hash);
            <OwnedTPClosedOrders<T>>::add_order(sender.clone(), tp_hash, order.hash);
        }
        Ok(())
    }
    fn order_match(
        tp_hash: T::Hash,
        order: &mut LimitOrder<T>,
    ) -> result::Result<bool, dispatch::DispatchError> {
        let mut head = <OrderLinkedItemList<T>>::read_head(tp_hash);
        let end_item_price;
        let otype = order.otype;
        let oprice = order.price;
        if otype == OrderType::Buy {
            end_item_price = Some(T::Price::min_value());
        } else {
            end_item_price = Some(T::Price::max_value());
        }
        let tp = Self::trade_pair(tp_hash).ok_or(Error::<T>::NoMatchingTradePair)?;
        let give: T::Hash;
        let have: T::Hash;
        match otype {
            OrderType::Buy => {
                give = tp.base;
                have = tp.quote;
            }
            OrderType::Sell => {
                give = tp.quote;
                have = tp.base;
            }
        };
        loop {
            if order.status == OrderStatus::Filled {
                break;
            }
            let item_price = Self::next_match_price(&head, !otype);
            if item_price == end_item_price {
                break;
            }
            let item_price = item_price.ok_or(Error::<T>::OrderMatchGetPriceError)?;
            if !Self::price_matched(oprice, otype, item_price) {
                break;
            }
            let item = <LinkedItemList<T>>::get((tp_hash, Some(item_price)))
                .ok_or(Error::<T>::OrderMatchGetLinkedListItemError)?;
            for o in item.orders.iter() {
                let mut o = Self::order(o).ok_or(Error::<T>::OrderMatchGetOrderError)?;
                let (base_qty, quote_qty) = Self::calculate_ex_amount(&o, &order)?;
                let give_qty: T::Balance;
                let have_qty: T::Balance;
                match otype {
                    OrderType::Buy => {
                        give_qty = base_qty;
                        have_qty = quote_qty;
                    }
                    OrderType::Sell => {
                        give_qty = quote_qty;
                        have_qty = base_qty;
                    }
                };
                if order.remained_sell_amount == order.sell_amount {
                    order.status = OrderStatus::PartialFilled;
                }
                if o.remained_sell_amount == o.sell_amount {
                    o.status = OrderStatus::PartialFilled;
                }
                <token::Module<T>>::do_unfreeze(order.owner.clone(), give, give_qty)?;
                <token::Module<T>>::do_unfreeze(o.owner.clone(), have, have_qty)?;
                <token::Module<T>>::do_transfer(
                    order.owner.clone(),
                    give,
                    o.owner.clone(),
                    give_qty,
                    None,
                )?;
                <token::Module<T>>::do_transfer(
                    o.owner.clone(),
                    have,
                    order.owner.clone(),
                    have_qty,
                    None,
                )?;
                order.remained_sell_amount = order
                    .remained_sell_amount
                    .checked_sub(&give_qty)
                    .ok_or(Error::<T>::OrderMatchSubstractError)?;
                order.remained_buy_amount = order
                    .remained_buy_amount
                    .checked_sub(&have_qty)
                    .ok_or(Error::<T>::OrderMatchSubstractError)?;
                o.remained_sell_amount = o
                    .remained_sell_amount
                    .checked_sub(&have_qty)
                    .ok_or(Error::<T>::OrderMatchSubstractError)?;
                o.remained_buy_amount = o
                    .remained_buy_amount
                    .checked_sub(&give_qty)
                    .ok_or(Error::<T>::OrderMatchSubstractError)?;
                if order.remained_buy_amount == Zero::zero() {
                    order.status = OrderStatus::Filled;
                    if order.remained_sell_amount != Zero::zero() {
                        <token::Module<T>>::do_unfreeze(
                            order.owner.clone(),
                            give,
                            order.remained_sell_amount,
                        )?;
                        order.remained_sell_amount = Zero::zero();
                    }
                    <OwnedTPOpenedOrders<T>>::remove_order(
                        order.owner.clone(),
                        tp_hash,
                        order.hash,
                    );
                    <OwnedTPClosedOrders<T>>::add_order(order.owner.clone(), tp_hash, order.hash);
                    {
                        if !order.is_finished() {
                            {
                                return Err(Error::<T>::OrderMatchOrderIsNotFinished.into());
                            };
                        }
                    };
                }
                if o.remained_buy_amount == Zero::zero() {
                    o.status = OrderStatus::Filled;
                    if o.remained_sell_amount != Zero::zero() {
                        <token::Module<T>>::do_unfreeze(
                            o.owner.clone(),
                            have,
                            o.remained_sell_amount,
                        )?;
                        o.remained_sell_amount = Zero::zero();
                    }
                    <OwnedTPOpenedOrders<T>>::remove_order(o.owner.clone(), tp_hash, o.hash);
                    <OwnedTPClosedOrders<T>>::add_order(o.owner.clone(), tp_hash, o.hash);
                    {
                        if !o.is_finished() {
                            {
                                return Err(Error::<T>::OrderMatchOrderIsNotFinished.into());
                            };
                        }
                    };
                }
                Orders::insert(order.hash.clone(), order.clone());
                Orders::insert(o.hash.clone(), o.clone());
                Self::set_tp_market_data(tp_hash, o.price, quote_qty)?;
                <OrderLinkedItemList<T>>::update_amount(tp_hash, o.price, have_qty, give_qty);
                <OrderLinkedItemList<T>>::remove_all(tp_hash, !otype);
                let trade = Trade::new(tp.base, tp.quote, &o, &order, base_qty, quote_qty);
                Trades::insert(trade.hash, trade.clone());
                Self::deposit_event(RawEvent::TradeCreated(
                    order.owner.clone(),
                    tp.base,
                    tp.quote,
                    trade.hash,
                    trade.clone(),
                ));
                <OrderOwnedTrades<T>>::add_trade(order.hash, trade.hash);
                <OrderOwnedTrades<T>>::add_trade(o.hash, trade.hash);
                <OwnedTrades<T>>::add_trade(order.owner.clone(), trade.hash);
                <OwnedTrades<T>>::add_trade(o.owner.clone(), trade.hash);
                <OwnedTPTrades<T>>::add_trade(order.owner.clone(), tp_hash, trade.hash);
                <OwnedTPTrades<T>>::add_trade(o.owner.clone(), tp_hash, trade.hash);
                <TradePairOwnedTrades<T>>::add_trade(tp_hash, trade.hash);
                if order.status == OrderStatus::Filled {
                    break;
                }
            }
            head = <OrderLinkedItemList<T>>::read_head(tp_hash);
        }
        if order.status == OrderStatus::Filled {
            Ok(true)
        } else {
            Ok(false)
        }
    }
    fn into_128<A: TryInto<u128>>(i: A) -> Result<u128, dispatch::DispatchError> {
        TryInto::<u128>::try_into(i).map_err(|_| Error::<T>::NumberCastError.into())
    }
    fn from_128<A: TryFrom<u128>>(i: u128) -> Result<A, dispatch::DispatchError> {
        TryFrom::<u128>::try_from(i).map_err(|_| Error::<T>::NumberCastError.into())
    }
    fn calculate_ex_amount(
        maker_order: &LimitOrder<T>,
        taker_order: &LimitOrder<T>,
    ) -> result::Result<(T::Balance, T::Balance), dispatch::DispatchError> {
        let buyer_order;
        let seller_order;
        if taker_order.otype == OrderType::Buy {
            buyer_order = taker_order;
            seller_order = maker_order;
        } else {
            buyer_order = maker_order;
            seller_order = taker_order;
        }
        let mut seller_order_filled = true;
        if seller_order.remained_buy_amount <= buyer_order.remained_sell_amount {
            let quote_qty: u128 = Self::into_128(seller_order.remained_buy_amount)?
                * T::PriceFactor::get()
                / maker_order.price.into();
            if Self::into_128(buyer_order.remained_buy_amount)? < quote_qty {
                seller_order_filled = false;
            }
        } else {
            let base_qty: u128 = Self::into_128(buyer_order.remained_buy_amount)?
                * maker_order.price.into()
                / T::PriceFactor::get();
            if Self::into_128(seller_order.remained_buy_amount)? >= base_qty {
                seller_order_filled = false;
            }
        }
        if seller_order_filled {
            let mut quote_qty: u128 = Self::into_128(seller_order.remained_buy_amount)?
                * T::PriceFactor::get()
                / maker_order.price.into();
            let buy_amount_v2 =
                quote_qty * Self::into_128(maker_order.price)? / T::PriceFactor::get();
            if buy_amount_v2 != Self::into_128(seller_order.remained_buy_amount)?
                && Self::into_128(buyer_order.remained_buy_amount)? > quote_qty
            {
                quote_qty = quote_qty + 1;
            }
            return Ok((seller_order.remained_buy_amount, Self::from_128(quote_qty)?));
        } else {
            let mut base_qty: u128 = Self::into_128(buyer_order.remained_buy_amount)?
                * maker_order.price.into()
                / T::PriceFactor::get();
            let buy_amount_v2 = base_qty * T::PriceFactor::get() / maker_order.price.into();
            if buy_amount_v2 != Self::into_128(buyer_order.remained_buy_amount)?
                && Self::into_128(seller_order.remained_buy_amount)? > base_qty
            {
                base_qty = base_qty + 1;
            }
            return Ok((Self::from_128(base_qty)?, buyer_order.remained_buy_amount));
        }
    }
    fn next_match_price(item: &OrderLinkedItem<T>, otype: OrderType) -> Option<T::Price> {
        if otype == OrderType::Buy {
            item.prev
        } else {
            item.next
        }
    }
    fn price_matched(
        order_price: T::Price,
        order_type: OrderType,
        linked_item_price: T::Price,
    ) -> bool {
        match order_type {
            OrderType::Sell => order_price <= linked_item_price,
            OrderType::Buy => order_price >= linked_item_price,
        }
    }
    pub fn set_tp_market_data(
        tp_hash: T::Hash,
        price: T::Price,
        amount: T::Balance,
    ) -> dispatch::DispatchResult {
        let mut tp = <TradePairs<T>>::get(tp_hash).ok_or(Error::<T>::NoMatchingTradePair)?;
        tp.latest_matched_price = Some(price);
        let mut bucket =
            <TPTradeDataBucket<T>>::get((tp_hash, <frame_system::Module<T>>::block_number()));
        bucket.0 = bucket.0 + amount;
        match bucket.1 {
            Some(tp_h_price) => {
                if price > tp_h_price {
                    bucket.1 = Some(price);
                }
            }
            None => {
                bucket.1 = Some(price);
            }
        }
        match bucket.2 {
            Some(tp_l_price) => {
                if price < tp_l_price {
                    bucket.2 = Some(price);
                }
            }
            None => {
                bucket.2 = Some(price);
            }
        }
        <TPTradeDataBucket<T>>::insert(
            (tp_hash, <frame_system::Module<T>>::block_number()),
            bucket,
        );
        <TradePairs<T>>::insert(tp_hash, tp);
        Ok(())
    }
    fn do_cancel_limit_order(
        sender: T::AccountId,
        order_hash: T::Hash,
    ) -> dispatch::DispatchResult {
        let mut order = Self::order(order_hash).ok_or(Error::<T>::NoMatchingOrder)?;
        {
            if !(order.owner == sender) {
                {
                    return Err(Error::<T>::CanOnlyCancelOwnOrder.into());
                };
            }
        };
        {
            if !!order.is_finished() {
                {
                    return Err(Error::<T>::CanOnlyCancelNotFinishedOrder.into());
                };
            }
        };
        let tp_hash = Self::ensure_trade_pair(order.base, order.quote)?;
        <OrderLinkedItemList<T>>::remove_order(
            tp_hash,
            order.price,
            order.hash,
            order.sell_amount,
            order.buy_amount,
        )?;
        order.status = OrderStatus::Canceled;
        <Orders<T>>::insert(order_hash, order.clone());
        <OwnedTPOpenedOrders<T>>::remove_order(sender.clone(), tp_hash, order_hash);
        <OwnedTPClosedOrders<T>>::add_order(sender.clone(), tp_hash, order_hash);
        let sell_hash = match order.otype {
            OrderType::Buy => order.base,
            OrderType::Sell => order.quote,
        };
        <token::Module<T>>::do_unfreeze(sender.clone(), sell_hash, order.remained_sell_amount)?;
        Self::deposit_event(RawEvent::OrderCanceled(sender, order_hash));
        Ok(())
    }
    fn debug_log_market(tp_hash: T::Hash) {
        let mut item = <OrderLinkedItemList<T>>::read_bottom(tp_hash);
        {
            ::std::io::_eprint(::core::fmt::Arguments::new_v1(
                &["[Market Orders]\n"],
                &match () {
                    () => [],
                },
            ));
        };
        loop {
            if item.price == Some(T::Price::min_value()) {
                ::std::io::_eprint(::core::fmt::Arguments::new_v1(
                    &["Bottom ==> "],
                    &match () {
                        () => [],
                    },
                ));
            } else if item.price == Some(T::Price::max_value()) {
                ::std::io::_eprint(::core::fmt::Arguments::new_v1(
                    &["Top ==> "],
                    &match () {
                        () => [],
                    },
                ));
            } else if item.price == None {
                ::std::io::_eprint(::core::fmt::Arguments::new_v1(
                    &["Head ==> "],
                    &match () {
                        () => [],
                    },
                ));
            }
            ::std::io::_eprint(::core::fmt::Arguments::new_v1(
                &[
                    "Price(",
                    "), Next(",
                    "), Prev(",
                    "), Sell_Amount(",
                    "), Buy_Amount(",
                    "), Orders(",
                    "): ",
                ],
                &match (
                    &item.price,
                    &item.next,
                    &item.prev,
                    &item.sell_amount,
                    &item.buy_amount,
                    &item.orders.len(),
                ) {
                    (arg0, arg1, arg2, arg3, arg4, arg5) => [
                        ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt),
                        ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                        ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Debug::fmt),
                        ::core::fmt::ArgumentV1::new(arg3, ::core::fmt::Debug::fmt),
                        ::core::fmt::ArgumentV1::new(arg4, ::core::fmt::Debug::fmt),
                        ::core::fmt::ArgumentV1::new(arg5, ::core::fmt::Display::fmt),
                    ],
                },
            ));
            let mut orders = item.orders.iter();
            loop {
                match orders.next() {
                    Some(order_hash) => {
                        let order = <Orders<T>>::get(order_hash).unwrap();
                        ::std::io::_eprint(::core::fmt::Arguments::new_v1(
                            &["(", "@[", "]: Sell[", ", ", "], Buy[", ", ", "]), "],
                            &match (
                                &order.hash,
                                &order.status,
                                &order.sell_amount,
                                &order.remained_sell_amount,
                                &order.buy_amount,
                                &order.remained_buy_amount,
                            ) {
                                (arg0, arg1, arg2, arg3, arg4, arg5) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg3, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg4, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg5, ::core::fmt::Debug::fmt),
                                ],
                            },
                        ));
                    }
                    None => break,
                }
            }
            {
                ::std::io::_eprint(::core::fmt::Arguments::new_v1(
                    &["\n"],
                    &match () {
                        () => [],
                    },
                ));
            };
            if item.next == Some(T::Price::min_value()) {
                break;
            } else {
                item = OrderLinkedItemList::<T>::read(tp_hash, item.next);
            }
        }
        {
            ::std::io::_eprint(::core::fmt::Arguments::new_v1(
                &["[Market Trades]\n"],
                &match () {
                    () => [],
                },
            ));
        };
        let index_end = Self::trade_pair_owned_trades_index(tp_hash);
        for i in 0..index_end {
            let hash = Self::trade_pair_owned_trades((tp_hash, i));
            if let Some(hash) = hash {
                let trade = <Trades<T>>::get(hash).unwrap();
                {
                    ::std::io::_eprint(::core::fmt::Arguments::new_v1(
                        &[
                            "[",
                            "/",
                            "] - ",
                            "@",
                            "[",
                            "]: [Buyer,Seller][",
                            ",",
                            "], [Maker,Taker][",
                            ",",
                            "], [Base,Quote][",
                            ", ",
                            "]\n",
                        ],
                        &match (
                            &trade.quote,
                            &trade.base,
                            &hash,
                            &trade.price,
                            &trade.otype,
                            &trade.buyer,
                            &trade.seller,
                            &trade.maker,
                            &trade.taker,
                            &trade.base_amount,
                            &trade.quote_amount,
                        ) {
                            (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) => {
                                [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg2, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg3, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg4, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg5, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg6, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg7, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg8, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg9, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(arg10, ::core::fmt::Debug::fmt),
                                ]
                            }
                        },
                    ));
                };
            }
        }
        {
            ::std::io::_eprint(::core::fmt::Arguments::new_v1(
                &["[Trade Pair Data]\n"],
                &match () {
                    () => [],
                },
            ));
        };
        let tp = Self::trade_pair(tp_hash).unwrap();
        {
            ::std::io::_eprint(::core::fmt::Arguments::new_v1(
                &["latest matched price: ", "\n"],
                &match (&tp.latest_matched_price,) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Debug::fmt)],
                },
            ));
        };
        ::std::io::_eprint(::core::fmt::Arguments::new_v1(
            &["\n"],
            &match () {
                () => [],
            },
        ));
    }
}
