// SPDX-License-Identifier: MIT

use crate::{
    Nl80211Attr, Nl80211AttrsBuilder, Nl80211Handle, Nl80211InterfaceGetRequest,
};

use super::{Nl80211InterfaceSetRequest, Nl80211InterfaceType};

pub struct Nl80211InterfaceHandle(Nl80211Handle);

#[derive(Debug)]
pub struct Nl80211Interface;

impl Nl80211Interface {
    /// TODO: ....
    pub fn new(if_index: u32) -> Nl80211AttrsBuilder<Self> {
        Nl80211AttrsBuilder::<Self>::new().if_index(if_index)
    }
}

impl Nl80211AttrsBuilder<Nl80211Interface> {
    // TODO: better name? (type is a keyword)
    pub fn interface_type(self, r#type: Nl80211InterfaceType) -> Self {
        self.replace(Nl80211Attr::IfType(r#type))
    }
}

impl Nl80211InterfaceHandle {
    pub fn new(handle: Nl80211Handle) -> Self {
        Nl80211InterfaceHandle(handle)
    }

    /// Retrieve the wireless interfaces
    /// (equivalent to `iw dev`)
    pub fn get(&mut self) -> Nl80211InterfaceGetRequest {
        Nl80211InterfaceGetRequest::new(self.0.clone())
    }

    /// Retrieve the wireless interfaces
    /// (equivalent to `iw dev`)
    // TODO: naming (set_...?)
    // TODO: no restrictions on the type of attributes?
    pub fn interface_type(
        &mut self,
        attributes: Vec<Nl80211Attr>,
    ) -> Nl80211InterfaceSetRequest {
        Nl80211InterfaceSetRequest::new(self.0.clone(), attributes)
    }
}
